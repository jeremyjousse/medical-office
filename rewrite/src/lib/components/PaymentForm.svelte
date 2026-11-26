<script lang="ts">
  import { Save } from 'lucide-svelte';
  import {
    createPayment,
    getPayment,
    listTreatmentOptionsForPayment,
    parseApiError,
    updatePayment,
    type MedicalTreatmentOption,
    type PaymentView,
  } from '../api';
  import { paymentTypeOptions } from '../i18n';
  import { appState } from '../state.svelte';
  import SearchSelect, { type SelectOption } from './SearchSelect.svelte';

  interface Props {
    paymentId?: number;
    fixedMedicalTreatmentId?: number;
    fixedTreatmentLabel?: string;
    initialTreatmentId?: number | null;
    defaultAmount?: number | null;
    defaultAccountOwner?: string | null;
    cancelLabel?: string;
    onSaved: (payment: PaymentView) => void;
    onCancel: () => void;
  }

  let {
    paymentId,
    fixedMedicalTreatmentId,
    fixedTreatmentLabel,
    initialTreatmentId = null,
    defaultAmount = null,
    defaultAccountOwner = null,
    cancelLabel,
    onSaved,
    onCancel,
  }: Props = $props();

  function todayLocalDate(): string {
    const d = new Date();
    const y = d.getFullYear();
    const m = String(d.getMonth() + 1).padStart(2, '0');
    const day = String(d.getDate()).padStart(2, '0');
    return `${y}-${m}-${day}`;
  }

  let loading = $state(false);
  let saving = $state(false);
  let fieldErrors = $state<Record<string, string>>({});
  let treatmentOptions = $state<MedicalTreatmentOption[]>([]);
  let isDeposited = $state(false);

  let medicalTreatmentId = $state<number | null>(null);
  let paidAt = $state(todayLocalDate());
  let paymentType = $state<number | null>(1);
  let amount = $state<number | null>(null);

  let accountOwner = $state('');
  let bankName = $state('');
  let checkNumber = $state('');

  let typeOptions = $derived(paymentTypeOptions(appState.locale));

  let selectOptions = $derived<SelectOption[]>(
    treatmentOptions.map((t) => {
      const name = [t.patient_last_name, t.patient_first_name].filter(Boolean).join(' ').trim();
      const priceStr = t.price !== null && t.price !== undefined ? t.price.toFixed(2) : '—';
      const detail = [t.treatment_type_name, t.date, priceStr].filter(Boolean).join(' ');
      return {
        value: t.id,
        label: name ? `${name} (${detail})` : `#${t.id} (${detail})`,
      };
    }),
  );

  function resolvedPatientLastName(): string {
    if (defaultAccountOwner) return defaultAccountOwner;
    const tid = fixedMedicalTreatmentId ?? medicalTreatmentId;
    if (!tid) return '';
    const found = treatmentOptions.find((t) => t.id === tid);
    return found?.patient_last_name ?? '';
  }

  $effect(() => {
    const activeUserId = appState.currentUser?.id;
    const currentPaymentId = paymentId;
    const lockedTreatmentId = fixedMedicalTreatmentId;
    const seedTreatmentId = initialTreatmentId;
    const seedAmount = defaultAmount;
    const seedOwner = defaultAccountOwner;
    if (!appState.isInitialized || !activeUserId) return;

    let cancelled = false;
    loading = true;
    fieldErrors = {};
    isDeposited = false;

    Promise.all([
      lockedTreatmentId ? Promise.resolve([]) : listTreatmentOptionsForPayment(),
      currentPaymentId ? getPayment(currentPaymentId) : Promise.resolve(null),
    ])
      .then(([options, existing]) => {
        if (cancelled) return;
        treatmentOptions = options;
        if (existing) {
          isDeposited = existing.bank_deposit_id !== null;
          medicalTreatmentId = lockedTreatmentId ?? existing.medical_treatment_id ?? null;
          paidAt = existing.paid_at ? existing.paid_at.slice(0, 10) : todayLocalDate();
          paymentType = existing.payment_type ?? 1;
          amount = existing.amount ?? null;
          if (existing.payment_bank_check) {
            accountOwner = existing.payment_bank_check.account_owner ?? '';
            bankName = existing.payment_bank_check.bank_name ?? '';
            checkNumber = existing.payment_bank_check.check_number ?? '';
          } else {
            accountOwner = existing.patient_last_name ?? seedOwner ?? '';
            bankName = '';
            checkNumber = '';
          }
        } else {
          const targetTid = lockedTreatmentId ?? seedTreatmentId ?? null;
          medicalTreatmentId = targetTid;
          paidAt = todayLocalDate();
          paymentType = 1;
          const matchedOption = targetTid ? options.find((t) => t.id === targetTid) : undefined;
          amount = seedAmount ?? matchedOption?.price ?? null;
          accountOwner = seedOwner ?? matchedOption?.patient_last_name ?? '';
          bankName = '';
          checkNumber = '';
        }
      })
      .catch((err) => {
        if (!cancelled) {
          appState.addFlash('error', parseApiError(err, appState.locale).message);
        }
      })
      .finally(() => {
        if (!cancelled) loading = false;
      });

    return () => {
      cancelled = true;
    };
  });

  function handleTreatmentChange(nextId: number | null) {
    medicalTreatmentId = nextId;
    if (!paymentId && nextId) {
      const opt = treatmentOptions.find((t) => t.id === nextId);
      if (opt) {
        if (opt.price !== null && opt.price !== undefined) {
          amount = opt.price;
        }
        if (!accountOwner.trim() && opt.patient_last_name) {
          accountOwner = opt.patient_last_name;
        }
      }
    }
  }

  function handlePaymentTypeChange() {
    if (paymentType === 1 && !accountOwner.trim()) {
      accountOwner = resolvedPatientLastName();
    }
  }

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    saving = true;
    fieldErrors = {};

    const parsedAmount = amount === null || (amount as unknown) === '' ? null : Number(amount);
    if (parsedAmount !== null && (!Number.isFinite(parsedAmount) || parsedAmount <= 0)) {
      fieldErrors = { amount: appState.tr('err_invalid_amount') };
      saving = false;
      return;
    }

    const payload = {
      medical_treatment_id: fixedMedicalTreatmentId ?? medicalTreatmentId,
      paid_at: paidAt,
      payment_type: paymentType,
      amount: parsedAmount,
      payment_bank_check:
        paymentType === 1
          ? {
              account_owner: accountOwner,
              bank_name: bankName,
              check_number: checkNumber,
            }
          : null,
    };

    try {
      const saved = paymentId
        ? await updatePayment(paymentId, payload)
        : await createPayment(payload);
      onSaved(saved);
    } catch (err) {
      const parsed = parseApiError(err, appState.locale);
      fieldErrors = parsed.fieldErrors;
      if (Object.keys(parsed.fieldErrors).length === 0) {
        appState.addFlash('error', parsed.message);
      }
    } finally {
      saving = false;
    }
  }
