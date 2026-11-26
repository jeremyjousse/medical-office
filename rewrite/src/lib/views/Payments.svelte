<script lang="ts">
  import { untrack } from 'svelte';
  import {
    Plus,
    Eye,
    Pencil,
    Trash2,
    ArrowLeft,
    Search,
    RotateCcw,
    FileText,
  } from 'lucide-svelte';
  import {
    deletePayment,
    getPayment,
    listPayments,
    parseApiError,
    type PaginatedResult,
    type PaymentView,
  } from '../api';
  import Modal from '../components/Modal.svelte';
  import Pagination from '../components/Pagination.svelte';
  import PaymentForm from '../components/PaymentForm.svelte';
  import {
    formatEur,
    paymentCheckStatusLabel,
    paymentTypeLabel,
    paymentTypeOptions,
  } from '../i18n';
  import { appState, defaultPaymentFilters, PER_PAGE_OPTIONS } from '../state.svelte';

  interface Props {
    mode: 'index' | 'new' | 'show' | 'edit';
    id?: number;
    query?: Record<string, string>;
  }

  let { mode, id, query = {} }: Props = $props();

  let listing = $state<PaginatedResult<PaymentView> | null>(null);
  let detail = $state<PaymentView | null>(null);
  let loading = $state(false);
  let deleteTarget = $state<PaymentView | null>(null);
  let deleting = $state(false);

  let typeOptions = $derived(paymentTypeOptions(appState.locale));

  let initialTreatmentId = $derived.by(() => {
    if (query.medical_treatment_id) {
      const n = Number(query.medical_treatment_id);
      if (Number.isInteger(n) && n > 0) return n;
    }
    return null;
  });

  async function loadIndex(targetPage = appState.paymentFilters.page) {
    loading = true;
    try {
      const f = appState.paymentFilters;
      listing = await listPayments({
        page: targetPage,
        per_page: f.per_page,
        paid_at: f.paid_at || undefined,
        payment_type: f.payment_type,
      });
      appState.paymentFilters.page = listing.page;
    } catch (err) {
      appState.addFlash('error', parseApiError(err, appState.locale).message);
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    const activeUserId = appState.currentUser?.id;
    const currentMode = mode;
    const currentId = id;
    if (!appState.isInitialized || !activeUserId) return;

    if (currentMode === 'index') {
      untrack(() => {
        loadIndex(appState.paymentFilters.page);
      });
      return;
    }

    if (currentMode === 'show' && currentId) {
      let cancelled = false;
      loading = true;
      getPayment(currentId)
        .then((res) => {
          if (!cancelled) detail = res;
        })
        .catch((err) => {
          if (cancelled) return;
          appState.addFlash('error', parseApiError(err, appState.locale).message);
          appState.navigate('/payments');
        })
        .finally(() => {
          if (!cancelled) loading = false;
        });

      return () => {
        cancelled = true;
      };
    }
  });

  function handleFilterSubmit(e: SubmitEvent) {
    e.preventDefault();
    appState.paymentFilters.page = 1;
    loadIndex(1);
  }

  function handleResetFilters() {
    const perPage = appState.paymentFilters.per_page;
    appState.paymentFilters = { ...defaultPaymentFilters(), per_page: perPage };
    loadIndex(1);
  }

  function handlePerPageChange() {
    appState.paymentFilters.page = 1;
    loadIndex(1);
  }

  function formatTreatmentLabel(p: PaymentView): string {
    const name = [p.patient_last_name, p.patient_first_name].filter(Boolean).join(' ').trim();
    const tInfo = [p.treatment_type_name, p.treatment_date ? p.treatment_date.slice(0, 10) : '']
      .filter(Boolean)
      .join(' ');
    if (name && tInfo) return `${name} (${tInfo})`;
    if (name) return name;
    if (tInfo) return tInfo;
    return p.medical_treatment_id ? `#${p.medical_treatment_id}` : '—';
  }

  async function confirmDelete() {
    if (!deleteTarget) return;
    deleting = true;
    try {
      await deletePayment(deleteTarget.id);
      deleteTarget = null;
      appState.addFlash('success', appState.tr('paymentDeletedSuccess'));
      await loadIndex(appState.paymentFilters.page);
      if (listing && listing.items.length === 0 && appState.paymentFilters.page > 1) {
        appState.paymentFilters.page -= 1;
        await loadIndex(appState.paymentFilters.page);
      }
    } catch (err) {
      deleteTarget = null;
      appState.addFlash('error', parseApiError(err, appState.locale).message);
    } finally {
      deleting = false;
    }
  }
