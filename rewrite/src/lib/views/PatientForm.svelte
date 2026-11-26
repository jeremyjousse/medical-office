<script lang="ts">
  import { untrack } from 'svelte';
  import { ArrowLeft, Save } from 'lucide-svelte';
  import {
    createPatient,
    getPatient,
    listCountries,
    listPractitionerOptions,
    parseApiError,
    updatePatient,
    type Country,
    type PractitionerOption,
  } from '../api';
  import SearchSelect from '../components/SearchSelect.svelte';
  import { appState } from '../state.svelte';

  interface Props {
    mode: 'new' | 'edit';
    id?: number;
  }

  let { mode, id }: Props = $props();

  let loading = $state(false);
  let saving = $state(false);
  let fieldErrors = $state<Record<string, string>>({});
  let countries = $state<Country[]>([]);
  let practitioners = $state<PractitionerOption[]>([]);

  let firstName = $state('');
  let lastName = $state('');
  let birthdate = $state('');
  let profession = $state('');
  let phone = $state('');
  let mobilePhone = $state('');
  let email = $state('');
  let address = $state('');
  let postalCode = $state('');
  let city = $state('');
  let countryId = $state<number | null>(null);

  let nationalInsuranceNumber = $state('');
  let familyDoctorId = $state<number | null>(null);
  let physicalTherapistsId = $state<number | null>(null);
  let osteopathId = $state<number | null>(null);
  let legalGuardianId = $state<number | null>(null);
  let note = $state('');
  let diabetic = $state(false);
  let universalHealthcareCoverage = $state(false);
  let longDurationDisease = $state(false);
  let diabeteNetworkId = $state<number | null>(null);

  let countryOptions = $derived(
    countries.map((c) => ({
      value: c.id,
      label: c.iso ? `${c.name ?? ''} (${c.iso})` : (c.name ?? String(c.id)),
    })),
  );

  function toPractitionerOptions(specialityId: number) {
    return practitioners
      .filter((p) => p.speciality_id === specialityId)
      .map((p) => {
        const name = [p.last_name, p.first_name].filter(Boolean).join(' ').trim();
        return {
          value: p.id,
          label: p.city ? `${name} (${p.city})` : name || `#${p.id}`,
        };
      });
  }

  let familyDoctorOptions = $derived(toPractitionerOptions(1));
  let physicalTherapistOptions = $derived(toPractitionerOptions(2));
  let osteopathOptions = $derived(toPractitionerOptions(3));

  function resetForm() {
    fieldErrors = {};
    firstName = '';
    lastName = '';
    birthdate = '';
    profession = '';
    phone = '';
    mobilePhone = '';
    email = '';
    address = '';
    postalCode = '';
    city = '';
    countryId = null;
    nationalInsuranceNumber = '';
    familyDoctorId = null;
    physicalTherapistsId = null;
    osteopathId = null;
    legalGuardianId = null;
    note = '';
    diabetic = false;
    universalHealthcareCoverage = false;
    longDurationDisease = false;
    diabeteNetworkId = null;
  }

  $effect(() => {
    const activeUserId = appState.currentUser?.id;
    const currentMode = mode;
    const currentId = id;
    if (!appState.isInitialized || !activeUserId) return;

    let cancelled = false;
    loading = true;
    if (currentMode === 'new') {
      untrack(() => resetForm());
    } else {
      fieldErrors = {};
    }

    Promise.all([
      listCountries(),
      listPractitionerOptions(),
      currentMode === 'edit' && currentId ? getPatient(currentId) : Promise.resolve(null),
    ])
      .then(([allCountries, allPractitioners, item]) => {
        if (cancelled) return;
        countries = allCountries;
        practitioners = allPractitioners;
        if (item) {
          firstName = item.first_name ?? '';
          lastName = item.last_name ?? '';
          birthdate = item.birthdate ? item.birthdate.slice(0, 10) : '';
          profession = item.profession ?? '';
          phone = item.phone ?? '';
          mobilePhone = item.mobile_phone ?? '';
          email = item.email ?? '';
          address = item.address ?? '';
          postalCode = item.postal_code ?? '';
          city = item.city ?? '';
          countryId = item.country_id ?? null;
          nationalInsuranceNumber = item.national_insurance_number ?? '';
          familyDoctorId = item.family_doctor_id ?? null;
          physicalTherapistsId = item.physical_therapists_id ?? null;
          osteopathId = item.osteopath_id ?? null;
          legalGuardianId = item.legal_guardian_id ?? null;
          note = item.note ?? '';
          diabetic = Boolean(item.diabetic);
          universalHealthcareCoverage = Boolean(item.universal_healthcare_coverage);
          longDurationDisease = Boolean(item.long_duration_disease);
          diabeteNetworkId = item.diabete_network_id ?? null;
        }
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

  function parseOptionalInt(
    val: number | string | null,
    field: string,
    errors: Record<string, string>,
  ): number | null {
    if (val === null || val === undefined || val === '') return null;
    const num = Number(val);
    if (!Number.isInteger(num) || num < 0) {
      errors[field] = appState.tr('err_invalid_integer');
      return null;
    }
    return num;
  }

  async function handleSave(e: SubmitEvent) {
    e.preventDefault();
    saving = true;
    fieldErrors = {};

    const intErrors: Record<string, string> = {};
    const parsedGuardianId = parseOptionalInt(legalGuardianId, 'legal_guardian_id', intErrors);
    const parsedDiabeteNetworkId = parseOptionalInt(
      diabeteNetworkId,
      'diabete_network_id',
      intErrors,
    );
    if (Object.keys(intErrors).length > 0) {
      fieldErrors = intErrors;
      saving = false;
      return;
    }

    const payload = {
      first_name: firstName.trim() ? firstName : null,
      last_name: lastName,
      birthdate: birthdate.trim() ? birthdate : null,
      profession: profession.trim() ? profession : null,
      phone: phone.trim() ? phone : null,
      mobile_phone: mobilePhone.trim() ? mobilePhone : null,
      email: email.trim() ? email : null,
      address: address.trim() ? address : null,
      postal_code: postalCode.trim() ? postalCode : null,
      city,
      country_id: countryId,
      national_insurance_number: nationalInsuranceNumber.trim()
        ? nationalInsuranceNumber
        : null,
      family_doctor_id: familyDoctorId,
      physical_therapists_id: physicalTherapistsId,
      osteopath_id: osteopathId,
      legal_guardian_id: parsedGuardianId,
      note: note.trim() ? note : null,
      diabetic,
      universal_healthcare_coverage: universalHealthcareCoverage,
      long_duration_disease: longDurationDisease,
      diabete_network_id: parsedDiabeteNetworkId,
    };

    try {
      if (mode === 'new') {
        const created = await createPatient(payload);
        appState.patientFilters.page = 1;
        appState.addFlash('success', appState.tr('patientCreatedSuccess'));
        appState.navigate(`/patients/${created.id}`);
      } else if (mode === 'edit' && id) {
        const updated = await updatePatient(id, payload);
        appState.addFlash('success', appState.tr('patientUpdatedSuccess'));
        appState.navigate(`/patients/${updated.id}`);
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
  <div class="card bg-base-100 border border-base-300 shadow-xs">
    <div class="card-body">
      <div class="flex items-center justify-between border-b border-base-200 pb-3 mb-4">
        <h1 class="text-xl font-bold">
          {mode === 'new' ? appState.tr('patientNewTitle') : appState.tr('patientEditTitle')}
        </h1>
        <button
          type="button"
          class="btn btn-sm btn-ghost"
          onclick={() => appState.navigate(mode === 'edit' && id ? `/patients/${id}` : '/patients')}
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
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="space-y-3">
              <div>
                <label class="label" for="pat-first-name">
                  <span class="label-text font-medium">{appState.tr('patientFirstName')}</span>
                </label>
                <input
                  id="pat-first-name"
                  type="text"
                  class={`input input-bordered w-full ${fieldErrors.first_name ? 'input-error' : ''}`}
                  bind:value={firstName}
                />
                {#if fieldErrors.first_name}
                  <p class="text-error text-xs mt-1">{fieldErrors.first_name}</p>
                {/if}
              </div>

              <div>
                <label class="label" for="pat-last-name">
                  <span class="label-text font-medium">{appState.tr('patientLastName')} *</span>
                </label>
                <input
                  id="pat-last-name"
                  type="text"
                  class={`input input-bordered w-full ${fieldErrors.last_name ? 'input-error' : ''}`}
                  bind:value={lastName}
                />
                {#if fieldErrors.last_name}
                  <p class="text-error text-xs mt-1">{fieldErrors.last_name}</p>
                {/if}
              </div>

              <div>
                <label class="label" for="pat-birthdate">
                  <span class="label-text font-medium">{appState.tr('patientBirthdate')}</span>
                </label>
                <input
                  id="pat-birthdate"
                  type="date"
                  class={`input input-bordered w-full ${fieldErrors.birthdate ? 'input-error' : ''}`}
                  bind:value={birthdate}
                />
                {#if fieldErrors.birthdate}
                  <p class="text-error text-xs mt-1">{fieldErrors.birthdate}</p>
                {/if}
              </div>

              <div>
                <label class="label" for="pat-profession">
                  <span class="label-text font-medium">{appState.tr('patientProfession')}</span>
                </label>
                <input
                  id="pat-profession"
                  type="text"
                  class={`input input-bordered w-full ${fieldErrors.profession ? 'input-error' : ''}`}
                  bind:value={profession}
                />
                {#if fieldErrors.profession}
                  <p class="text-error text-xs mt-1">{fieldErrors.profession}</p>
                {/if}
              </div>

              <div>
                <label class="label" for="pat-phone">
                  <span class="label-text font-medium">{appState.tr('patientPhone')}</span>
                </label>
                <input
                  id="pat-phone"
                  type="text"
                  class={`input input-bordered w-full ${fieldErrors.phone ? 'input-error' : ''}`}
                  bind:value={phone}
                />
                {#if fieldErrors.phone}
                  <p class="text-error text-xs mt-1">{fieldErrors.phone}</p>
                {/if}
              </div>

              <div>
                <label class="label" for="pat-mobile">
                  <span class="label-text font-medium">{appState.tr('patientMobilePhone')}</span>
                </label>
                <input
                  id="pat-mobile"
                  type="text"
                  class={`input input-bordered w-full ${fieldErrors.mobile_phone ? 'input-error' : ''}`}
                  bind:value={mobilePhone}
                />
                {#if fieldErrors.mobile_phone}
                  <p class="text-error text-xs mt-1">{fieldErrors.mobile_phone}</p>
                {/if}
              </div>

              <div>
                <label class="label" for="pat-email">
                  <span class="label-text font-medium">{appState.tr('patientEmail')}</span>
                </label>
                <input
                  id="pat-email"
                  type="text"
                  class={`input input-bordered w-full ${fieldErrors.email ? 'input-error' : ''}`}
                  bind:value={email}
                />
                {#if fieldErrors.email}
                  <p class="text-error text-xs mt-1">{fieldErrors.email}</p>
                {/if}
              </div>

              <div>
                <label class="label" for="pat-address">
                  <span class="label-text font-medium">{appState.tr('patientAddress')}</span>
                </label>
                <input
                  id="pat-address"
                  type="text"
                  class={`input input-bordered w-full ${fieldErrors.address ? 'input-error' : ''}`}
                  bind:value={address}
                />
                {#if fieldErrors.address}
                  <p class="text-error text-xs mt-1">{fieldErrors.address}</p>
                {/if}
              </div>

              <div>
                <label class="label" for="pat-postal">
                  <span class="label-text font-medium">{appState.tr('patientPostalCode')}</span>
                </label>
                <input
                  id="pat-postal"
                  type="text"
                  class={`input input-bordered w-full ${fieldErrors.postal_code ? 'input-error' : ''}`}
                  bind:value={postalCode}
                />
                {#if fieldErrors.postal_code}
                  <p class="text-error text-xs mt-1">{fieldErrors.postal_code}</p>
                {/if}
              </div>

              <div>
                <label class="label" for="pat-city">
                  <span class="label-text font-medium">{appState.tr('patientCity')} *</span>
                </label>
                <input
                  id="pat-city"
                  type="text"
                  class={`input input-bordered w-full ${fieldErrors.city ? 'input-error' : ''}`}
                  bind:value={city}
                />
                {#if fieldErrors.city}
                  <p class="text-error text-xs mt-1">{fieldErrors.city}</p>
                {/if}
              </div>

              <div>
                <label class="label" for="pat-country">
                  <span class="label-text font-medium">{appState.tr('patientCountry')} *</span>
                </label>
                <SearchSelect
                  id="pat-country"
                  options={countryOptions}
                  bind:value={countryId}
                  error={fieldErrors.country_id}
                />
                {#if fieldErrors.country_id}
                  <p class="text-error text-xs mt-1">{fieldErrors.country_id}</p>
                {/if}
              </div>
            </div>

            <div class="space-y-3">
              <div>
                <label class="label" for="pat-ssn">
                  <span class="label-text font-medium">
                    {appState.tr('patientNationalInsuranceNumber')}
                  </span>
                </label>
                <input
                  id="pat-ssn"
                  type="text"
                  class={`input input-bordered w-full ${
                    fieldErrors.national_insurance_number ? 'input-error' : ''
                  }`}
                  bind:value={nationalInsuranceNumber}
                />
                {#if fieldErrors.national_insurance_number}
                  <p class="text-error text-xs mt-1">{fieldErrors.national_insurance_number}</p>
                {/if}
              </div>

              <div>
                <label class="label" for="pat-family-doctor">
                  <span class="label-text font-medium">{appState.tr('patientFamilyDoctor')}</span>
                </label>
                <SearchSelect
                  id="pat-family-doctor"
                  options={familyDoctorOptions}
                  bind:value={familyDoctorId}
                  error={fieldErrors.family_doctor_id}
                />
                {#if fieldErrors.family_doctor_id}
                  <p class="text-error text-xs mt-1">{fieldErrors.family_doctor_id}</p>
                {/if}
              </div>

              <div>
                <label class="label" for="pat-physio">
                  <span class="label-text font-medium">
                    {appState.tr('patientPhysicalTherapist')}
                  </span>
                </label>
                <SearchSelect
                  id="pat-physio"
                  options={physicalTherapistOptions}
                  bind:value={physicalTherapistsId}
                  error={fieldErrors.physical_therapists_id}
                />
                {#if fieldErrors.physical_therapists_id}
                  <p class="text-error text-xs mt-1">{fieldErrors.physical_therapists_id}</p>
                {/if}
              </div>

              <div>
                <label class="label" for="pat-osteopath">
                  <span class="label-text font-medium">{appState.tr('patientOsteopath')}</span>
                </label>
                <SearchSelect
                  id="pat-osteopath"
                  options={osteopathOptions}
                  bind:value={osteopathId}
                  error={fieldErrors.osteopath_id}
                />
                {#if fieldErrors.osteopath_id}
                  <p class="text-error text-xs mt-1">{fieldErrors.osteopath_id}</p>
                {/if}
              </div>

              <div>
                <label class="label" for="pat-guardian">
                  <span class="label-text font-medium">{appState.tr('patientLegalGuardian')}</span>
                </label>
                <input
                  id="pat-guardian"
                  type="number"
                  step="1"
                  min="0"
                  class={`input input-bordered w-full ${
                    fieldErrors.legal_guardian_id ? 'input-error' : ''
                  }`}
                  bind:value={legalGuardianId}
                />
                {#if fieldErrors.legal_guardian_id}
                  <p class="text-error text-xs mt-1">{fieldErrors.legal_guardian_id}</p>
                {/if}
              </div>

              <div>
                <label class="label" for="pat-note">
                  <span class="label-text font-medium">{appState.tr('patientNote')}</span>
                </label>
                <textarea
                  id="pat-note"
                  rows="3"
                  class={`textarea textarea-bordered w-full ${fieldErrors.note ? 'textarea-error' : ''}`}
                  bind:value={note}
                ></textarea>
                {#if fieldErrors.note}
                  <p class="text-error text-xs mt-1">{fieldErrors.note}</p>
                {/if}
              </div>

              <div class="pt-2 space-y-2">
                <label class="flex items-center gap-3 cursor-pointer">
                  <input type="checkbox" class="checkbox checkbox-sm" bind:checked={diabetic} />
                  <span class="label-text font-medium">{appState.tr('patientDiabetic')}</span>
                </label>

                <label class="flex items-center gap-3 cursor-pointer">
                  <input
                    type="checkbox"
                    class="checkbox checkbox-sm"
                    bind:checked={universalHealthcareCoverage}
                  />
                  <span class="label-text font-medium">
                    {appState.tr('patientUniversalHealthcare')}
                  </span>
                </label>

                <label class="flex items-center gap-3 cursor-pointer">
                  <input
                    type="checkbox"
                    class="checkbox checkbox-sm"
                    bind:checked={longDurationDisease}
                  />
                  <span class="label-text font-medium">
                    {appState.tr('patientLongDurationDisease')}
                  </span>
                </label>
              </div>

              <div>
                <label class="label" for="pat-diabete-network">
                  <span class="label-text font-medium">{appState.tr('patientDiabeteNetwork')}</span>
                </label>
                <input
                  id="pat-diabete-network"
                  type="number"
                  step="1"
                  min="0"
                  class={`input input-bordered w-full ${
                    fieldErrors.diabete_network_id ? 'input-error' : ''
                  }`}
                  bind:value={diabeteNetworkId}
                />
                {#if fieldErrors.diabete_network_id}
                  <p class="text-error text-xs mt-1">{fieldErrors.diabete_network_id}</p>
                {/if}
              </div>
            </div>
          </div>

          <div class="flex justify-end gap-2 pt-4 border-t border-base-200">
            <button
              type="button"
              class="btn btn-ghost"
              onclick={() =>
                appState.navigate(mode === 'edit' && id ? `/patients/${id}` : '/patients')}
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