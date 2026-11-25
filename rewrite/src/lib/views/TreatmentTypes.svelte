<script lang="ts">
  import { untrack } from 'svelte';
  import { Plus, Eye, Pencil, Trash2, ArrowLeft, Save, Search, RotateCcw } from 'lucide-svelte';
  import {
    createTreatmentType,
    deleteTreatmentType,
    getTreatmentType,
    listTreatmentTypes,
    parseApiError,
    updateTreatmentType,
    type MedicalTreatmentType,
    type PaginatedResult,
  } from '../api';
  import Modal from '../components/Modal.svelte';
  import Pagination from '../components/Pagination.svelte';
  import { appState, type ResourceMode } from '../state.svelte';

  interface Props {
    mode: ResourceMode;
    id?: number;
  }

  let { mode, id }: Props = $props();

  let page = $state(1);
  let filterName = $state('');
  let listing = $state<PaginatedResult<MedicalTreatmentType> | null>(null);
  let loading = $state(false);

  let current = $state<MedicalTreatmentType | null>(null);
  let saving = $state(false);
  let fieldErrors = $state<Record<string, string>>({});

  let name = $state('');
  let price = $state<number | null>(null);

  let deleteTarget = $state<MedicalTreatmentType | null>(null);
  let deleting = $state(false);

  async function loadIndex(targetPage = page) {
    loading = true;
    try {
      listing = await listTreatmentTypes({
        page: targetPage,
        per_page: 8,
        name: filterName,
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
      name = '';
      price = null;
      return () => {
        cancelled = true;
      };
    }

    if ((currentMode === 'show' || currentMode === 'edit') && currentId) {
      loading = true;
      fieldErrors = {};
      getTreatmentType(currentId)
        .then((item) => {
          if (cancelled) return;
          current = item;
          if (currentMode === 'edit') {
            name = item.name ?? '';
            price = item.price ?? null;
          }
        })
        .catch((err) => {
          if (cancelled) return;
          appState.addFlash('error', parseApiError(err, appState.locale).message);
          appState.navigate('/treatment-types');
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
    filterName = '';
    page = 1;
    loadIndex(1);
  }

  async function handleSave(e: SubmitEvent) {
    e.preventDefault();
    saving = true;
    fieldErrors = {};

    const parsedPrice = price === null ? null : Number(price);
    if (parsedPrice !== null && (!Number.isFinite(parsedPrice) || parsedPrice < 0)) {
      fieldErrors = { price: appState.tr('err_invalid_price') };
      saving = false;
      return;
    }

    try {
      if (mode === 'new') {
        await createTreatmentType({ name, price: parsedPrice });
        page = 1;
        appState.addFlash('success', appState.tr('treatmentTypeCreatedSuccess'));
        appState.navigate('/treatment-types');
      } else if (mode === 'edit' && id) {
        const updated = await updateTreatmentType(id, { name, price: parsedPrice });
        appState.addFlash('success', appState.tr('treatmentTypeUpdatedSuccess'));
        appState.navigate(`/treatment-types/${updated.id}`);
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

  async function confirmDelete() {
    if (!deleteTarget) return;
    deleting = true;
    try {
      await deleteTreatmentType(deleteTarget.id);
      deleteTarget = null;
      appState.addFlash('success', appState.tr('treatmentTypeDeletedSuccess'));
      await loadIndex(page);
      if (listing && listing.items.length === 0 && page > 1) {
        page = page - 1;
        await loadIndex(page);
      }
    } catch (err) {
      deleteTarget = null;
      appState.addFlash('error', parseApiError(err, appState.locale).message);
    } finally {
      deleting = false;
    }
  }
</script>

<div class="max-w-5xl mx-auto px-4 py-6">
  {#if mode === 'index'}
    <div class="card bg-base-100 border border-base-300 shadow-xs">
      <div class="card-body">
        <div class="flex flex-wrap items-center justify-between gap-3 border-b border-base-200 pb-3">
          <h1 class="text-xl font-bold">{appState.tr('treatmentTypesTitle')}</h1>
          <button
            type="button"
            class="btn btn-primary btn-sm"
            onclick={() => appState.navigate('/treatment-types/new')}
          >
            <Plus class="w-4 h-4" />
            {appState.tr('treatmentTypeNewBtn')}
          </button>
        </div>

        {#if listing}
          <Pagination
            page={listing.page}
            perPage={listing.per_page}
            filteredTotal={listing.filtered_total}
            total={listing.total}
            label={appState.tr('treatmentTypePlural')}
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
                <th>{appState.tr('treatmentTypeName')}</th>
                <th>{appState.tr('treatmentTypePrice')}</th>
                <th>{appState.tr('actions')}</th>
              </tr>
              <tr class="bg-base-200/60">
                <td></td>
                <td>
                  <input
                    type="text"
                    class="input input-bordered input-xs w-full max-w-xs"
                    aria-label={appState.tr('treatmentTypeName')}
                    bind:value={filterName}
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
                  <td colspan="4" class="text-center py-8">
                    <span class="loading loading-spinner loading-md text-primary"></span>
                  </td>
                </tr>
              {:else if listing && listing.items.length === 0}
                <tr>
                  <td colspan="4" class="text-center py-6 text-base-content/60">
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
                        onclick={() => appState.navigate(`/treatment-types/${item.id}`)}
                      >
                        <Eye class="w-3.5 h-3.5" />
                        {appState.tr('show')}
                      </button>
                    </td>
                    <td>{item.name ?? ''}</td>
                    <td>{item.price !== null && item.price !== undefined ? item.price.toFixed(2) : '—'}</td>
                    <td class="whitespace-nowrap">
                      <div class="flex items-center gap-1">
                        <button
                          type="button"
                          class="btn btn-xs btn-ghost"
                          onclick={() => appState.navigate(`/treatment-types/${item.id}/edit`)}
                        >
                          <Pencil class="w-3.5 h-3.5" />
                          {appState.tr('edit')}
                        </button>
                        <button
                          type="button"
                          class="btn btn-xs btn-error btn-outline"
                          onclick={() => (deleteTarget = item)}
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
    <div class="max-w-2xl mx-auto">
      <div class="card bg-base-100 border border-base-300 shadow-xs">
        <div class="card-body">
          <div class="flex items-center justify-between border-b border-base-200 pb-3 mb-4">
            <h1 class="text-xl font-bold">{appState.tr('treatmentTypeDetailTitle')}</h1>
            <div class="flex items-center gap-2">
              {#if current}
                <button
                  type="button"
                  class="btn btn-sm btn-outline"
                  onclick={() => appState.navigate(`/treatment-types/${current!.id}/edit`)}
                >
                  <Pencil class="w-4 h-4" />
                  {appState.tr('edit')}
                </button>
              {/if}
              <button
                type="button"
                class="btn btn-sm btn-ghost"
                onclick={() => appState.navigate('/treatment-types')}
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
                  <th class="w-40 bg-base-200/50">{appState.tr('treatmentTypeName')}</th>
                  <td>{current.name ?? ''}</td>
                </tr>
                <tr>
                  <th class="bg-base-200/50">{appState.tr('treatmentTypePrice')}</th>
                  <td>
                    {current.price !== null && current.price !== undefined
                      ? current.price.toFixed(2)
                      : '—'}
                  </td>
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
                ? appState.tr('treatmentTypeNewTitle')
                : appState.tr('treatmentTypeEditTitle')}
            </h1>
            <button
              type="button"
              class="btn btn-sm btn-ghost"
              onclick={() => appState.navigate('/treatment-types')}
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
                <label class="label" for="tt-name">
                  <span class="label-text font-medium">{appState.tr('treatmentTypeName')} *</span>
                </label>
                <input
                  id="tt-name"
                  type="text"
                  class={`input input-bordered w-full ${fieldErrors.name ? 'input-error' : ''}`}
                  bind:value={name}
                />
                {#if fieldErrors.name}
                  <p class="text-error text-xs mt-1">{fieldErrors.name}</p>
                {/if}
              </div>

              <div>
                <label class="label" for="tt-price">
                  <span class="label-text font-medium">{appState.tr('treatmentTypePrice')} *</span>
                </label>
                <input
                  id="tt-price"
                  type="number"
                  step="0.01"
                  min="0"
                  class={`input input-bordered w-full ${fieldErrors.price ? 'input-error' : ''}`}
                  bind:value={price}
                />
                {#if fieldErrors.price}
                  <p class="text-error text-xs mt-1">{fieldErrors.price}</p>
                {/if}
              </div>

              <div class="flex justify-end gap-2 pt-2">
                <button
                  type="button"
                  class="btn btn-ghost"
                  onclick={() => appState.navigate('/treatment-types')}
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

<Modal
  open={deleteTarget !== null}
  title={appState.tr('treatmentTypeDeleteConfirmTitle')}
  confirmLabel={appState.tr('destroy')}
  confirmVariant="error"
  loading={deleting}
  onConfirm={confirmDelete}
  onCancel={() => (deleteTarget = null)}
>
  <p>
    {appState.tr('treatmentTypeDeleteConfirmBody', {
      name: deleteTarget?.name ?? '',
    })}
  </p>
</Modal>