</script>

<div class="max-w-7xl mx-auto px-4 py-6">
  {#if mode === 'index'}
    <div class="card bg-base-100 border border-base-300 shadow-xs">
      <div class="card-body">
        <div class="flex flex-wrap items-center justify-between gap-3 border-b border-base-200 pb-3">
          <h1 class="text-xl font-bold">{appState.tr('paymentsTitle')}</h1>
          <div class="flex items-center gap-2">
            <button
              type="button"
              class="btn btn-outline btn-sm"
              onclick={() => appState.navigate('/payments/listing')}
            >
              <FileText class="w-4 h-4" />
              {appState.tr('paymentPrintListingBtn')}
            </button>
            <button
              type="button"
              class="btn btn-primary btn-sm"
              onclick={() => appState.navigate('/payments/new')}
            >
              <Plus class="w-4 h-4" />
              {appState.tr('paymentNewBtn')}
            </button>
          </div>
        </div>

        <div class="flex flex-wrap items-center justify-between gap-3 pt-1">
          <div class="flex items-center gap-2 text-sm">
            <span>{appState.tr('showCountPrefix')}</span>
            <select
              class="select select-bordered select-xs"
              aria-label={appState.tr('perPageSuffix')}
              bind:value={appState.paymentFilters.per_page}
              onchange={handlePerPageChange}
            >
              {#each PER_PAGE_OPTIONS as opt (opt.value)}
                <option value={opt.value}>
                  {'labelKey' in opt && opt.labelKey ? appState.tr(opt.labelKey) : opt.label}
                </option>
              {/each}
            </select>
            <span>{appState.tr('perPageSuffix')}</span>
          </div>
        </div>

        {#if listing}
          <Pagination
            page={listing.page}
            perPage={listing.per_page}
            filteredTotal={listing.filtered_total}
            total={listing.total}
            label={appState.tr('paymentPlural')}
            onPageChange={(nextPage) => {
              appState.paymentFilters.page = nextPage;
              loadIndex(nextPage);
            }}
          />
        {/if}

        <form onsubmit={handleFilterSubmit} class="overflow-x-auto">
          <table class="table table-zebra table-sm border border-base-200">
            <thead>
              <tr>
                <th></th>
                <th>{appState.tr('paymentMedicalTreatment')}</th>
                <th>{appState.tr('paymentPaidAt')}</th>
                <th>{appState.tr('paymentType')}</th>
                <th class="text-right">{appState.tr('paymentAmount')}</th>
                <th>{appState.tr('actions')}</th>
              </tr>
              <tr class="bg-base-200/60">
                <td></td>
                <td></td>
                <td>
                  <input
                    type="date"
                    class="input input-bordered input-xs w-full max-w-44"
                    aria-label={appState.tr('paymentPaidAt')}
                    bind:value={appState.paymentFilters.paid_at}
                  />
                </td>
                <td>
                  <select
                    class="select select-bordered select-xs w-full min-w-36"
                    aria-label={appState.tr('paymentType')}
                    bind:value={appState.paymentFilters.payment_type}
                  >
                    <option value={null}>{appState.tr('paymentAllTypes')}</option>
                    {#each typeOptions as opt (opt.value)}
                      <option value={opt.value}>{opt.label}</option>
                    {/each}
                  </select>
                </td>
                <td></td>
                <td class="whitespace-nowrap">
                  <div class="flex items-center gap-1">
                    <button type="submit" class="btn btn-xs btn-neutral">
                      <Search class="w-3.5 h-3.5" />
                      {appState.tr('filter')}
                    </button>
                    <button
                      type="button"
                      class="btn btn-xs btn-ghost"
                      onclick={handleResetFilters}
                    >
                      <RotateCcw class="w-3.5 h-3.5" />
                      {appState.tr('reset')}
                    </button>
                  </div>
                </td>
              </tr>
            </thead>
            <tbody>
              {#if loading && !listing}
                <tr>
                  <td colspan="6" class="text-center py-8">
                    <span class="loading loading-spinner loading-md text-primary"></span>
                  </td>
                </tr>
              {:else if listing && listing.items.length === 0}
                <tr>
                  <td colspan="6" class="text-center py-6 text-base-content/60">
                    {appState.tr('noRecords')}
                  </td>
                </tr>
              {:else if listing}
                {#each listing.items as item (item.id)}
                  <tr>
                    <td>
                      <button
                        type="button"
                        class="btn btn-xs btn-outline"
                        onclick={() => appState.navigate(`/payments/${item.id}`)}
                      >
                        <Eye class="w-3.5 h-3.5" />
                        {appState.tr('show')}
                      </button>
                    </td>
                    <td>
                      {#if item.medical_treatment_id}
                        <a
                          href={`#/medical-treatments/${item.medical_treatment_id}`}
                          class="link link-hover"
                          onclick={(e) => {
                            e.preventDefault();
                            appState.navigate(`/medical-treatments/${item.medical_treatment_id}`);
                          }}
                        >
                          {formatTreatmentLabel(item)}
                        </a>
                      {:else}
                        {formatTreatmentLabel(item)}
                      {/if}
                    </td>
                    <td>{item.paid_at ? item.paid_at.slice(0, 10) : ''}</td>
                    <td>{paymentTypeLabel(appState.locale, item.payment_type)}</td>
                    <td class="text-right font-mono">{formatEur(item.amount)}</td>
                    <td class="whitespace-nowrap">
                      <div class="flex items-center gap-1">
                        <button
                          type="button"
                          class="btn btn-xs btn-ghost"
                          onclick={() => appState.navigate(`/payments/${item.id}/edit`)}
                        >
                          <Pencil class="w-3.5 h-3.5" />
                          {appState.tr('edit')}
                        </button>
                        <button
                          type="button"
                          class="btn btn-xs btn-error btn-outline"
                          disabled={item.bank_deposit_id !== null}
                          onclick={() => {
                            if (item.bank_deposit_id === null) deleteTarget = item;
                          }}
                        >
                          <Trash2 class="w-3.5 h-3.5" />
                          {appState.tr('destroy')}
                        </button>
                      </div>
                    </td>
                  </tr>
                {/each}
              {/if}
            </tbody>
          </table>
        </form>
      </div>
    </div>
  {:else if mode === 'show'}
    <div class="max-w-3xl mx-auto">
      <div class="card bg-base-100 border border-base-300 shadow-xs">
        <div class="card-body">
          <div class="flex items-center justify-between border-b border-base-200 pb-3 mb-4">
            <h1 class="text-xl font-bold">{appState.tr('paymentDetailTitle')}</h1>
            <div class="flex items-center gap-2">
              {#if detail}
                <button
                  type="button"
                  class="btn btn-sm btn-outline"
                  onclick={() => appState.navigate(`/payments/${detail!.id}/edit`)}
                >
                  <Pencil class="w-4 h-4" />
                  {appState.tr('edit')}
                </button>
              {/if}
              <button
                type="button"
                class="btn btn-sm btn-ghost"
                onclick={() => appState.navigate('/payments')}
              >
                <ArrowLeft class="w-4 h-4" />
                {appState.tr('back')}
              </button>
            </div>
          </div>

          {#if loading}
            <div class="flex justify-center py-8">
              <span class="loading loading-spinner loading-md text-primary"></span>
            </div>
          {:else if detail}
            <div class="overflow-x-auto">
              <table class="table table-bordered">
                <tbody>
                  <tr>
                    <th class="w-52 bg-base-200/50">{appState.tr('paymentMedicalTreatment')}</th>
                    <td>
                      {#if detail.medical_treatment_id}
                        <a
                          href={`#/medical-treatments/${detail.medical_treatment_id}`}
                          class="link link-primary"
                          onclick={(e) => {
                            e.preventDefault();
                            appState.navigate(`/medical-treatments/${detail!.medical_treatment_id}`);
                          }}
                        >
                          {formatTreatmentLabel(detail)}
                        </a>
                      {:else}
                        {formatTreatmentLabel(detail)}
                      {/if}
                    </td>
                  </tr>
                  <tr>
                    <th class="bg-base-200/50">{appState.tr('paymentPaidAt')}</th>
                    <td>{detail.paid_at ? detail.paid_at.slice(0, 10) : ''}</td>
                  </tr>
                  <tr>
                    <th class="bg-base-200/50">{appState.tr('paymentType')}</th>
                    <td>{paymentTypeLabel(appState.locale, detail.payment_type)}</td>
                  </tr>
                  <tr>
                    <th class="bg-base-200/50">{appState.tr('paymentAmount')}</th>
                    <td class="font-mono">{formatEur(detail.amount)}</td>
                  </tr>
                  {#if detail.bank_deposit_id !== null}
                    <tr>
                      <th class="bg-base-200/50">{appState.tr('paymentBankDeposit')}</th>
                      <td>
                        <a
                          href={`#/bank-deposits/${detail.bank_deposit_id}`}
                          class="link link-primary"
                          onclick={(e) => {
                            e.preventDefault();
                            appState.navigate(`/bank-deposits/${detail!.bank_deposit_id}`);
                          }}
                        >
                          #{detail.bank_deposit_id}
                        </a>
                      </td>
                    </tr>
                  {/if}
                </tbody>
              </table>
            </div>

            {#if detail.payment_bank_check}
              {@const chk = detail.payment_bank_check}
              <div class="mt-6">
                <h2 class="text-base font-semibold mb-2">{appState.tr('paymentCheckSection')}</h2>
                <div class="overflow-x-auto">
                  <table class="table table-bordered">
                    <tbody>
                      <tr>
                        <th class="w-52 bg-base-200/50">{appState.tr('paymentCheckOwner')}</th>
                        <td>{chk.account_owner ?? '—'}</td>
                      </tr>
                      <tr>
                        <th class="bg-base-200/50">{appState.tr('paymentCheckBankName')}</th>
                        <td>{chk.bank_name ?? '—'}</td>
                      </tr>
                      <tr>
                        <th class="bg-base-200/50">{appState.tr('paymentCheckNumber')}</th>
                        <td>{chk.check_number ?? '—'}</td>
                      </tr>
                      <tr>
                        <th class="bg-base-200/50">{appState.tr('paymentCheckStatus')}</th>
                        <td>
                          <span
                            class={`badge badge-sm ${
                              chk.status === 1
                                ? 'badge-success'
                                : chk.status === -1
                                  ? 'badge-error'
                                  : 'badge-warning'
                            }`}
                          >
                            {paymentCheckStatusLabel(appState.locale, chk.status)}
                          </span>
                        </td>
                      </tr>
                    </tbody>
                  </table>
                </div>
              </div>
            {/if}
          {/if}
        </div>
      </div>
    </div>
  {:else}
    <div class="max-w-3xl mx-auto">
      <div class="card bg-base-100 border border-base-300 shadow-xs">
        <div class="card-body">
          <div class="flex items-center justify-between border-b border-base-200 pb-3 mb-4">
            <h1 class="text-xl font-bold">
              {mode === 'new' ? appState.tr('paymentNewTitle') : appState.tr('paymentEditTitle')}
            </h1>
            <button
              type="button"
              class="btn btn-sm btn-ghost"
              onclick={() =>
                appState.navigate(mode === 'edit' && id ? `/payments/${id}` : '/payments')}
            >
              <ArrowLeft class="w-4 h-4" />
              {appState.tr('back')}
            </button>
          </div>

          <PaymentForm
            paymentId={mode === 'edit' ? id : undefined}
            initialTreatmentId={mode === 'new' ? initialTreatmentId : null}
            cancelLabel={appState.tr('back')}
            onSaved={(saved) => {
              if (mode === 'new') {
                appState.paymentFilters.page = 1;
                appState.addFlash('success', appState.tr('paymentCreatedSuccess'));
              } else {
                appState.addFlash('success', appState.tr('paymentUpdatedSuccess'));
              }
              appState.navigate(`/payments/${saved.id}`);
            }}
            onCancel={() =>
              appState.navigate(mode === 'edit' && id ? `/payments/${id}` : '/payments')}
          />
        </div>
      </div>
    </div>
  {/if}
</div>

<Modal
  open={deleteTarget !== null}
  title={appState.tr('paymentDeleteConfirmTitle')}
  confirmLabel={appState.tr('destroy')}
  confirmVariant="error"
  loading={deleting}
  onConfirm={confirmDelete}
  onCancel={() => (deleteTarget = null)}
>
  <p>{appState.tr('paymentDeleteConfirmBody')}</p>
</Modal>
