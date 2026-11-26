<script lang="ts">
  import { untrack } from 'svelte';
  import { Plus, Eye, Pencil, Trash2, ArrowLeft, Search, RotateCcw } from 'lucide-svelte';
  import {
    deleteTreatment,
    listTreatmentTypeOptions,
    listTreatments,
    parseApiError,
    type MedicalTreatmentType,
    type MedicalTreatmentView,
    type PaginatedResult,
  } from '../api';
  import Modal from '../components/Modal.svelte';
  import Pagination from '../components/Pagination.svelte';
  import PatientAutocomplete from '../components/PatientAutocomplete.svelte';
  import TreatmentForm from '../components/TreatmentForm.svelte';
  import {
    locationLabel,
    locationOptions,
    treatmentStatusLabel,
    treatmentStatusOptions,
  } from '../i18n';
  import { appState, defaultTreatmentFilters } from '../state.svelte';

  interface Props {
    mode: 'index' | 'new' | 'edit';
    id?: number;
    query?: Record<string, string>;
  }

  let { mode, id, query = {} }: Props = $props();

  const PER_PAGE_OPTIONS = [
    { value: 2, label: '2' },
    { value: 10, label: '10' },
    { value: 20, label: '20' },
    { value: 50, label: '50' },
    { value: 100, label: '100' },
    { value: 10000000, labelKey: 'allOption' as const },
  ];

  let listing = $state<PaginatedResult<MedicalTreatmentView> | null>(null);
  let treatmentTypes = $state<MedicalTreatmentType[]>([]);
  let loading = $state(false);
  let deleteTarget = $state<MedicalTreatmentView | null>(null);
  let deleting = $state(false);

  let locOptions = $derived(locationOptions(appState.locale));
  let statusOptions = $derived(treatmentStatusOptions(appState.locale));

  let initialNewPatientId = $derived.by(() => {
    if (query.patient_id) {
      const n = Number(query.patient_id);
      if (Number.isInteger(n) && n > 0) return n;
    }
    return null;
  });

  async function loadIndex(targetPage = appState.treatmentFilters.page) {
    loading = true;
    try {
      const f = appState.treatmentFilters;
      listing = await listTreatments({
        page: targetPage,
        per_page: f.per_page,
        patient_id: f.patient_id,
        medical_treatment_type_id: f.medical_treatment_type_id,
        date_gteq: f.date_gteq || undefined,
        date_lteq: f.date_lteq || undefined,
        location_id: f.location_id,
        status: f.status,
      });
      appState.treatmentFilters.page = listing.page;
    } catch (err) {
      appState.addFlash('error', parseApiError(err, appState.locale).message);
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    const activeUserId = appState.currentUser?.id;
    const currentMode = mode;
    const currentQuery = query;
    if (!appState.isInitialized || !activeUserId) return;

    if (currentMode !== 'index') return;

    let cancelled = false;

    untrack(() => {
      const hasPatientQuery =
        currentQuery.patient_id !== undefined && currentQuery.patient_id !== '';
      const hasStatusQuery = currentQuery.status !== undefined && currentQuery.status !== '';

      if (hasPatientQuery || hasStatusQuery) {
        const perPage = appState.treatmentFilters.per_page;
        const next = { ...defaultTreatmentFilters(), per_page: perPage };
        if (hasPatientQuery) {
          const pid = Number(currentQuery.patient_id);
          if (Number.isInteger(pid) && pid > 0) next.patient_id = pid;
        }
        if (hasStatusQuery) {
          const st = Number(currentQuery.status);
          if (st === 0 || st === 1) next.status = st;
        }
        appState.treatmentFilters = next;

        if (typeof window !== 'undefined') {
          window.history.replaceState(null, '', '#/medical-treatments');
        }
      }

      Promise.all([
        listTreatmentTypeOptions(),
        loadIndex(appState.treatmentFilters.page),
      ])
        .then(([types]) => {
          if (!cancelled) treatmentTypes = types;
        })
        .catch((err) => {
          if (!cancelled) {
            appState.addFlash('error', parseApiError(err, appState.locale).message);
          }
        });
    });

    return () => {
      cancelled = true;
    };
  });

  function handleFilterSubmit(e: SubmitEvent) {
    e.preventDefault();
    appState.treatmentFilters.page = 1;
    loadIndex(1);
  }

  function handleResetFilters() {
    const perPage = appState.treatmentFilters.per_page;
    appState.treatmentFilters = { ...defaultTreatmentFilters(), per_page: perPage };
    loadIndex(1);
  }

  function handlePerPageChange() {
    appState.treatmentFilters.page = 1;
    loadIndex(1);
  }

  function formatPatientName(tr: MedicalTreatmentView): string {
    return [tr.patient_first_name, tr.patient_last_name].filter(Boolean).join(' ').trim() || '—';
  }

  async function confirmDelete() {
    if (!deleteTarget) return;
    deleting = true;
    try {
      await deleteTreatment(deleteTarget.id);
      deleteTarget = null;
      appState.addFlash('success', appState.tr('treatmentDeletedSuccess'));
      await loadIndex(appState.treatmentFilters.page);
      if (listing && listing.items.length === 0 && appState.treatmentFilters.page > 1) {
        appState.treatmentFilters.page -= 1;
        await loadIndex(appState.treatmentFilters.page);
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
          <h1 class="text-xl font-bold">{appState.tr('treatmentsTitle')}</h1>
          <button
            type="button"
            class="btn btn-primary btn-sm"
            onclick={() => appState.navigate('/medical-treatments/new')}
          >
            <Plus class="w-4 h-4" />
            {appState.tr('treatmentNewBtn')}
          </button>
        </div>

        <div class="flex flex-wrap items-center justify-between gap-3 pt-1">
          <div class="flex items-center gap-2 text-sm">
            <span>{appState.tr('showCountPrefix')}</span>
            <select
              class="select select-bordered select-xs"
              aria-label={appState.tr('perPageSuffix')}
              bind:value={appState.treatmentFilters.per_page}
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
            label={appState.tr('treatmentPlural')}
            onPageChange={(nextPage) => {
              appState.treatmentFilters.page = nextPage;
              loadIndex(nextPage);
            }}
          />
        {/if}

        <form onsubmit={handleFilterSubmit} class="overflow-x-auto">
          <table class="table table-zebra table-sm border border-base-200">
            <thead>
              <tr>
                <th></th>
                <th>{appState.tr('treatmentPatient')}</th>
                <th>{appState.tr('treatmentType')}</th>
                <th>{appState.tr('treatmentDate')}</th>
                <th>{appState.tr('treatmentLocation')}</th>
                <th>{appState.tr('treatmentStatus')}</th>
                <th>{appState.tr('actions')}</th>
              </tr>
              <tr class="bg-base-200/60">
                <td></td>
                <td>
                  <PatientAutocomplete
                    compact
                    bind:value={appState.treatmentFilters.patient_id}
                  />
                </td>
                <td>
                  <select
                    class="select select-bordered select-xs w-full min-w-32"
                    aria-label={appState.tr('treatmentType')}
                    bind:value={appState.treatmentFilters.medical_treatment_type_id}
                  >
                    <option value={null}>{appState.tr('treatmentAllTypes')}</option>
                    {#each treatmentTypes as tt (tt.id)}
                      <option value={tt.id}>{tt.name ?? `#${tt.id}`}</option>
                    {/each}
                  </select>
                </td>
                <td>
                  <div class="flex items-center gap-1 min-w-60">
                    <input
                      type="date"
                      class="input input-bordered input-xs w-full"
                      aria-label="Date from"
                      bind:value={appState.treatmentFilters.date_gteq}
                    />
                    <span class="text-xs text-base-content/60">{appState.tr('toRange')}</span>
                    <input
                      type="date"
                      class="input input-bordered input-xs w-full"
                      aria-label="Date to"
                      bind:value={appState.treatmentFilters.date_lteq}
                    />
                  </div>
                </td>
                <td>
                  <select
                    class="select select-bordered select-xs w-full min-w-24"
                    aria-label={appState.tr('treatmentLocation')}
                    bind:value={appState.treatmentFilters.location_id}
                  >
                    <option value={null}>{appState.tr('treatmentAllLocations')}</option>
                    {#each locOptions as opt (opt.value)}
                      <option value={opt.value}>{opt.label}</option>
                    {/each}
                  </select>
                </td>
                <td>
                  <select
                    class="select select-bordered select-xs w-full min-w-24"
                    aria-label={appState.tr('treatmentStatus')}
                    bind:value={appState.treatmentFilters.status}
                  >
                    <option value={null}>{appState.tr('treatmentAllStatuses')}</option>
                    {#each statusOptions as opt (opt.value)}
                      <option value={opt.value}>{opt.label}</option>
                    {/each}
                  </select>
                </td>
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
                  <td colspan="7" class="text-center py-8">
                    <span class="loading loading-spinner loading-md text-primary"></span>
                  </td>
                </tr>
              {:else if listing && listing.items.length === 0}
                <tr>
                  <td colspan="7" class="text-center py-6 text-base-content/60">
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
                        onclick={() => appState.navigate(`/medical-treatments/${item.id}`)}
                      >
                        <Eye class="w-3.5 h-3.5" />
                        {appState.tr('detail')}
                      </button>
                    </td>
                    <td>
                      {#if item.patient_id}
                        <a
                          href={`#/patients/${item.patient_id}`}
                          class="link link-hover"
                          onclick={(e) => {
                            e.preventDefault();
                            appState.navigate(`/patients/${item.patient_id}`);
                          }}
                        >
                          {formatPatientName(item)}
                        </a>
                      {:else}
                        {formatPatientName(item)}
                      {/if}
                    </td>
                    <td>{item.treatment_type_name ?? '—'}</td>
                    <td>{item.date ?? ''}</td>
                    <td>{locationLabel(appState.locale, item.location_id)}</td>
                    <td>
                      <span
                        class={`badge badge-xs ${
                          item.status === 1 ? 'badge-success' : 'badge-warning'
                        }`}
                      >
                        {treatmentStatusLabel(appState.locale, item.status)}
                      </span>
                    </td>
                    <td class="whitespace-nowrap">
                      <div class="flex items-center gap-1">
                        <button
                          type="button"
                          class="btn btn-xs btn-ghost"
                          onclick={() => appState.navigate(`/medical-treatments/${item.id}/edit`)}
                        >
                          <Pencil class="w-3.5 h-3.5" />
                          {appState.tr('edit')}
                        </button>
                        <button
                          type="button"
                          class="btn btn-xs btn-error btn-outline"
                          disabled={item.payments_count > 0}
                          onclick={() => {
                            if (item.payments_count === 0) deleteTarget = item;
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
  {:else}
    <div class="max-w-3xl mx-auto">
      <div class="card bg-base-100 border border-base-300 shadow-xs">
        <div class="card-body">
          <div class="flex items-center justify-between border-b border-base-200 pb-3 mb-4">
            <h1 class="text-xl font-bold">
              {mode === 'new'
                ? appState.tr('treatmentNewTitle')
                : appState.tr('treatmentEditTitle')}
            </h1>
            <button
              type="button"
              class="btn btn-sm btn-ghost"
              onclick={() =>
                appState.navigate(
                  mode === 'edit' && id ? `/medical-treatments/${id}` : '/medical-treatments',
                )}
            >
              <ArrowLeft class="w-4 h-4" />
              {appState.tr('back')}
            </button>
          </div>

          <TreatmentForm
            treatmentId={mode === 'edit' ? id : undefined}
            initialPatientId={mode === 'new' ? initialNewPatientId : null}
            cancelLabel={appState.tr('back')}
            onSaved={(saved) => {
              if (mode === 'new') {
                appState.treatmentFilters.page = 1;
                appState.addFlash('success', appState.tr('treatmentCreatedSuccess'));
              } else {
                appState.addFlash('success', appState.tr('treatmentUpdatedSuccess'));
              }
              appState.navigate(`/medical-treatments/${saved.id}`);
            }}
            onCancel={() =>
              appState.navigate(
                mode === 'edit' && id ? `/medical-treatments/${id}` : '/medical-treatments',
              )}
          />
        </div>
      </div>
    </div>
  {/if}
</div>

<Modal
  open={deleteTarget !== null}
  title={appState.tr('treatmentDeleteConfirmTitle')}
  confirmLabel={appState.tr('destroy')}
  confirmVariant="error"
  loading={deleting}
  onConfirm={confirmDelete}
  onCancel={() => (deleteTarget = null)}
>
  <p>
    {appState.tr('treatmentDeleteConfirmBody', {
      date: deleteTarget?.date ?? '',
    })}
  </p>
</Modal>