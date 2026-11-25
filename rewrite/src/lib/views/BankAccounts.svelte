<script lang="ts">
  import { untrack } from 'svelte';
  import { Plus, Eye, Pencil, ArrowLeft, Save, Search, RotateCcw } from 'lucide-svelte';
  import {
    createBankAccount,
    getBankAccount,
    listBankAccounts,
    parseApiError,
    updateBankAccount,
    type BankAccount,
    type PaginatedResult,
  } from '../api';
  import Pagination from '../components/Pagination.svelte';
  import { appState, type ResourceMode } from '../state.svelte';

  interface Props {
    mode: ResourceMode;
    id?: number;
  }

  let { mode, id }: Props = $props();

  let page = $state(1);
  let filterBankName = $state('');
  let listing = $state<PaginatedResult<BankAccount> | null>(null);
  let loading = $state(false);

  let current = $state<BankAccount | null>(null);
  let saving = $state(false);
  let fieldErrors = $state<Record<string, string>>({});

  let bankName = $state('');
  let accountNumber = $state<number | null>(null);
  let depositNumber = $state<number | null>(null);

  async function loadIndex(targetPage = page) {
    loading = true;
    try {
      listing = await listBankAccounts({
        page: targetPage,
        per_page: 8,
        bank_name: filterBankName,
      });
      page = listing.page;
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

    let cancelled = false;

    if (currentMode === 'index') {
      untrack(() => {
        loadIndex(page);
      });
      return () => {
        cancelled = true;
      };
    }

    if (currentMode === 'new') {
      fieldErrors = {};
      bankName = '';
      accountNumber = null;
      depositNumber = null;
      return () => {
        cancelled = true;
      };
    }

    if ((currentMode === 'show' || currentMode === 'edit') && currentId) {
      loading = true;
      fieldErrors = {};
      getBankAccount(currentId)
        .then((item) => {
          if (cancelled) return;
          current = item;
          if (currentMode === 'edit') {
            bankName = item.bank_name ?? '';
            accountNumber = item.account_number ?? null;
            depositNumber = item.bank_check_deposit_number ?? null;
          }
        })
        .catch((err) => {
          if (cancelled) return;
          appState.addFlash('error', parseApiError(err, appState.locale).message);
          appState.navigate('/bank-accounts');
        })
        .finally(() => {
          if (!cancelled) loading = false;
        });
    }

    return () => {
      cancelled = true;
    };
  });

  function handleFilterSubmit(e: SubmitEvent) {
    e.preventDefault();
    page = 1;
    loadIndex(1);
  }

  function handleResetFilters() {
    filterBankName = '';
    page = 1;
    loadIndex(1);
  }

  function parseOptionalInt(raw: number | null): { valid: boolean; value: number | null } {
    if (raw === null) return { valid: true, value: null };
    const n = Number(raw);
    if (!Number.isSafeInteger(n) || n < 0) return { valid: false, value: null };
    return { valid: true, value: n };
  }

  async function handleSave(e: SubmitEvent) {
    e.preventDefault();
    saving = true;
    fieldErrors = {};

    const acc = parseOptionalInt(accountNumber);
    const dep = parseOptionalInt(depositNumber);
    const clientErrors: Record<string, string> = {};
    if (!acc.valid) clientErrors.account_number = appState.tr('err_invalid_integer');
    if (!dep.valid) clientErrors.bank_check_deposit_number = appState.tr('err_invalid_integer');
    if (Object.keys(clientErrors).length > 0) {
      fieldErrors = clientErrors;
      saving = false;
      return;
    }

    const payload = {
      bank_name: bankName,
      account_number: acc.value,
      bank_check_deposit_number: dep.value,
    };

    try {
      if (mode === 'new') {
        const created = await createBankAccount(payload);
        appState.addFlash('success', appState.tr('bankAccountCreatedSuccess'));
        appState.navigate(`/bank-accounts/${created.id}`);
      } else if (mode === 'edit' && id) {
        const updated = await updateBankAccount(id, payload);
        appState.addFlash('success', appState.tr('bankAccountUpdatedSuccess'));
        appState.navigate(`/bank-accounts/${updated.id}`);
      }
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

<div class="max-w-5xl mx-auto px-4 py-6">
  {#if mode === 'index'}
    <div class="card bg-base-100 border border-base-300 shadow-xs">
      <div class="card-body">
        <div class="flex flex-wrap items-center justify-between gap-3 border-b border-base-200 pb-3">
          <h1 class="text-xl font-bold">{appState.tr('bankAccountsTitle')}</h1>
          <button
            type="button"
            class="btn btn-primary btn-sm"
            onclick={() => appState.navigate('/bank-accounts/new')}
          >
            <Plus class="w-4 h-4" />
            {appState.tr('bankAccountNewBtn')}
          </button>
        </div>

        {#if listing}
          <Pagination
            page={listing.page}
            perPage={listing.per_page}
            filteredTotal={listing.filtered_total}
            total={listing.total}
            label={appState.tr('bankAccountPlural')}
            onPageChange={(nextPage) => {
              page = nextPage;
              loadIndex(nextPage);
            }}
          />
        {/if}

        <form onsubmit={handleFilterSubmit} class="overflow-x-auto">
          <table class="table table-zebra table-sm border border-base-200">
            <thead>
              <tr>
                <th></th>
                <th>{appState.tr('bankAccountName')}</th>
                <th>{appState.tr('bankAccountNumber')}</th>
                <th>{appState.tr('bankAccountDepositNumber')}</th>
                <th>{appState.tr('actions')}</th>
              </tr>
              <tr class="bg-base-200/60">
                <td></td>
                <td>
                  <input
                    type="text"
                    class="input input-bordered input-xs w-full max-w-xs"
                    aria-label={appState.tr('bankAccountName')}
                    bind:value={filterBankName}
                  />
                </td>
                <td></td>
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
                  <td colspan="5" class="text-center py-8">
                    <span class="loading loading-spinner loading-md text-primary"></span>
                  </td>
                </tr>
              {:else if listing && listing.items.length === 0}
                <tr>
                  <td colspan="5" class="text-center py-6 text-base-content/60">
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
                        onclick={() => appState.navigate(`/bank-accounts/${item.id}`)}
                      >
                        <Eye class="w-3.5 h-3.5" />
                        {appState.tr('detail')}
                      </button>
                    </td>
                    <td>{item.bank_name ?? ''}</td>
                    <td>{item.account_number ?? '—'}</td>
                    <td>{item.bank_check_deposit_number ?? '—'}</td>
                    <td>
                      <button
                        type="button"
                        class="btn btn-xs btn-ghost"
                        onclick={() => appState.navigate(`/bank-accounts/${item.id}/edit`)}
                      >
                        <Pencil class="w-3.5 h-3.5" />
                        {appState.tr('edit')}
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
  {:else if mode === 'show'}
    <div class="max-w-2xl mx-auto">
      <div class="card bg-base-100 border border-base-300 shadow-xs">
        <div class="card-body">
          <div class="flex items-center justify-between border-b border-base-200 pb-3 mb-4">
            <h1 class="text-xl font-bold">{appState.tr('bankAccountDetailTitle')}</h1>
            <div class="flex items-center gap-2">
              {#if current}
                <button
                  type="button"
                  class="btn btn-sm btn-outline"
                  onclick={() => appState.navigate(`/bank-accounts/${current!.id}/edit`)}
                >
                  <Pencil class="w-4 h-4" />
                  {appState.tr('edit')}
                </button>
              {/if}
              <button
                type="button"
                class="btn btn-sm btn-ghost"
                onclick={() => appState.navigate('/bank-accounts')}
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
          {:else if current}
            <table class="table table-bordered">
              <tbody>
                <tr>
                  <th class="w-52 bg-base-200/50">{appState.tr('bankAccountName')}</th>
                  <td>{current.bank_name ?? ''}</td>
                </tr>
                <tr>
                  <th class="bg-base-200/50">{appState.tr('bankAccountNumber')}</th>
                  <td>{current.account_number ?? '—'}</td>
                </tr>
                <tr>
                  <th class="bg-base-200/50">{appState.tr('bankAccountDepositNumber')}</th>
                  <td>{current.bank_check_deposit_number ?? '—'}</td>
                </tr>
              </tbody>
            </table>
          {/if}
        </div>
      </div>
    </div>
  {:else}
    <div class="max-w-2xl mx-auto">
      <div class="card bg-base-100 border border-base-300 shadow-xs">
        <div class="card-body">
          <div class="flex items-center justify-between border-b border-base-200 pb-3 mb-4">
            <h1 class="text-xl font-bold">
              {mode === 'new'
                ? appState.tr('bankAccountNewTitle')
                : appState.tr('bankAccountEditTitle')}
            </h1>
            <button
              type="button"
              class="btn btn-sm btn-ghost"
              onclick={() => appState.navigate('/bank-accounts')}
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
            <form onsubmit={handleSave} class="space-y-4">
              <div>
                <label class="label" for="ba-name">
                  <span class="label-text font-medium">{appState.tr('bankAccountName')} *</span>
                </label>
                <input
                  id="ba-name"
                  type="text"
                  class={`input input-bordered w-full ${fieldErrors.bank_name ? 'input-error' : ''}`}
                  bind:value={bankName}
                />
                {#if fieldErrors.bank_name}
                  <p class="text-error text-xs mt-1">{fieldErrors.bank_name}</p>
                {/if}
              </div>

              <div>
                <label class="label" for="ba-account">
                  <span class="label-text font-medium">{appState.tr('bankAccountNumber')}</span>
                </label>
                <input
                  id="ba-account"
                  type="number"
                  min="0"
                  class={`input input-bordered w-full ${
                    fieldErrors.account_number ? 'input-error' : ''
                  }`}
                  bind:value={accountNumber}
                />
                {#if fieldErrors.account_number}
                  <p class="text-error text-xs mt-1">{fieldErrors.account_number}</p>
                {/if}
              </div>

              <div>
                <label class="label" for="ba-deposit">
                  <span class="label-text font-medium">
                    {appState.tr('bankAccountDepositNumber')}
                  </span>
                </label>
                <input
                  id="ba-deposit"
                  type="number"
                  min="0"
                  class={`input input-bordered w-full ${
                    fieldErrors.bank_check_deposit_number ? 'input-error' : ''
                  }`}
                  bind:value={depositNumber}
                />
                {#if fieldErrors.bank_check_deposit_number}
                  <p class="text-error text-xs mt-1">{fieldErrors.bank_check_deposit_number}</p>
                {/if}
              </div>

              <div class="flex justify-end gap-2 pt-2">
                <button
                  type="button"
                  class="btn btn-ghost"
                  onclick={() => appState.navigate('/bank-accounts')}
                >
                  {appState.tr('back')}
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
        </div>
      </div>
    </div>
  {/if}
</div>
