<script lang="ts">
  import { untrack } from 'svelte';
  import {
    Plus,
    Eye,
    Trash2,
    ArrowLeft,
    Search,
    RotateCcw,
    Printer,
    Save,
  } from 'lucide-svelte';
  import {
    createBankDeposit,
    deleteBankDeposit,
    getBankDeposit,
    listBankAccountOptions,
    listBankDeposits,
    listPendingBankChecks,
    parseApiError,
    type BankAccount,
    type BankCheckDepositRow,
    type BankDepositDetail,
    type BankDepositView,
    type PaginatedResult,
  } from '../api';
  import Modal from '../components/Modal.svelte';
  import Pagination from '../components/Pagination.svelte';
  import {
    bankDepositTypeLabel,
    bankDepositTypeOptions,
    formatEur,
    formatSlipDate,
  } from '../i18n';
  import { appState, defaultBankDepositFilters } from '../state.svelte';

  interface Props {
    mode: 'index' | 'new' | 'show';
    id?: number;
  }

  let { mode, id }: Props = $props();

  const DEPOSIT_TYPE_CHECK = 1;

  let listing = $state<PaginatedResult<BankDepositView> | null>(null);
  let detail = $state<BankDepositDetail | null>(null);
  let loading = $state(false);
  let deleteTarget = $state<BankDepositView | null>(null);
  let deleting = $state(false);

  // New deposit form state
  let bankAccounts = $state<BankAccount[]>([]);
  let pendingChecks = $state<BankCheckDepositRow[]>([]);
  let selectedBankAccountId = $state<number | null>(null);
  let selectedCheckIds = $state<number[]>([]);
  let saving = $state(false);
  let fieldErrors = $state<Record<string, string>>({});

  let typeOptions = $derived(bankDepositTypeOptions(appState.locale));

  let selectedChecksTotal = $derived.by(() => {
    const selectedSet = new Set(selectedCheckIds);
    let cents = 0;
    for (const chk of pendingChecks) {
      if (selectedSet.has(chk.id) && chk.amount !== null) {
        cents += Math.round(chk.amount * 100);
      }
    }
    return cents / 100;
  });

  async function loadIndex(targetPage = appState.bankDepositFilters.page) {
    loading = true;
    try {
      const f = appState.bankDepositFilters;
      const parsedNum =
        f.number !== null && f.number !== undefined && Number.isInteger(Number(f.number))
          ? Number(f.number)
          : null;
      listing = await listBankDeposits({
        page: targetPage,
        per_page: 10,
        deposit_type: f.deposit_type,
        number: parsedNum,
      });
      appState.bankDepositFilters.page = listing.page;
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
        loadIndex(appState.bankDepositFilters.page);
      });
      return;
    }

    if (currentMode === 'new') {
      let cancelled = false;
      loading = true;
      fieldErrors = {};
      Promise.all([listBankAccountOptions(), listPendingBankChecks()])
        .then(([accounts, checks]) => {
          if (cancelled) return;
          bankAccounts = accounts;
          pendingChecks = checks;
          selectedBankAccountId = accounts.length > 0 ? accounts[0].id : null;
          selectedCheckIds = checks.map((c) => c.id);
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
    }

    if (currentMode === 'show' && currentId) {
      let cancelled = false;
      loading = true;
      getBankDeposit(currentId)
        .then((res) => {
          if (!cancelled) detail = res;
        })
        .catch((err) => {
          if (cancelled) return;
          appState.addFlash('error', parseApiError(err, appState.locale).message);
          appState.navigate('/bank-deposits');
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
    appState.bankDepositFilters.page = 1;
    loadIndex(1);
  }

  function handleResetFilters() {
    appState.bankDepositFilters = defaultBankDepositFilters();
    loadIndex(1);
  }

  function toggleCheck(checkId: number) {
    if (selectedCheckIds.includes(checkId)) {
      selectedCheckIds = selectedCheckIds.filter((id) => id !== checkId);
    } else {
      selectedCheckIds = [...selectedCheckIds, checkId];
    }
  }

  async function handleCreateDeposit(e: SubmitEvent) {
    e.preventDefault();
    saving = true;
    fieldErrors = {};

    try {
      const created = await createBankDeposit({
        deposit_type: DEPOSIT_TYPE_CHECK,
        bank_account_id: selectedBankAccountId,
        payment_bank_check_ids: selectedCheckIds,
      });
      appState.bankDepositFilters.page = 1;
      appState.addFlash('success', appState.tr('bankDepositCreatedSuccess'));
      appState.navigate(`/bank-deposits/${created.deposit.id}`);
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

  async function confirmDelete() {
    if (!deleteTarget) return;
    deleting = true;
    try {
      await deleteBankDeposit(deleteTarget.id);
      deleteTarget = null;
      appState.addFlash('success', appState.tr('bankDepositDeletedSuccess'));
      await loadIndex(appState.bankDepositFilters.page);
      if (listing && listing.items.length === 0 && appState.bankDepositFilters.page > 1) {
        appState.bankDepositFilters.page -= 1;
        await loadIndex(appState.bankDepositFilters.page);
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
          <h1 class="text-xl font-bold">{appState.tr('bankDepositsTitle')}</h1>
          <button
            type="button"
            class="btn btn-primary btn-sm"
            onclick={() => appState.navigate('/bank-deposits/new?deposit_type=1')}
          >
            <Plus class="w-4 h-4" />
            {appState.tr('bankDepositNewBtn')}
          </button>
        </div>

        {#if listing}
          <Pagination
            page={listing.page}
            perPage={listing.per_page}
            filteredTotal={listing.filtered_total}
            total={listing.total}
            label={appState.tr('bankDepositPlural')}
            onPageChange={(nextPage) => {
              appState.bankDepositFilters.page = nextPage;
              loadIndex(nextPage);
            }}
          />
        {/if}

        <form onsubmit={handleFilterSubmit} class="overflow-x-auto">
          <table class="table table-zebra table-sm border border-base-200">
            <thead>
              <tr>
                <th></th>
                <th>{appState.tr('bankDepositDate')}</th>
                <th>{appState.tr('bankDepositType')}</th>
                <th>{appState.tr('bankDepositNumber')}</th>
                <th class="text-right">{appState.tr('bankDepositAmount')}</th>
                <th>{appState.tr('actions')}</th>
              </tr>
              <tr class="bg-base-200/60">
                <td></td>
                <td></td>
                <td>
                  <select
                    class="select select-bordered select-xs w-full min-w-32"
                    aria-label={appState.tr('bankDepositType')}
                    bind:value={appState.bankDepositFilters.deposit_type}
                  >
                    <option value={null}>{appState.tr('bankDepositAllTypes')}</option>
                    {#each typeOptions as opt (opt.value)}
                      <option value={opt.value}>{opt.label}</option>
                    {/each}
                  </select>
                </td>
                <td>
                  <input
                    type="number"
                    min="0"
                    class="input input-bordered input-xs w-full max-w-32"
                    aria-label={appState.tr('bankDepositNumber')}
                    bind:value={appState.bankDepositFilters.number}
                  />
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
                        onclick={() => appState.navigate(`/bank-deposits/${item.id}`)}
                      >
                        <Eye class="w-3.5 h-3.5" />
                        {appState.tr('detail')}
                      </button>
                    </td>
                    <td>{item.deposit_date ? item.deposit_date.slice(0, 10) : ''}</td>
                    <td>{bankDepositTypeLabel(appState.locale, item.deposit_type)}</td>
                    <td>{item.number ?? '—'}</td>
                    <td class="text-right font-mono">{formatEur(item.amount)}</td>
                    <td>
                      <button
                        type="button"
                        class="btn btn-xs btn-error btn-outline"
                        onclick={() => (deleteTarget = item)}
                      >
                        <Trash2 class="w-3.5 h-3.5" />
                        {appState.tr('destroy')}
                      </button>
                    </td>
                  </tr>
                {/each}
              {/if}
            </tbody>
          </table>
        </form>
      </div>
    </div>
  {:else if mode === 'new'}
    <div class="max-w-4xl mx-auto">
      <div class="card bg-base-100 border border-base-300 shadow-xs">
        <div class="card-body">
          <div class="flex items-center justify-between border-b border-base-200 pb-3 mb-4">
            <h1 class="text-xl font-bold">{appState.tr('bankDepositNewTitle')}</h1>
            <button
              type="button"
              class="btn btn-sm btn-ghost"
              onclick={() => appState.navigate('/bank-deposits')}
            >
              <ArrowLeft class="w-4 h-4" />
              {appState.tr('back')}
            </button>
          </div>

          {#if loading}
            <div class="flex justify-center py-8">
              <span class="loading loading-spinner loading-md text-primary"></span>
            </div>
          {:else}
            <form onsubmit={handleCreateDeposit} class="space-y-5">
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <div class="label">
                    <span class="label-text font-medium">{appState.tr('bankDepositType')}</span>
                  </div>
                  <div class="input input-bordered flex items-center bg-base-200/40">
                    {bankDepositTypeLabel(appState.locale, DEPOSIT_TYPE_CHECK)}
                  </div>
                </div>

                <div>
                  <label class="label" for="bd-bank-account">
                    <span class="label-text font-medium">
                      {appState.tr('bankDepositBankAccount')} *
                    </span>
                  </label>
                  <select
                    id="bd-bank-account"
                    class={`select select-bordered w-full ${
                      fieldErrors.bank_account_id ? 'select-error' : ''
                    }`}
                    bind:value={selectedBankAccountId}
                  >
                    <option value={null}>—</option>
                    {#each bankAccounts as ba (ba.id)}
                      <option value={ba.id}>{ba.bank_name ?? `#${ba.id}`}</option>
                    {/each}
                  </select>
                  {#if fieldErrors.bank_account_id}
                    <p class="text-error text-xs mt-1">{fieldErrors.bank_account_id}</p>
                  {/if}
                </div>
              </div>

              {#if bankAccounts.length === 0}
                <div class="alert alert-warning text-sm">
                  <span>{appState.tr('bankDepositNoAccountsWarning')}</span>
                  <button
                    type="button"
                    class="btn btn-xs btn-outline"
                    onclick={() => appState.navigate('/bank-accounts/new')}
                  >
                    {appState.tr('bankAccountNewBtn')}
                  </button>
                </div>
              {/if}

              <div>
                <div class="flex flex-wrap items-center justify-between gap-2 mb-2">
                  <h2 class="text-base font-semibold">{appState.tr('bankDepositPendingChecks')}</h2>
                  {#if pendingChecks.length > 0}
                    <div class="flex items-center gap-2">
                      <button
                        type="button"
                        class="btn btn-xs btn-ghost"
                        onclick={() => (selectedCheckIds = pendingChecks.map((c) => c.id))}
                      >
                        {appState.tr('bankDepositSelectAll')}
                      </button>
                      <button
                        type="button"
                        class="btn btn-xs btn-ghost"
                        onclick={() => (selectedCheckIds = [])}
                      >
                        {appState.tr('bankDepositDeselectAll')}
                      </button>
                    </div>
                  {/if}
                </div>

                {#if fieldErrors.payment_bank_check_ids}
                  <p class="text-error text-xs mb-2">{fieldErrors.payment_bank_check_ids}</p>
                {/if}

                {#if pendingChecks.length === 0}
                  <div class="border border-base-200 rounded-lg p-6 text-center text-sm text-base-content/60">
                    {appState.tr('bankDepositNoPendingChecks')}
                  </div>
                {:else}
                  <div class="overflow-x-auto border border-base-200 rounded-lg">
                    <table class="table table-zebra table-sm">
                      <thead>
                        <tr>
                          <th class="w-10"></th>
                          <th>{appState.tr('paymentPaidAt')}</th>
                          <th>{appState.tr('paymentCheckOwner')}</th>
                          <th>{appState.tr('paymentCheckBankName')}</th>
                          <th>{appState.tr('paymentCheckNumber')}</th>
                          <th class="text-right">{appState.tr('paymentAmount')}</th>
                        </tr>
                      </thead>
                      <tbody>
                        {#each pendingChecks as chk (chk.id)}
                          <tr
                            class="cursor-pointer hover"
                            onclick={() => toggleCheck(chk.id)}
                          >
                            <td onclick={(e) => e.stopPropagation()}>
                              <input
                                type="checkbox"
                                class="checkbox checkbox-sm checkbox-primary"
                                checked={selectedCheckIds.includes(chk.id)}
                                onchange={() => toggleCheck(chk.id)}
                              />
                            </td>
                            <td>{chk.paid_at ?? ''}</td>
                            <td>{chk.account_owner ?? '—'}</td>
                            <td>{chk.bank_name ?? '—'}</td>
                            <td>{chk.check_number ?? '—'}</td>
                            <td class="text-right font-mono">{formatEur(chk.amount)}</td>
                          </tr>
                        {/each}
                      </tbody>
                    </table>
                  </div>

                  <div class="mt-3 text-right text-sm font-semibold">
                    {appState.tr('bankDepositSelectedSummary', {
                      count: selectedCheckIds.length,
                      total: formatEur(selectedChecksTotal),
                    })}
                  </div>
                {/if}
              </div>

              <div class="flex justify-end gap-2 pt-2">
                <button
                  type="button"
                  class="btn btn-ghost"
                  disabled={saving}
                  onclick={() => appState.navigate('/bank-deposits')}
                >
                  {appState.tr('back')}
                </button>
                <button
                  type="submit"
                  class="btn btn-primary"
                  disabled={saving || pendingChecks.length === 0 || bankAccounts.length === 0}
                >
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
        </div>
      </div>
    </div>
  {:else if mode === 'show'}
    <div class="max-w-3xl mx-auto">
      <div class="card bg-base-100 border border-base-300 shadow-xs">
        <div class="card-body">
          <div class="flex items-center justify-between border-b border-base-200 pb-3 mb-4 no-print">
            <h1 class="text-xl font-bold">{appState.tr('bankDepositDetailTitle')}</h1>
            <div class="flex items-center gap-2">
              <button
                type="button"
                class="btn btn-sm btn-primary"
                onclick={() => window.print()}
              >
                <Printer class="w-4 h-4" />
                {appState.tr('print')}
              </button>
              <button
                type="button"
                class="btn btn-sm btn-ghost"
                onclick={() => appState.navigate('/bank-deposits')}
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
            {@const dep = detail.deposit}
            {@const off = detail.office}
            <div class="space-y-6 text-sm leading-relaxed">
              <div>
                {#if off.name}
                  <div class="font-semibold text-base">{off.name}</div>
                {/if}
                {#if off.address}
                  <div>{off.address}</div>
                {/if}
                {#if off.postal_code || off.city}
                  <div>{[off.postal_code, off.city].filter(Boolean).join(' ')}</div>
                {/if}
              </div>

              <div>
                <div class="font-semibold">{dep.bank_account_name ?? '—'}</div>
                <div>
                  {appState.tr('bankDepositSlipAccountNo')}
                  {dep.bank_account_number ?? '—'}
                </div>
              </div>

              <div>
                {appState.tr('bankDepositSlipDatePrefix', {
                  city: off.city ?? '',
                  date: formatSlipDate(dep.deposit_date ?? dep.created_at, appState.locale),
                })}
              </div>

              <div class="space-y-1 font-semibold">
                <div>
                  {appState.tr('bankDepositSlipNumberLabel', {
                    number: dep.number ?? '',
                  })}
                </div>
                <div>
                  {appState.tr('bankDepositSlipCheckCount', {
                    count: detail.checks.length,
                  })}
                </div>
              </div>

              <div class="overflow-x-auto">
                <table class="table table-sm border border-base-200">
                  <thead>
                    <tr>
                      <th>{appState.tr('paymentPaidAt')}</th>
                      <th>{appState.tr('paymentCheckOwner')}</th>
                      <th>{appState.tr('paymentCheckBankName')}</th>
                      <th>{appState.tr('paymentCheckNumber')}</th>
                      <th class="text-right">{appState.tr('paymentAmount')}</th>
                    </tr>
                  </thead>
                  <tbody>
                    {#each detail.checks as chk (chk.id)}
                      <tr>
                        <td>{formatSlipDate(chk.paid_at, appState.locale)}</td>
                        <td>{chk.account_owner ?? '—'}</td>
                        <td>{chk.bank_name ?? '—'}</td>
                        <td>{chk.check_number ?? '—'}</td>
                        <td class="text-right font-mono">{formatEur(chk.amount)}</td>
                      </tr>
                    {/each}
                  </tbody>
                </table>
              </div>

              <div class="text-right text-2xl font-bold pt-2">
                {appState.tr('bankDepositSlipTotal', {
                  total: formatEur(dep.amount),
                })}
              </div>
            </div>
          {/if}
        </div>
      </div>
    </div>
  {/if}
</div>

<Modal
  open={deleteTarget !== null}
  title={appState.tr('bankDepositDeleteConfirmTitle')}
  confirmLabel={appState.tr('destroy')}
  confirmVariant="error"
  loading={deleting}
  onConfirm={confirmDelete}
  onCancel={() => (deleteTarget = null)}
>
  <p>
    {appState.tr('bankDepositDeleteConfirmBody', {
      number: deleteTarget?.number ?? '',
    })}
  </p>
</Modal>
