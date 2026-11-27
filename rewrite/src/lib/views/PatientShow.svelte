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
    ExternalLink,
    FileText,
    AlertTriangle,
  } from 'lucide-svelte';
  import {
    createDocumentScan,
    getDocumentScan,
    getPatient,
    getTreatment,
    listDocumentScans,
    listTreatmentTypeOptions,
    listTreatments,
    openDocumentScanInViewer,
    parseApiError,
    pickDocumentScanFile,
    scanAssetUrl,
    type DocumentScanView,
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

  // Document scans
  let documentScans = $state<DocumentScanView[]>([]);
  let scanAddModalOpen = $state(false);
  let scanTitle = $state('');
  let scanSourcePath = $state<string | null>(null);
  let scanSaving = $state(false);
  let scanFieldError = $state<string | null>(null);

  let scanDetailModalOpen = $state(false);
  let scanDetailLoading = $state(false);
  let scanDetail = $state<DocumentScanView | null>(null);
  let scanPdfBlobUrl = $state<string | null>(null);
  let scanOpeningViewer = $state(false);
  let scanDetailRequestToken = 0;

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
      scanAddModalOpen = false;
      scanDetailModalOpen = false;
      scanDetailRequestToken += 1;
      scanTitle = '';
      scanSourcePath = null;
      scanFieldError = null;
      revokePdfBlobUrl();
    });

    Promise.all([
      getPatient(currentId),
      listTreatmentTypeOptions(),
      listTreatments({ page: 1, per_page: 8, patient_id: currentId }),
      listDocumentScans(currentId),
    ])
      .then(([p, types, list, scans]) => {
        if (cancelled) return;
        patient = p;
        treatmentTypes = types;
        treatmentsListing = list;
        documentScans = scans;
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

  function revokePdfBlobUrl() {
    if (scanPdfBlobUrl) {
      URL.revokeObjectURL(scanPdfBlobUrl);
      scanPdfBlobUrl = null;
    }
  }

  function openScanAddModal() {
    scanTitle = '';
    scanSourcePath = null;
    scanFieldError = null;
    scanAddModalOpen = true;
  }

  async function handlePickScanFile() {
    try {
      const selected = await pickDocumentScanFile();
      if (selected) {
        scanSourcePath = selected;
        scanFieldError = null;
      }
    } catch (err) {
      appState.addFlash('error', parseApiError(err, appState.locale).message);
    }
  }

  async function handleCreateScan(e: SubmitEvent) {
    e.preventDefault();
    if (!patient) return;
    if (!scanSourcePath) {
      scanFieldError = appState.tr('documentScanFileRequired');
      return;
    }

    scanSaving = true;
    scanFieldError = null;
    try {
      await createDocumentScan({
        patient_id: patient.id,
        title: scanTitle.trim() ? scanTitle.trim() : null,
        source_path: scanSourcePath,
      });
      scanAddModalOpen = false;
      appState.addFlash('success', appState.tr('documentScanCreatedSuccess'));
      documentScans = await listDocumentScans(patient.id);
    } catch (err) {
      const parsed = parseApiError(err, appState.locale);
      scanFieldError = parsed.message;
    } finally {
      scanSaving = false;
    }
  }

  async function openScanDetail(scanId: number) {
    const token = ++scanDetailRequestToken;
    revokePdfBlobUrl();
    scanDetailModalOpen = true;
    scanDetailLoading = true;
    scanDetail = null;
    try {
      const detail = await getDocumentScan(scanId);
      if (token !== scanDetailRequestToken) return;
      scanDetail = detail;
      if (detail.is_pdf && detail.file_exists && detail.file_path) {
        try {
          const resp = await fetch(scanAssetUrl(detail.file_path));
          if (token !== scanDetailRequestToken) return;
          if (resp.ok) {
            const buf = await resp.arrayBuffer();
            if (token !== scanDetailRequestToken) return;
            const blob = new Blob([buf], { type: 'application/pdf' });
            scanPdfBlobUrl = URL.createObjectURL(blob);
          }
        } catch {
          // Fall back to direct asset URL in iframe if fetch fails
        }
      }
    } catch (err) {
      if (token !== scanDetailRequestToken) return;
      scanDetailModalOpen = false;
      appState.addFlash('error', parseApiError(err, appState.locale).message);
    } finally {
      if (token === scanDetailRequestToken) {
        scanDetailLoading = false;
      }
    }
  }

  function closeScanDetailModal() {
    scanDetailRequestToken += 1;
    scanDetailModalOpen = false;
    revokePdfBlobUrl();
  }

  async function handleOpenInViewer() {
    if (!scanDetail) return;
    scanOpeningViewer = true;
    try {
      await openDocumentScanInViewer(scanDetail.id);
    } catch (err) {
      appState.addFlash('error', parseApiError(err, appState.locale).message);
    } finally {
      scanOpeningViewer = false;
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
              onclick={openScanAddModal}
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

        <div class="pt-4 border-t border-base-200 space-y-3">
          <h2 class="text-lg font-semibold">{appState.tr('documentScansTitle')}</h2>

          {#if documentScans.length === 0}
            <p class="text-sm text-base-content/60 py-2">{appState.tr('documentScanNoRecords')}</p>
          {:else}
            <div class="overflow-x-auto">
              <table class="table table-zebra table-sm border border-base-200">
                <thead>
                  <tr>
                    <th class="w-28"></th>
                    <th>{appState.tr('documentScanColumn')}</th>
                    <th class="w-52">{appState.tr('documentScanDate')}</th>
                  </tr>
                </thead>
                <tbody>
                  {#each documentScans as scan (scan.id)}
                    <tr>
                      <td>
                        <button
                          type="button"
                          class="btn btn-xs btn-outline"
                          onclick={() => openScanDetail(scan.id)}
                        >
                          <Eye class="w-3.5 h-3.5" />
                          {appState.tr('detail')}
                        </button>
                      </td>
                      <td>
                        <div class="flex items-center gap-3">
                          {#if scan.thumb_exists && scan.thumb_path}
                            <img
                              src={scanAssetUrl(scan.thumb_path)}
                              alt={scan.title ?? scan.file ?? 'Document scan thumbnail'}
                              class="w-12 h-12 object-contain rounded border border-base-300 bg-base-100 shrink-0"
                            />
                          {:else if scan.is_pdf && scan.file_exists}
                            <div
                              class="w-12 h-12 rounded border border-base-300 bg-base-200/70 flex items-center justify-center shrink-0"
                            >
                              <FileText class="w-5 h-5 text-base-content/70" />
                            </div>
                          {/if}
                          <div class="min-w-0">
                            <div class="font-medium truncate">
                              {scan.title?.trim() ? scan.title : scan.file ?? `#${scan.id}`}
                            </div>
                            {#if scan.file}
                              <div class="text-xs text-base-content/60 font-mono truncate">
                                {scan.file}
                              </div>
                            {/if}
                          </div>
                        </div>
                      </td>
                      <td class="whitespace-nowrap">{scan.created_at ?? ''}</td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          {/if}
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

<!-- Add Document Scan Modal -->
<Modal
  open={scanAddModalOpen}
  title={appState.tr('documentScanNewTitle')}
  boxClass="max-w-lg"
  loading={scanSaving}
  hideActions
  onCancel={() => {
    if (!scanSaving) scanAddModalOpen = false;
  }}
>
  {#if scanAddModalOpen && patient}
    <form onsubmit={handleCreateScan} class="space-y-4">
      <div class="text-sm">
        <span class="font-medium text-base-content/70">{appState.tr('documentScanPatient')}:</span>
        <span class="ml-1 font-semibold">{patientDisplayName}</span>
      </div>

      <label class="form-control w-full">
        <div class="label">
          <span class="label-text font-medium">{appState.tr('documentScanTitle')}</span>
        </div>
        <input
          type="text"
          class="input input-bordered input-sm w-full"
          maxlength={255}
          bind:value={scanTitle}
          disabled={scanSaving}
        />
      </label>

      <div class="space-y-2">
        <div class="label-text font-medium">{appState.tr('documentScanFile')}</div>
        <button
          type="button"
          class="btn btn-outline btn-sm"
          disabled={scanSaving}
          onclick={handlePickScanFile}
        >
          <FileUp class="w-4 h-4" />
          {appState.tr('documentScanChooseFile')}
        </button>

        {#if scanSourcePath}
          <div class="bg-base-200/70 rounded-lg p-2.5 text-xs">
            <span class="font-semibold">{appState.tr('documentScanSelectedFile')}:</span>
            <span class="font-mono break-all ml-1">{scanSourcePath}</span>
          </div>
        {/if}

        {#if scanFieldError}
          <p class="text-error text-xs">{scanFieldError}</p>
        {/if}
      </div>

      <div class="modal-action pt-2">
        <button
          type="button"
          class="btn btn-ghost btn-sm"
          disabled={scanSaving}
          onclick={() => (scanAddModalOpen = false)}
        >
          {appState.tr('close')}
        </button>
        <button type="submit" class="btn btn-primary btn-sm" disabled={scanSaving}>
          {#if scanSaving}
            <span class="loading loading-spinner loading-xs"></span>
          {/if}
          {appState.tr('documentScanAddBtn')}
        </button>
      </div>
    </form>
  {/if}
</Modal>

<!-- Document Scan Preview Modal -->
<Modal
  open={scanDetailModalOpen}
  title={scanDetail?.title?.trim() || scanDetail?.file || appState.tr('documentScanDetailTitle')}
  boxClass="max-w-4xl"
  hideActions={!scanDetail}
  onCancel={closeScanDetailModal}
>
  {#if scanDetailLoading}
    <div class="flex justify-center py-8">
      <span class="loading loading-spinner loading-md text-primary"></span>
    </div>
  {:else if scanDetail}
    <div class="space-y-3">
      <div class="flex flex-wrap items-center justify-between gap-2 text-xs text-base-content/70">
        <span class="font-mono">{scanDetail.file ?? ''}</span>
        <span>{scanDetail.created_at ?? ''}</span>
      </div>

      {#if !scanDetail.file_exists || !scanDetail.file_path}
        <div class="alert alert-warning text-sm">
          <AlertTriangle class="w-5 h-5 shrink-0" />
          <span>{appState.tr('documentScanMissingFile')}</span>
        </div>
      {:else if scanDetail.is_pdf}
        <iframe
          title={scanDetail.title ?? scanDetail.file ?? 'PDF preview'}
          src={scanPdfBlobUrl ?? scanAssetUrl(scanDetail.file_path)}
          class="w-full h-[65vh] rounded-lg border border-base-300 bg-base-200"
        ></iframe>
      {:else}
        <div class="flex justify-center bg-base-200/50 rounded-lg border border-base-300 p-2">
          <img
            src={scanAssetUrl(scanDetail.file_path)}
            alt={scanDetail.title ?? scanDetail.file ?? 'Document scan'}
            class="max-h-[65vh] object-contain rounded"
          />
        </div>
      {/if}
    </div>
  {/if}

  {#snippet footer()}
    {#if scanDetail && scanDetail.file_exists}
      <button
        type="button"
        class="btn btn-outline btn-sm"
        disabled={scanOpeningViewer}
        onclick={handleOpenInViewer}
      >
        {#if scanOpeningViewer}
          <span class="loading loading-spinner loading-xs"></span>
        {:else}
          <ExternalLink class="w-3.5 h-3.5" />
        {/if}
        {appState.tr('documentScanOpenViewer')}
      </button>
    {/if}
    <button type="button" class="btn btn-ghost btn-sm" onclick={closeScanDetailModal}>
      {appState.tr('close')}
    </button>
  {/snippet}
</Modal>