<script lang="ts">
  import { Save } from 'lucide-svelte';
  import {
    createTreatment,
    getTreatment,
    listTreatmentTypeOptions,
    parseApiError,
    priceFinder,
    updateTreatment,
    type MedicalTreatmentType,
    type MedicalTreatmentView,
  } from '../api';
  import { locationOptions } from '../i18n';
  import { appState } from '../state.svelte';
  import PatientAutocomplete from './PatientAutocomplete.svelte';

  interface Props {
    treatmentId?: number;
    fixedPatientId?: number;
    fixedPatientLabel?: string;
    initialPatientId?: number | null;
    cancelLabel?: string;
    onSaved: (treatment: MedicalTreatmentView) => void;
    onCancel: () => void;
  }

  let {
    treatmentId,
    fixedPatientId,
    fixedPatientLabel,
    initialPatientId = null,
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
  let treatmentTypes = $state<MedicalTreatmentType[]>([]);

  let patientId = $state<number | null>(null);
  let date = $state(todayLocalDate());
  let locationId = $state<number | null>(1);
  let medicalTreatmentTypeId = $state<number | null>(null);
  let price = $state<number | null>(null);
  let note = $state('');

  let locOptions = $derived(locationOptions(appState.locale));

  $effect(() => {
    const activeUserId = appState.currentUser?.id;
    const currentTreatmentId = treatmentId;
    const lockedPatientId = fixedPatientId;
    const seedPatientId = initialPatientId;
    if (!appState.isInitialized || !activeUserId) return;

    let cancelled = false;
    loading = true;
    fieldErrors = {};

    Promise.all([
      listTreatmentTypeOptions(),
      currentTreatmentId ? getTreatment(currentTreatmentId) : Promise.resolve(null),
    ])
      .then(([types, detail]) => {
        if (cancelled) return;
        treatmentTypes = types;
        if (detail) {
          const tr = detail.treatment;
          patientId = lockedPatientId ?? tr.patient_id ?? null;
          date = tr.date ? tr.date.slice(0, 10) : todayLocalDate();
          locationId = tr.location_id ?? null;
          medicalTreatmentTypeId = tr.medical_treatment_type_id ?? null;
          price = tr.price ?? null;
          note = tr.note ?? '';
        } else {
          patientId = lockedPatientId ?? seedPatientId ?? null;
          date = todayLocalDate();
          locationId = 1;
          medicalTreatmentTypeId = null;
          price = null;
          note = '';
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

  async function handleTypeChange() {
    if (!medicalTreatmentTypeId) {
      price = null;
      return;
    }
    try {
      const foundPrice = await priceFinder(medicalTreatmentTypeId);
      if (foundPrice !== null && foundPrice !== undefined) {
        price = foundPrice;
      }
    } catch {
      const fallback = treatmentTypes.find((t) => t.id === medicalTreatmentTypeId);
      if (fallback && fallback.price !== null) {
        price = fallback.price;
      }
    }
  }

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    saving = true;
    fieldErrors = {};

    const parsedPrice = price === null || (price as unknown) === '' ? null : Number(price);
    if (parsedPrice !== null && (!Number.isFinite(parsedPrice) || parsedPrice < 0)) {
      fieldErrors = { price: appState.tr('err_invalid_price') };
      saving = false;
      return;
    }

    const payload = {
      patient_id: fixedPatientId ?? patientId,
      date,
      location_id: locationId,
      medical_treatment_type_id: medicalTreatmentTypeId,
      price: parsedPrice,
      note: note.trim() ? note : null,
    };

    try {
      const saved = treatmentId
        ? await updateTreatment(treatmentId, payload)
        : await createTreatment(payload);
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
    {#if fixedPatientId}
      <div class="bg-base-200/60 px-3 py-2 rounded-lg text-sm">
        <span class="font-medium text-base-content/70">{appState.tr('treatmentPatient')}:</span>
        <span class="font-semibold ml-1">{fixedPatientLabel ?? `#${fixedPatientId}`}</span>
      </div>
    {:else}
      <div>
        <label class="label" for="mt-patient">
          <span class="label-text font-medium">{appState.tr('treatmentPatient')} *</span>
        </label>
        <PatientAutocomplete
          id="mt-patient"
          bind:value={patientId}
          error={fieldErrors.patient_id}
        />
        {#if fieldErrors.patient_id}
          <p class="text-error text-xs mt-1">{fieldErrors.patient_id}</p>
        {/if}
      </div>
    {/if}

    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div>
        <label class="label" for="mt-date">
          <span class="label-text font-medium">{appState.tr('treatmentDate')} *</span>
        </label>
        <input
          id="mt-date"
          type="date"
          class={`input input-bordered w-full ${fieldErrors.date ? 'input-error' : ''}`}
          bind:value={date}
        />
        {#if fieldErrors.date}
          <p class="text-error text-xs mt-1">{fieldErrors.date}</p>
        {/if}
      </div>

      <div>
        <label class="label" for="mt-location">
          <span class="label-text font-medium">{appState.tr('treatmentLocation')} *</span>
        </label>
        <select
          id="mt-location"
          class={`select select-bordered w-full ${fieldErrors.location_id ? 'select-error' : ''}`}
          bind:value={locationId}
        >
          <option value={null}>—</option>
          {#each locOptions as opt (opt.value)}
            <option value={opt.value}>{opt.label}</option>
          {/each}
        </select>
        {#if fieldErrors.location_id}
          <p class="text-error text-xs mt-1">{fieldErrors.location_id}</p>
        {/if}
      </div>

      <div>
        <label class="label" for="mt-type">
          <span class="label-text font-medium">{appState.tr('treatmentType')} *</span>
        </label>
        <select
          id="mt-type"
          class={`select select-bordered w-full ${
            fieldErrors.medical_treatment_type_id ? 'select-error' : ''
          }`}
          bind:value={medicalTreatmentTypeId}
          onchange={handleTypeChange}
        >
          <option value={null}>—</option>
          {#each treatmentTypes as tt (tt.id)}
            <option value={tt.id}>{tt.name ?? `#${tt.id}`}</option>
          {/each}
        </select>
        {#if fieldErrors.medical_treatment_type_id}
          <p class="text-error text-xs mt-1">{fieldErrors.medical_treatment_type_id}</p>
        {/if}
      </div>

      <div>
        <label class="label" for="mt-price">
          <span class="label-text font-medium">{appState.tr('treatmentPrice')} *</span>
        </label>
        <input
          id="mt-price"
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
    </div>

    <div>
      <label class="label" for="mt-note">
        <span class="label-text font-medium">{appState.tr('treatmentNote')}</span>
      </label>
      <textarea
        id="mt-note"
        rows="3"
        class={`textarea textarea-bordered w-full ${fieldErrors.note ? 'textarea-error' : ''}`}
        bind:value={note}
      ></textarea>
      {#if fieldErrors.note}
        <p class="text-error text-xs mt-1">{fieldErrors.note}</p>
      {/if}
    </div>

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