<script lang="ts">
  import { untrack } from 'svelte';
  import {
    Plus,
    Upload,
    Eye,
    Pencil,
    Trash2,
    Search,
    RotateCcw,
    ChevronDown,
    Stethoscope,
  } from 'lucide-svelte';
  import {
    deletePatient,
    listPatients,
    parseApiError,
    type PaginatedResult,
    type PatientView,
  } from '../api';
  import Modal from '../components/Modal.svelte';
  import Pagination from '../components/Pagination.svelte';
  import { appState, defaultPatientFilters } from '../state.svelte';

  const PER_PAGE_OPTIONS = [
    { value: 2, label: '2' },
    { value: 10, label: '10' },
    { value: 20, label: '20' },
    { value: 50, label: '50' },
    { value: 100, label: '100' },
    { value: 10000000, labelKey: 'allOption' as const },
  ];

  let listing = $state<PaginatedResult<PatientView> | null>(null);
  let loading = $state(false);
  let deleteTarget = $state<PatientView | null>(null);
  let deleting = $state(false);

  async function loadIndex(targetPage = appState.patientFilters.page) {
    loading = true;
    try {
      const f = appState.patientFilters;
      listing = await listPatients({
        page: targetPage,
        per_page: f.per_page,
        first_name: f.first_name,
        last_name: f.last_name,
        phone: f.phone,
        diabetic: f.diabetic ? true : undefined,
      });
      appState.patientFilters.page = listing.page;
    } catch (err) {
      appState.addFlash('error', parseApiError(err, appState.locale).message);
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    const activeUserId = appState.currentUser?.id;
    if (!appState.isInitialized || !activeUserId) return;
    untrack(() => {
      loadIndex(appState.patientFilters.page);
    });
  });

  function handleFilterSubmit(e: SubmitEvent) {
    e.preventDefault();
    appState.patientFilters.page = 1;
    loadIndex(1);
  }

  function handleResetFilters() {
    const perPage = appState.patientFilters.per_page;
    appState.patientFilters = { ...defaultPatientFilters(), per_page: perPage };
    loadIndex(1);
  }

  function handlePerPageChange() {
    appState.patientFilters.page = 1;
    loadIndex(1);
  }

  function formatPhones(p: PatientView): string {
    return [p.phone, p.mobile_phone].filter((v) => v && v.trim()).join(' - ');
  }

  function closeDropdownMenu() {
    if (document.activeElement instanceof HTMLElement) {
      document.activeElement.blur();
    }
  }

  async function confirmDelete() {
    if (!deleteTarget) return;
    deleting = true;
    try {
      await deletePatient(deleteTarget.id);
      deleteTarget = null;
      appState.addFlash('success', appState.tr('patientDeletedSuccess'));
      await loadIndex(appState.patientFilters.page);
      if (listing && listing.items.length === 0 && appState.patientFilters.page > 1) {
        appState.patientFilters.page -= 1;
        await loadIndex(appState.patientFilters.page);
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
  <div class="card bg-base-100 border border-base-300 shadow-xs">
    <div class="card-body">
      <div class="flex flex-wrap items-center justify-between gap-3 border-b border-base-200 pb-3">
        <h1 class="text-xl font-bold">{appState.tr('patientsTitle')}</h1>
        <div class="flex items-center gap-2">
          <button
            type="button"
            class="btn btn-outline btn-sm"
            onclick={() => appState.navigate('/patients/import')}
          >
            <Upload class="w-4 h-4" />
            {appState.tr('patientImportBtn')}
          </button>
          <button
            type="button"
            class="btn btn-primary btn-sm"
            onclick={() => appState.navigate('/patients/new')}
          >
            <Plus class="w-4 h-4" />
            {appState.tr('patientNewBtn')}
          </button>
        </div>
      </div>

      <div class="flex flex-wrap items-center justify-between gap-3 pt-1">
        <div class="flex items-center gap-2 text-sm">
          <span>{appState.tr('showCountPrefix')}</span>
          <select
            class="select select-bordered select-xs"
            aria-label={appState.tr('perPageSuffix')}
            bind:value={appState.patientFilters.per_page}
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
          label={appState.tr('patientPlural')}
          onPageChange={(nextPage) => {
            appState.patientFilters.page = nextPage;
            loadIndex(nextPage);
          }}
        />
      {/if}

      <form onsubmit={handleFilterSubmit} class="overflow-x-auto">
        <table class="table table-zebra table-sm border border-base-200">
          <thead>
            <tr>
              <th></th>
              <th>{appState.tr('patientFirstName')}</th>
              <th>{appState.tr('patientLastName')}</th>
              <th>{appState.tr('patientPhone')}</th>
              <th>{appState.tr('patientDiabetic')}</th>
              <th>{appState.tr('patientTreatmentsCount')}</th>
              <th>{appState.tr('actions')}</th>
            </tr>
            <tr class="bg-base-200/60">
              <td></td>
              <td>
                <input
                  type="text"
                  class="input input-bordered input-xs w-full min-w-28"
                  aria-label={appState.tr('patientFirstName')}
                  bind:value={appState.patientFilters.first_name}
                />
              </td>
              <td>
                <input
                  type="text"
                  class="input input-bordered input-xs w-full min-w-28"
                  aria-label={appState.tr('patientLastName')}
                  bind:value={appState.patientFilters.last_name}
                />
              </td>
              <td>
                <input
                  type="text"
                  class="input input-bordered input-xs w-full min-w-32"
                  aria-label={appState.tr('patientPhone')}
                  bind:value={appState.patientFilters.phone}
                />
              </td>
              <td class="text-center">
                <input
                  type="checkbox"
                  class="checkbox checkbox-xs"
                  aria-label={appState.tr('patientDiabetic')}
                  bind:checked={appState.patientFilters.diabetic}
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
                      onclick={() => appState.navigate(`/patients/${item.id}`)}
                    >
                      <Eye class="w-3.5 h-3.5" />
                      {appState.tr('detail')}
                    </button>
                  </td>
                  <td>{item.first_name ?? ''}</td>
                  <td>{item.last_name ?? ''}</td>
                  <td>{formatPhones(item)}</td>
                  <td>
                    {#if item.diabetic}
                      <span class="badge badge-warning badge-xs">{appState.tr('yes')}</span>
                    {:else}
                      <span class="text-base-content/50">{appState.tr('no')}</span>
                    {/if}
                  </td>
                  <td>{item.treatments_count}</td>
                  <td class="whitespace-nowrap">
                    <div class="join">
                      <button
                        type="button"
                        class="btn btn-xs btn-outline join-item"
                        onclick={() => appState.navigate(`/patients/${item.id}/edit`)}
                      >
                        <Pencil class="w-3.5 h-3.5" />
                        {appState.tr('edit')}
                      </button>
                      <div class="dropdown dropdown-end join-item">
                        <button
                          type="button"
                          tabindex="0"
                          class="btn btn-xs btn-outline join-item px-1.5"
                          aria-label={appState.tr('actions')}
                        >
                          <ChevronDown class="w-3.5 h-3.5" />
                        </button>
                        <ul
                          class="dropdown-content menu bg-base-100 rounded-box z-50 w-52 p-1.5 shadow-lg border border-base-300 mt-1"
                        >
                          <li>
                            <button
                              type="button"
                              onclick={() => {
                                closeDropdownMenu();
                                appState.navigate(`/medical-treatments?patient_id=${item.id}`);
                              }}
                            >
                              <Stethoscope class="w-3.5 h-3.5" />
                              {appState.tr('navMedicalTreatments')}
                            </button>
                          </li>
                          <li>
                            <button
                              type="button"
                              class={item.treatments_count > 0 ? 'disabled opacity-40' : 'text-error'}
                              disabled={item.treatments_count > 0}
                              onclick={() => {
                                if (item.treatments_count > 0) return;
                                closeDropdownMenu();
                                deleteTarget = item;
                              }}
                            >
                              <Trash2 class="w-3.5 h-3.5" />
                              {appState.tr('destroy')}
                            </button>
                          </li>
                        </ul>
                      </div>
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
</div>

<Modal
  open={deleteTarget !== null}
  title={appState.tr('patientDeleteConfirmTitle')}
  confirmLabel={appState.tr('destroy')}
  confirmVariant="error"
  loading={deleting}
  onConfirm={confirmDelete}
  onCancel={() => (deleteTarget = null)}
>
  <p>
    {appState.tr('patientDeleteConfirmBody', {
      name: [deleteTarget?.first_name, deleteTarget?.last_name].filter(Boolean).join(' '),
    })}
  </p>
</Modal>