</script>

{#if loading}
  <div class="flex justify-center py-8">
    <span class="loading loading-spinner loading-md text-primary"></span>
  </div>
{:else}
  <form onsubmit={handleSubmit} class="space-y-4">
    {#if isDeposited}
      <div class="alert alert-warning text-sm py-2">
        <span>{appState.tr('paymentDepositedReadOnlyNotice')}</span>
      </div>
    {/if}

    {#if fixedMedicalTreatmentId}
      <div class="bg-base-200/60 px-3 py-2 rounded-lg text-sm">
        <span class="font-medium text-base-content/70">{appState.tr('paymentMedicalTreatment')}:</span>
        <span class="font-semibold ml-1">{fixedTreatmentLabel ?? `#${fixedMedicalTreatmentId}`}</span>
      </div>
    {:else}
      <div>
        <label class="label" for="pay-treatment">
          <span class="label-text font-medium">{appState.tr('paymentMedicalTreatment')} *</span>
        </label>
        <SearchSelect
          id="pay-treatment"
          options={selectOptions}
          value={medicalTreatmentId}
          disabled={isDeposited}
          error={fieldErrors.medical_treatment_id}
          onChange={handleTreatmentChange}
        />
        {#if fieldErrors.medical_treatment_id}
          <p class="text-error text-xs mt-1">{fieldErrors.medical_treatment_id}</p>
        {/if}
      </div>
    {/if}

    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <div>
        <label class="label" for="pay-type">
          <span class="label-text font-medium">{appState.tr('paymentType')} *</span>
        </label>
        <select
          id="pay-type"
          class={`select select-bordered w-full ${fieldErrors.payment_type ? 'select-error' : ''}`}
          disabled={isDeposited}
          bind:value={paymentType}
          onchange={handlePaymentTypeChange}
        >
          {#each typeOptions as opt (opt.value)}
            <option value={opt.value}>{opt.label}</option>
          {/each}
        </select>
        {#if fieldErrors.payment_type}
          <p class="text-error text-xs mt-1">{fieldErrors.payment_type}</p>
        {/if}
      </div>

      <div>
        <label class="label" for="pay-paid-at">
          <span class="label-text font-medium">{appState.tr('paymentPaidAt')} *</span>
        </label>
        <input
          id="pay-paid-at"
          type="date"
          class={`input input-bordered w-full ${fieldErrors.paid_at ? 'input-error' : ''}`}
          bind:value={paidAt}
        />
        {#if fieldErrors.paid_at}
          <p class="text-error text-xs mt-1">{fieldErrors.paid_at}</p>
        {/if}
      </div>

      <div>
        <label class="label" for="pay-amount">
          <span class="label-text font-medium">{appState.tr('paymentAmount')} *</span>
        </label>
        <input
          id="pay-amount"
          type="number"
          step="0.01"
          min="0.01"
          disabled={isDeposited}
          class={`input input-bordered w-full ${fieldErrors.amount ? 'input-error' : ''}`}
          bind:value={amount}
        />
        {#if fieldErrors.amount}
          <p class="text-error text-xs mt-1">{fieldErrors.amount}</p>
        {/if}
      </div>
    </div>

    {#if paymentType === 1}
      <div class="border border-base-300 rounded-lg p-4 bg-base-200/30 space-y-3">
        <div class="text-sm font-semibold">{appState.tr('paymentCheckSection')}</div>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div>
            <label class="label" for="chk-owner">
              <span class="label-text font-medium">{appState.tr('paymentCheckOwner')} *</span>
            </label>
            <input
              id="chk-owner"
              type="text"
              class={`input input-bordered w-full ${fieldErrors.account_owner ? 'input-error' : ''}`}
              bind:value={accountOwner}
            />
            {#if fieldErrors.account_owner}
              <p class="text-error text-xs mt-1">{fieldErrors.account_owner}</p>
            {/if}
          </div>

          <div>
            <label class="label" for="chk-bank">
              <span class="label-text font-medium">{appState.tr('paymentCheckBankName')} *</span>
            </label>
            <input
              id="chk-bank"
              type="text"
              class={`input input-bordered w-full ${fieldErrors.bank_name ? 'input-error' : ''}`}
              bind:value={bankName}
            />
            {#if fieldErrors.bank_name}
              <p class="text-error text-xs mt-1">{fieldErrors.bank_name}</p>
            {/if}
          </div>

          <div>
            <label class="label" for="chk-num">
              <span class="label-text font-medium">{appState.tr('paymentCheckNumber')} *</span>
            </label>
            <input
              id="chk-num"
              type="text"
              class={`input input-bordered w-full ${fieldErrors.check_number ? 'input-error' : ''}`}
              bind:value={checkNumber}
            />
            {#if fieldErrors.check_number}
              <p class="text-error text-xs mt-1">{fieldErrors.check_number}</p>
            {/if}
          </div>
        </div>
      </div>
    {/if}

    <div class="flex justify-end gap-2 pt-2">
      <button type="button" class="btn btn-ghost" disabled={saving} onclick={onCancel}>
        {cancelLabel ?? appState.tr('cancel')}
      </button>
      <button type="submit" class="btn btn-primary" disabled={saving}>
        {#if saving}
          <span class="loading loading-spinner loading-xs"></span>
          {appState.tr('saving')}
        {:else}
          <Save class="w-4 h-4" />
          {appState.tr('save')}
        {/if}
      </button>
    </div>
  </form>
{/if}
