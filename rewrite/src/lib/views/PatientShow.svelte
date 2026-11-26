<script lang="ts">
  import { untrack } from 'svelte';
  import {
    ArrowLeft,
    Pencil,
    Plus,
    FileUp,
    Eye,
    Search,
    RotateCcw,
  } from 'lucide-svelte';
  import {
    getPatient,
    getTreatment,
    listTreatmentTypeOptions,
    listTreatments,
    parseApiError,
    type MedicalTreatmentDetail,
    type MedicalTreatmentType,
    type MedicalTreatmentView,
    type PaginatedResult,
    type PatientView,
  } from '../api';
  import Modal from '../components/Modal.svelte';
  import Pagination from '../components/Pagination.svelte';
  import TreatmentForm from '../components/TreatmentForm.svelte';
  import {
    locationLabel,
    locationOptions,
    treatmentStatusLabel,
    treatmentStatusOptions,
  } from '../i18n';
  import { appState } from '../state.svelte';

  interface Props {
    id: number;
  }

  let { id }: Props = $props();

  let patient = $state<PatientView | null>(null);
  let treatmentTypes = $state<MedicalTreatmentType[]>([]);
  let loading = $state(false);

  // Patient-scoped treatments listing (8/page)
  let tPage = $state(1);
  let filterTypeId = $state<number | null>(null);
  let filterDateGteq = $state('');
  let filterDateLteq = $state('');
  let filterLocationId = $state<number | null>(null);
  let filterStatus = $state<number | null>(null);
  let treatmentsListing = $state<PaginatedResult<MedicalTreatmentView> | null>(null);
  let treatmentsLoading = $state(false);

  // Modals
  let addModalOpen = $state(false);
  let detailModalOpen = $state(false);
  let detailLoading = $state(false);
  let detailData = $state<MedicalTreatmentDetail | null>(null);

  let locOptions = $derived(locationOptions(appState.locale));
  let statusOptions = $derived(treatmentStatusOptions(appState.locale));
  let patientDisplayName = $derived(
    patient
      ? [patient.first_name, patient.last_name].filter(Boolean).join(' ').trim() || `#${patient.id}`
      : '',
  );

  async function loadPatientTreatments(patientId: number, targetPage = tPage) {
    treatmentsLoading = true;
    try {
      treatmentsListing = await listTreatments({
        page: targetPage,
        per_page: 8,
        patient_id: patientId,
        medical_treatment_type_id: filterTypeId,
        date_gteq: filterDateGteq || undefined,
        date_lteq: filterDateLteq || undefined,
        location_id: filterLocationId,
        status: filterStatus,
      });
      tPage = treatmentsListing.page;
    } catch (err) {
      appState.addFlash('error', parseApiError(err, appState.locale).message);
    } finally {
      treatmentsLoading = false;
    }
  }

  $effect(() => {
    const activeUserId = appState.currentUser?.id;
    const currentId = id;
    if (!appState.isInitialized || !activeUserId || !currentId) return;

    let cancelled = false;
    loading = true;
    untrack(() => {
      tPage = 1;
      filterTypeId = null;
      filterDateGteq = '';
      filterDateLteq = '';
      filterLocationId = null;
      filterStatus = null;
      addModalOpen = false;
      detailModalOpen = false;
    });

    Promise.all([
      getPatient(currentId),
      listTreatmentTypeOptions(),
      listTreatments({ page: 1, per_page: 8, patient_id: currentId }),
    ])
      .then(([p, types, list]) => {
        if (cancelled) return;
        patient = p;
        treatmentTypes = types;
        treatmentsListing = list;
      })
      .catch((err) => {
        if (cancelled) return;
        appState.addFlash('error', parseApiError(err, appState.locale).message);
        appState.navigate('/patients');
      })
      .finally(() => {
        if (!cancelled) loading = false;
      });

    return () => {
      cancelled = true;
    };
  });

  function handleTreatmentFilterSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (!patient) return;
    tPage = 1;
    loadPatientTreatments(patient.id, 1);
  }

  function handleTreatmentFilterReset() {
    if (!patient) return;
    filterTypeId = null;
    filterDateGteq = '';
    filterDateLteq = '';
    filterLocationId = null;
    filterStatus = null;
    tPage = 1;
    loadPatientTreatments(patient.id, 1);
  }

  async function openTreatmentDetail(treatmentId: number) {
    detailModalOpen = true;
    detailLoading = true;
    detailData = null;
    try {
      detailData = await getTreatment(treatmentId);
    } catch (err) {
      detailModalOpen = false;
      appState.addFlash('error', parseApiError(err, appState.locale).message);
    } finally {
      detailLoading = false;
    }
  }

  async function handleTreatmentSaved() {
    addModalOpen = false;
    appState.addFlash('success', appState.tr('treatmentCreatedSuccess'));
    if (patient) {
      tPage = 1;
      await Promise.all([loadPatientTreatments(patient.id, 1), getPatient(patient.id).then((p) => (patient = p))]);
    }
  }
