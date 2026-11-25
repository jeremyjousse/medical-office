<script lang="ts">
  import { untrack } from 'svelte';
  import { Plus, Eye, Pencil, ArrowLeft, Save, Search, RotateCcw } from 'lucide-svelte';
  import {
    createPractitioner,
    getPractitioner,
    listCountries,
    listPractitioners,
    parseApiError,
    updatePractitioner,
    type Country,
    type PaginatedResult,
    type PractitionerView,
  } from '../api';
  import Pagination from '../components/Pagination.svelte';
  import SearchSelect from '../components/SearchSelect.svelte';
  import { specialityLabel, specialityOptions } from '../i18n';
  import { appState, type ResourceMode } from '../state.svelte';

  interface Props {
    mode: ResourceMode;
    id?: number;
  }

  let { mode, id }: Props = $props();

  // Index state
  let page = $state(1);
  let filterSpecialityId = $state<number | null>(null);
  let filterFirstName = $state('');
  let filterLastName = $state('');
  let filterCity = $state('');
  let listing = $state<PaginatedResult<PractitionerView> | null>(null);
  let loading = $state(false);

  // Show & Form state
  let current = $state<PractitionerView | null>(null);
  let countries = $state<Country[]>([]);
  let saving = $state(false);
  let fieldErrors = $state<Record<string, string>>({});

  let specialityId = $state<number | null>(null);
  let firstName = $state('');
  let lastName = $state('');
  let phone = $state('');
  let mobilePhone = $state('');
  let email = $state('');
  let address = $state('');
  let postalCode = $state('');
  let city = $state('');
  let countryId = $state<number | null>(null);
  let note = $state('');

  let countryOptions = $derived(
    countries.map((c) => ({
      value: c.id,
      label: c.iso ? `${c.name ?? ''} (${c.iso})` : (c.name ?? String(c.id)),
    })),
  );

  let specOptions = $derived(specialityOptions(appState.locale));

  async function loadIndex(targetPage = page) {
    loading = true;
    try {
      listing = await listPractitioners({
        page: targetPage,
        per_page: 8,
        speciality_id: filterSpecialityId,
        first_name: filterFirstName,
        last_name: filterLastName,
        city: filterCity,
      });
      page = listing.page;
    } catch (err) {
      appState.addFlash('error', parseApiError(err, appState.locale).message);
    } finally {
      loading = false;
    }
  }

  function resetForm() {
    fieldErrors = {};
    specialityId = null;
    firstName = '';
    lastName = '';
    phone = '';
    mobilePhone = '';
    email = '';
    address = '';
    postalCode = '';
    city = '';
    countryId = null;
    note = '';
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
      untrack(() => resetForm());
      loading = true;
      listCountries()
        .then((res) => {
          if (!cancelled) countries = res;
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

    if ((currentMode === 'show' || currentMode === 'edit') && currentId) {
      loading = true;
      fieldErrors = {};
      Promise.all([
        getPractitioner(currentId),
        currentMode === 'edit' ? listCountries() : Promise.resolve([] as Country[]),
      ])
        .then(([item, allCountries]) => {
          if (cancelled) return;
          current = item;
          if (currentMode === 'edit') {
            countries = allCountries;
            specialityId = item.speciality_id ?? null;
            firstName = item.first_name ?? '';
            lastName = item.last_name ?? '';
            phone = item.phone ?? '';
            mobilePhone = item.mobile_phone ?? '';
            email = item.email ?? '';
            address = item.address ?? '';
            postalCode = item.postal_code ?? '';
            city = item.city ?? '';
            countryId = item.country_id ?? null;
            note = item.note ?? '';
          }
        })
        .catch((err) => {
          if (cancelled) return;
          appState.addFlash('error', parseApiError(err, appState.locale).message);
          appState.navigate('/practitioners');
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
    filterSpecialityId = null;
    filterFirstName = '';
    filterLastName = '';
    filterCity = '';
    page = 1;
    loadIndex(1);
  }

  async function handleSave(e: SubmitEvent) {
    e.preventDefault();
    saving = true;
    fieldErrors = {};
    const payload = {
      speciality_id: specialityId,
      first_name: firstName,
      last_name: lastName,
      phone: phone.trim() ? phone : null,
      mobile_phone: mobilePhone.trim() ? mobilePhone : null,
      email: email.trim() ? email : null,
      address,
      postal_code: postalCode,
      city,
      country_id: countryId,
      note: note.trim() ? note : null,
    };

    try {
      if (mode === 'new') {
        await createPractitioner(payload);
        page = 1;
        appState.addFlash('success', appState.tr('practitionerCreatedSuccess'));
        appState.navigate('/practitioners');
      } else if (mode === 'edit' && id) {
        const updated = await updatePractitioner(id, payload);
        appState.addFlash('success', appState.tr('practitionerUpdatedSuccess'));
        appState.navigate(`/practitioners/${updated.id}`);
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

<div class="max-w-7xl mx-auto px-4 py-6">
  {#if mode === 'index'}
    <div class="card bg-base-100 border border-base-300 shadow-xs">
      <div class="card-body">
        <div class="flex flex-wrap items-center justify-between gap-3 border-b border-base-200 pb-3">
          <h1 class="text-xl font-bold">{appState.tr('practitionersTitle')}</h1>
          <button
            type="button"
            class="btn btn-primary btn-sm"
            onclick={() => appState.navigate('/practitioners/new')}
          >
            <Plus class="w-4 h-4" />
            {appState.tr('practitionerNewBtn')}
          </button>
        </div>

        {#if listing}
          <Pagination
            page={listing.page}
            perPage={listing.per_page}
            filteredTotal={listing.filtered_total}
            total={listing.total}
            label={appState.tr('practitionerPlural')}
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
                <th>{appState.tr('practitionerSpeciality')}</th>
                <th>{appState.tr('practitionerFirstName')}</th>
                <th>{appState.tr('practitionerLastName')}</th>
                <th>{appState.tr('practitionerCity')}</th>
                <th>{appState.tr('actions')}</th>
              </tr>
              <tr class="bg-base-200/60">
                <td></td>
                <td>
                  <select
                    class="select select-bordered select-xs w-full min-w-36"
                    aria-label={appState.tr('practitionerSpeciality')}
                    bind:value={filterSpecialityId}
                  >
                    <option value={null}>{appState.tr('practitionerAllSpecialities')}</option>
                    {#each specOptions as opt (opt.value)}
                      <option value={opt.value}>{opt.label}</option>
                    {/each}
                  </select>
                </td>
                <td>
                  <input
                    type="text"
                    class="input input-bordered input-xs w-full min-w-28"
                    aria-label={appState.tr('practitionerFirstName')}
                    bind:value={filterFirstName}
                  />
                </td>
                <td>
                  <input
                    type="text"
                    class="input input-bordered input-xs w-full min-w-28"
                    aria-label={appState.tr('practitionerLastName')}
                    bind:value={filterLastName}
                  />
                </td>
                <td>
                  <input
                    type="text"
                    class="input input-bordered input-xs w-full min-w-28"
                    aria-label={appState.tr('practitionerCity')}
                    bind:value={filterCity}
                  />
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
                        onclick={() => appState.navigate(`/practitioners/${item.id}`)}
                      >
                        <Eye class="w-3.5 h-3.5" />
                        {appState.tr('show')}
                      </button>
                    </td>
                    <td>{specialityLabel(appState.locale, item.speciality_id)}</td>
                    <td>{item.first_name ?? ''}</td>
                    <td>{item.last_name ?? ''}</td>
                    <td>{item.city ?? ''}</td>
                    <td>
                      <button
                        type="button"
                        class="btn btn-xs btn-ghost"
                        onclick={() => appState.navigate(`/practitioners/${item.id}/edit`)}
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
    <div class="max-w-3xl mx-auto">
      <div class="card bg-base-100 border border-base-300 shadow-xs">
        <div class="card-body">
          <div class="flex items-center justify-between border-b border-base-200 pb-3 mb-4">
            <h1 class="text-xl font-bold">{appState.tr('practitionerDetailTitle')}</h1>
            <div class="flex items-center gap-2">
              {#if current}
                <button
                  type="button"
                  class="btn btn-sm btn-outline"
                  onclick={() => appState.navigate(`/practitioners/${current!.id}/edit`)}
                >
                  <Pencil class="w-4 h-4" />
                  {appState.tr('edit')}
                </button>
              {/if}
              <button
                type="button"
                class="btn btn-sm btn-ghost"
                onclick={() => appState.navigate('/practitioners')}
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
            <div class="overflow-x-auto">
              <table class="table table-bordered">
                <tbody>
                  <tr>
                    <th class="w-48 bg-base-200/50">{appState.tr('practitionerSpeciality')}</th>
                    <td>{specialityLabel(appState.locale, current.speciality_id)}</td>
                  </tr>
                  <tr>
                    <th class="bg-base-200/50">{appState.tr('practitionerFirstName')}</th>
                    <td>{current.first_name ?? ''}</td>
                  </tr>
                  <tr>
                    <th class="bg-base-200/50">{appState.tr('practitionerLastName')}</th>
                    <td>{current.last_name ?? ''}</td>
                  </tr>
                  <tr>
                    <th class="bg-base-200/50">{appState.tr('practitionerPhone')}</th>
                    <td>{current.phone ?? ''}</td>
                  </tr>
                  <tr>
                    <th class="bg-base-200/50">{appState.tr('practitionerMobilePhone')}</th>
                    <td>{current.mobile_phone ?? ''}</td>
                  </tr>
                  <tr>
                    <th class="bg-base-200/50">{appState.tr('practitionerEmail')}</th>
                    <td>{current.email ?? ''}</td>
                  </tr>
                  <tr>
                    <th class="bg-base-200/50">{appState.tr('practitionerAddress')}</th>
                    <td>{current.address ?? ''}</td>
                  </tr>
                  <tr>
                    <th class="bg-base-200/50">{appState.tr('practitionerPostalCode')}</th>
                    <td>{current.postal_code ?? ''}</td>
                  </tr>
                  <tr>
                    <th class="bg-base-200/50">{appState.tr('practitionerCity')}</th>
                    <td>{current.city ?? ''}</td>
                  </tr>
                  <tr>
                    <th class="bg-base-200/50">{appState.tr('practitionerCountry')}</th>
                    <td>{current.country_name ?? ''}</td>
                  </tr>
                  <tr>
                    <th class="bg-base-200/50">{appState.tr('practitionerNote')}</th>
                    <td class="whitespace-pre-wrap">{current.note ?? ''}</td>
                  </tr>
                </tbody>
              </table>
            </div>
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
              {mode === 'new'
                ? appState.tr('practitionerNewTitle')
                : appState.tr('practitionerEditTitle')}
            </h1>
            <button
              type="button"
              class="btn btn-sm btn-ghost"
              onclick={() => appState.navigate('/practitioners')}
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
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="md:col-span-2">
                  <label class="label" for="prac-speciality">
                    <span class="label-text font-medium">
                      {appState.tr('practitionerSpeciality')} *
                    </span>
                  </label>
                  <select
                    id="prac-speciality"
                    class={`select select-bordered w-full ${
                      fieldErrors.speciality_id ? 'select-error' : ''
                    }`}
                    bind:value={specialityId}
                  >
                    <option value={null}>—</option>
                    {#each specOptions as opt (opt.value)}
                      <option value={opt.value}>{opt.label}</option>
                    {/each}
                  </select>
                  {#if fieldErrors.speciality_id}
                    <p class="text-error text-xs mt-1">{fieldErrors.speciality_id}</p>
                  {/if}
                </div>

                <div>
                  <label class="label" for="prac-first-name">
                    <span class="label-text font-medium">
                      {appState.tr('practitionerFirstName')} *
                    </span>
                  </label>
                  <input
                    id="prac-first-name"
                    type="text"
                    class={`input input-bordered w-full ${
                      fieldErrors.first_name ? 'input-error' : ''
                    }`}
                    bind:value={firstName}
                  />
                  {#if fieldErrors.first_name}
                    <p class="text-error text-xs mt-1">{fieldErrors.first_name}</p>
                  {/if}
                </div>

                <div>
                  <label class="label" for="prac-last-name">
                    <span class="label-text font-medium">
                      {appState.tr('practitionerLastName')} *
                    </span>
                  </label>
                  <input
                    id="prac-last-name"
                    type="text"
                    class={`input input-bordered w-full ${
                      fieldErrors.last_name ? 'input-error' : ''
                    }`}
                    bind:value={lastName}
                  />
                  {#if fieldErrors.last_name}
                    <p class="text-error text-xs mt-1">{fieldErrors.last_name}</p>
                  {/if}
                </div>

                <div>
                  <label class="label" for="prac-phone">
                    <span class="label-text font-medium">{appState.tr('practitionerPhone')}</span>
                  </label>
                  <input
                    id="prac-phone"
                    type="text"
                    class={`input input-bordered w-full ${fieldErrors.phone ? 'input-error' : ''}`}
                    bind:value={phone}
                  />
                  {#if fieldErrors.phone}
                    <p class="text-error text-xs mt-1">{fieldErrors.phone}</p>
                  {/if}
                </div>

                <div>
                  <label class="label" for="prac-mobile">
                    <span class="label-text font-medium">
                      {appState.tr('practitionerMobilePhone')}
                    </span>
                  </label>
                  <input
                    id="prac-mobile"
                    type="text"
                    class={`input input-bordered w-full ${
                      fieldErrors.mobile_phone ? 'input-error' : ''
                    }`}
                    bind:value={mobilePhone}
                  />
                  {#if fieldErrors.mobile_phone}
                    <p class="text-error text-xs mt-1">{fieldErrors.mobile_phone}</p>
                  {/if}
                </div>

                <div class="md:col-span-2">
                  <label class="label" for="prac-email">
                    <span class="label-text font-medium">{appState.tr('practitionerEmail')}</span>
                  </label>
                  <input
                    id="prac-email"
                    type="text"
                    class={`input input-bordered w-full ${fieldErrors.email ? 'input-error' : ''}`}
                    bind:value={email}
                  />
                  {#if fieldErrors.email}
                    <p class="text-error text-xs mt-1">{fieldErrors.email}</p>
                  {/if}
                </div>

                <div class="md:col-span-2">
                  <label class="label" for="prac-address">
                    <span class="label-text font-medium">
                      {appState.tr('practitionerAddress')} *
                    </span>
                  </label>
                  <input
                    id="prac-address"
                    type="text"
                    class={`input input-bordered w-full ${
                      fieldErrors.address ? 'input-error' : ''
                    }`}
                    bind:value={address}
                  />
                  {#if fieldErrors.address}
                    <p class="text-error text-xs mt-1">{fieldErrors.address}</p>
                  {/if}
                </div>

                <div>
                  <label class="label" for="prac-postal">
                    <span class="label-text font-medium">
                      {appState.tr('practitionerPostalCode')} *
                    </span>
                  </label>
                  <input
                    id="prac-postal"
                    type="text"
                    class={`input input-bordered w-full ${
                      fieldErrors.postal_code ? 'input-error' : ''
                    }`}
                    bind:value={postalCode}
                  />
                  {#if fieldErrors.postal_code}
                    <p class="text-error text-xs mt-1">{fieldErrors.postal_code}</p>
                  {/if}
                </div>

                <div>
                  <label class="label" for="prac-city">
                    <span class="label-text font-medium">{appState.tr('practitionerCity')} *</span>
                  </label>
                  <input
                    id="prac-city"
                    type="text"
                    class={`input input-bordered w-full ${fieldErrors.city ? 'input-error' : ''}`}
                    bind:value={city}
                  />
                  {#if fieldErrors.city}
                    <p class="text-error text-xs mt-1">{fieldErrors.city}</p>
                  {/if}
                </div>

                <div class="md:col-span-2">
                  <label class="label" for="prac-country">
                    <span class="label-text font-medium">
                      {appState.tr('practitionerCountry')} *
                    </span>
                  </label>
                  <SearchSelect
                    id="prac-country"
                    options={countryOptions}
                    bind:value={countryId}
                    error={fieldErrors.country_id}
                  />
                  {#if fieldErrors.country_id}
                    <p class="text-error text-xs mt-1">{fieldErrors.country_id}</p>
                  {/if}
                </div>

                <div class="md:col-span-2">
                  <label class="label" for="prac-note">
                    <span class="label-text font-medium">{appState.tr('practitionerNote')}</span>
                  </label>
                  <textarea
                    id="prac-note"
                    rows="3"
                    class={`textarea textarea-bordered w-full ${
                      fieldErrors.note ? 'textarea-error' : ''
                    }`}
                    bind:value={note}
                  ></textarea>
                  {#if fieldErrors.note}
                    <p class="text-error text-xs mt-1">{fieldErrors.note}</p>
                  {/if}
                </div>
              </div>

              <div class="flex justify-end gap-2 pt-2">
                <button
                  type="button"
                  class="btn btn-ghost"
                  onclick={() => appState.navigate('/practitioners')}
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