</script>

<div class="max-w-7xl mx-auto px-4 py-6 space-y-6">
  <div class="card bg-base-100 border border-base-300 shadow-xs">
    <div class="card-body">
      <div class="flex flex-wrap items-center justify-between gap-3 border-b border-base-200 pb-3 mb-4">
        <h1 class="text-xl font-bold">{appState.tr('patientDetailTitle')}</h1>
        <div class="flex flex-wrap items-center gap-2">
          <button
            type="button"
            class="btn btn-sm btn-ghost"
            onclick={() => appState.navigate('/patients')}
          >
            <ArrowLeft class="w-4 h-4" />
            {appState.tr('back')}
          </button>
          {#if patient}
            <button
              type="button"
              class="btn btn-sm btn-outline"
              onclick={() => appState.navigate(`/patients/${patient!.id}/edit`)}
            >
              <Pencil class="w-4 h-4" />
              {appState.tr('edit')}
            </button>
            <button
              type="button"
              class="btn btn-sm btn-primary"
              onclick={() => (addModalOpen = true)}
            >
              <Plus class="w-4 h-4" />
              {appState.tr('patientAddTreatment')}
            </button>
            <button
              type="button"
              class="btn btn-sm btn-outline"
              data-hook="add-document-scan"
              onclick={() => appState.addFlash('info', appState.tr('patientDocumentScanDeferred'))}
            >
              <FileUp class="w-4 h-4" />
              {appState.tr('patientAddDocumentScan')}
            </button>
          {/if}
        </div>
      </div>

      {#if loading}
        <div class="flex justify-center py-8">
          <span class="loading loading-spinner loading-md text-primary"></span>
        </div>
      {:else if patient}
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
          <div class="overflow-x-auto">
            <table class="table table-bordered table-sm">
              <tbody>
                <tr>
                  <th class="w-48 bg-base-200/50">{appState.tr('patientFirstName')}</th>
                  <td>{patient.first_name ?? ''}</td>
                </tr>
                <tr>
                  <th class="bg-base-200/50">{appState.tr('patientLastName')}</th>
                  <td>{patient.last_name ?? ''}</td>
                </tr>
                <tr>
                  <th class="bg-base-200/50">{appState.tr('patientBirthdate')}</th>
                  <td>{patient.birthdate ?? ''}</td>
                </tr>
                <tr>
                  <th class="bg-base-200/50">{appState.tr('patientProfession')}</th>
                  <td>{patient.profession ?? ''}</td>
                </tr>
                <tr>
                  <th class="bg-base-200/50">{appState.tr('patientPhone')}</th>
                  <td>{patient.phone ?? ''}</td>
                </tr>
                <tr>
                  <th class="bg-base-200/50">{appState.tr('patientMobilePhone')}</th>
                  <td>{patient.mobile_phone ?? ''}</td>
                </tr>
                <tr>
                  <th class="bg-base-200/50">{appState.tr('patientEmail')}</th>
                  <td>{patient.email ?? ''}</td>
                </tr>
                <tr>
                  <th class="bg-base-200/50">{appState.tr('patientAddress')}</th>
                  <td>{patient.address ?? ''}</td>
                </tr>
                <tr>
                  <th class="bg-base-200/50">{appState.tr('patientPostalCode')}</th>
                  <td>{patient.postal_code ?? ''}</td>
                </tr>
                <tr>
                  <th class="bg-base-200/50">{appState.tr('patientCity')}</th>
                  <td>{patient.city ?? ''}</td>
                </tr>
                <tr>
                  <th class="bg-base-200/50">{appState.tr('patientCountry')}</th>
                  <td>{patient.country_name ?? ''}</td>
                </tr>
              </tbody>
            </table>
          </div>

          <div class="overflow-x-auto">
            <table class="table table-bordered table-sm">
              <tbody>
                <tr>
                  <th class="w-56 bg-base-200/50">
                    {appState.tr('patientNationalInsuranceNumber')}
                  </th>
                  <td>{patient.national_insurance_number ?? ''}</td>
                </tr>
                <tr>
                  <th class="bg-base-200/50">{appState.tr('patientFamilyDoctor')}</th>
                  <td>{patient.family_doctor_label ?? ''}</td>
                </tr>
                <tr>
                  <th class="bg-base-200/50">{appState.tr('patientPhysicalTherapist')}</th>
                  <td>{patient.physical_therapists_label ?? ''}</td>
                </tr>
                <tr>
                  <th class="bg-base-200/50">{appState.tr('patientOsteopath')}</th>
                  <td>{patient.osteopath_label ?? ''}</td>
                </tr>
                <tr>
                  <th class="bg-base-200/50">{appState.tr('patientLegalGuardian')}</th>
                  <td>{patient.legal_guardian_id ?? ''}</td>
                </tr>
                <tr>
                  <th class="bg-base-200/50">{appState.tr('patientNote')}</th>
                  <td class="whitespace-pre-wrap">{patient.note ?? ''}</td>
                </tr>
                <tr>
                  <th class="bg-base-200/50">{appState.tr('patientDiabetic')}</th>
                  <td>{patient.diabetic ? appState.tr('yes') : appState.tr('no')}</td>
                </tr>
                <tr>
                  <th class="bg-base-200/50">{appState.tr('patientUniversalHealthcare')}</th>
                  <td>
                    {patient.universal_healthcare_coverage ? appState.tr('yes') : appState.tr('no')}
                  </td>
                </tr>
                <tr>
                  <th class="bg-base-200/50">{appState.tr('patientLongDurationDisease')}</th>
                  <td>
                    {patient.long_duration_disease ? appState.tr('yes') : appState.tr('no')}
                  </td>
                </tr>
                <tr>
                  <th class="bg-base-200/50">{appState.tr('patientDiabeteNetwork')}</th>
                  <td>{patient.diabete_network_id ?? ''}</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <div class="pt-4 border-t border-base-200 space-y-3">
          <h2 class="text-lg font-semibold">{appState.tr('treatmentsTitle')}</h2>

          {#if treatmentsListing}
            <Pagination
              page={treatmentsListing.page}
              perPage={treatmentsListing.per_page}
              filteredTotal={treatmentsListing.filtered_total}
              total={treatmentsListing.total}
              label={appState.tr('treatmentPlural')}
              onPageChange={(nextPage) => {
                tPage = nextPage;
                loadPatientTreatments(patient!.id, nextPage);
              }}
            />
          {/if}

          <form onsubmit={handleTreatmentFilterSubmit} class="overflow-x-auto">
            <table class="table table-zebra table-sm border border-base-200">
              <thead>
                <tr>
                  <th></th>
                  <th>{appState.tr('treatmentType')}</th>
                  <th>{appState.tr('treatmentDate')}</th>
                  <th>{appState.tr('treatmentLocation')}</th>
                  <th>{appState.tr('treatmentStatus')}</th>
                  <th>{appState.tr('actions')}</th>
                </tr>
                <tr class="bg-base-200/60">
                  <td></td>
                  <td>
                    <select
                      class="select select-bordered select-xs w-full min-w-36"
                      aria-label={appState.tr('treatmentType')}
                      bind:value={filterTypeId}
                    >
                      <option value={null}>{appState.tr('treatmentAllTypes')}</option>
                      {#each treatmentTypes as tt (tt.id)}
                        <option value={tt.id}>{tt.name ?? `#${tt.id}`}</option>
                      {/each}
                    </select>
                  </td>
                  <td>
                    <div class="flex items-center gap-1 min-w-64">
                      <input
                        type="date"
                        class="input input-bordered input-xs w-full"
                        aria-label="Date from"
                        bind:value={filterDateGteq}
                      />
                      <span class="text-xs text-base-content/60">{appState.tr('toRange')}</span>
                      <input
                        type="date"
                        class="input input-bordered input-xs w-full"
                        aria-label="Date to"
                        bind:value={filterDateLteq}
                      />
                    </div>
                  </td>
                  <td>
                    <select
                      class="select select-bordered select-xs w-full min-w-28"
                      aria-label={appState.tr('treatmentLocation')}
                      bind:value={filterLocationId}
                    >
                      <option value={null}>{appState.tr('treatmentAllLocations')}</option>
                      {#each locOptions as opt (opt.value)}
                        <option value={opt.value}>{opt.label}</option>
                      {/each}
                    </select>
                  </td>
                  <td>
                    <select
                      class="select select-bordered select-xs w-full min-w-28"
                      aria-label={appState.tr('treatmentStatus')}
                      bind:value={filterStatus}
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
                        onclick={handleTreatmentFilterReset}
                      >
                        <RotateCcw class="w-3.5 h-3.5" />
                        {appState.tr('reset')}
                      </button>
                    </div>
                  </td>
                </tr>
              </thead>
              <tbody>
                {#if treatmentsLoading && !treatmentsListing}
                  <tr>
                    <td colspan="6" class="text-center py-6">
                      <span class="loading loading-spinner loading-md text-primary"></span>
                    </td>
                  </tr>
                {:else if treatmentsListing && treatmentsListing.items.length === 0}
                  <tr>
                    <td colspan="6" class="text-center py-6 text-base-content/60">
                      {appState.tr('noRecords')}
                    </td>
                  </tr>
                {:else if treatmentsListing}
                  {#each treatmentsListing.items as tr (tr.id)}
                    <tr>
                      <td>
                        <button
                          type="button"
                          class="btn btn-xs btn-outline"
                          onclick={() => openTreatmentDetail(tr.id)}
                        >
                          <Eye class="w-3.5 h-3.5" />
                          {appState.tr('detail')}
                        </button>
                      </td>
                      <td>{tr.treatment_type_name ?? '—'}</td>
                      <td>{tr.date ?? ''}</td>
                      <td>{locationLabel(appState.locale, tr.location_id)}</td>
                      <td>
                        <span
                          class={`badge badge-xs ${
                            tr.status === 1 ? 'badge-success' : 'badge-warning'
                          }`}
                        >
                          {treatmentStatusLabel(appState.locale, tr.status)}
                        </span>
                      </td>
                      <td></td>
                    </tr>
                  {/each}
                {/if}
              </tbody>
            </table>
          </form>
        </div>
      {/if}
    </div>
  </div>
</div>

<!-- Add Medical Treatment Modal -->
<Modal
  open={addModalOpen}
  title={appState.tr('treatmentNewTitle')}
  boxClass="max-w-2xl"
  hideActions
  onCancel={() => (addModalOpen = false)}
>
  {#if addModalOpen && patient}
    <TreatmentForm
      fixedPatientId={patient.id}
      fixedPatientLabel={patientDisplayName}
      onSaved={handleTreatmentSaved}
      onCancel={() => (addModalOpen = false)}
    />
  {/if}
</Modal>

<!-- Treatment Detail Modal -->
<Modal
  open={detailModalOpen}
  title={appState.tr('treatmentDetailTitle')}
  boxClass="max-w-xl"
  hideActions={!detailData}
  onCancel={() => (detailModalOpen = false)}
>
  {#if detailLoading}
    <div class="flex justify-center py-8">
      <span class="loading loading-spinner loading-md text-primary"></span>
    </div>
  {:else if detailData}
    {@const tr = detailData.treatment}
    <ul class="divide-y divide-base-200 border border-base-200 rounded-lg text-sm">
      <li class="px-3 py-2 flex justify-between gap-2">
        <span class="font-medium text-base-content/70">{appState.tr('treatmentPatient')}</span>
        <span>
          {[tr.patient_first_name, tr.patient_last_name].filter(Boolean).join(' ')}
        </span>
      </li>
      <li class="px-3 py-2 flex justify-between gap-2">
        <span class="font-medium text-base-content/70">{appState.tr('treatmentType')}</span>
        <span>
          {tr.treatment_type_name ?? '—'}
          {#if tr.treatment_type_price !== null && tr.treatment_type_price !== undefined}
            ({tr.treatment_type_price.toFixed(2)})
          {/if}
        </span>
      </li>
      <li class="px-3 py-2 flex justify-between gap-2">
        <span class="font-medium text-base-content/70">{appState.tr('treatmentDate')}</span>
        <span>{tr.date ?? ''}</span>
      </li>
      <li class="px-3 py-2 flex justify-between gap-2">
        <span class="font-medium text-base-content/70">{appState.tr('treatmentLocation')}</span>
        <span>{locationLabel(appState.locale, tr.location_id)}</span>
      </li>
      <li class="px-3 py-2 flex justify-between gap-2">
        <span class="font-medium text-base-content/70">{appState.tr('treatmentPrice')}</span>
        <span>{tr.price !== null && tr.price !== undefined ? tr.price.toFixed(2) : '—'}</span>
      </li>
      <li class="px-3 py-2 flex justify-between gap-2">
        <span class="font-medium text-base-content/70">{appState.tr('treatmentStatus')}</span>
        <span class={`badge badge-xs ${tr.status === 1 ? 'badge-success' : 'badge-warning'}`}>
          {treatmentStatusLabel(appState.locale, tr.status)}
        </span>
      </li>
      <li class="px-3 py-2">
        <div class="font-medium text-base-content/70 mb-1">{appState.tr('treatmentNote')}</div>
        <div class="whitespace-pre-wrap">{tr.note ?? ''}</div>
      </li>
    </ul>

    {#if detailData.payments.length > 0}
      <div class="mt-4">
        <h4 class="font-semibold text-sm mb-2">{appState.tr('treatmentPaymentsHeading')}</h4>
        <table class="table table-xs table-bordered">
          <thead>
            <tr>
              <th>{appState.tr('treatmentPaymentPaidAt')}</th>
              <th>{appState.tr('treatmentPaymentAmount')}</th>
            </tr>
          </thead>
          <tbody>
            {#each detailData.payments as pay (pay.id)}
              <tr>
                <td>{pay.paid_at ?? ''}</td>
                <td>{pay.amount !== null && pay.amount !== undefined ? pay.amount.toFixed(2) : '—'}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  {/if}

  {#snippet footer()}
    {#if detailData}
      {@const tr = detailData.treatment}
      <button
        type="button"
        class="btn btn-outline btn-sm"
        onclick={() => {
          detailModalOpen = false;
          appState.navigate(`/medical-treatments/${tr.id}/edit`);
        }}
      >
        <Pencil class="w-3.5 h-3.5" />
        {appState.tr('edit')}
      </button>
    {/if}
    <button
      type="button"
      class="btn btn-ghost btn-sm"
      onclick={() => (detailModalOpen = false)}
    >
      {appState.tr('close')}
    </button>
  {/snippet}
</Modal>