<script lang="ts">
  import { ArrowLeft, Save } from 'lucide-svelte';
  import {
    getOffice,
    listCountries,
    parseApiError,
    updateOffice,
    type Country,
  } from '../api';
  import SearchSelect from '../components/SearchSelect.svelte';
  import { appState } from '../state.svelte';

  let loading = $state(true);
  let saving = $state(false);
  let countries = $state<Country[]>([]);
  let fieldErrors = $state<Record<string, string>>({});

  let name = $state('');
  let address = $state('');
  let postalCode = $state('');
  let city = $state('');
  let countryId = $state<number | null>(null);
  let phone = $state('');
  let website = $state('');
  let email = $state('');

  let countryOptions = $derived(
    countries.map((c) => ({
      value: c.id,
      label: c.iso ? `${c.name ?? ''} (${c.iso})` : (c.name ?? String(c.id)),
    })),
  );

  $effect(() => {
    const activeUserId = appState.currentUser?.id;
    if (!appState.isInitialized || !activeUserId) return;

    let cancelled = false;
    loading = true;
    fieldErrors = {};

    Promise.all([getOffice(), listCountries()])
      .then(([office, allCountries]) => {
        if (cancelled) return;
        countries = allCountries;
        name = office.name ?? '';
        address = office.address ?? '';
        postalCode = office.postal_code ?? '';
        city = office.city ?? '';
        countryId = office.country_id ?? null;
        phone = office.phone ?? '';
        website = office.website ?? '';
        email = office.email ?? '';
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

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    saving = true;
    fieldErrors = {};
    try {
      await updateOffice({
        name,
        address,
        postal_code: postalCode,
        city,
        country_id: countryId,
        phone,
        website: website.trim() ? website : null,
        email,
      });
      await appState.refreshDbStatus();
      appState.addFlash('success', appState.tr('officeUpdatedSuccess'));
      appState.navigate('/');
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

<div class="max-w-3xl mx-auto px-4 py-6">
  <div class="card bg-base-100 border border-base-300 shadow-xs">
    <div class="card-body">
      <div class="flex items-center justify-between border-b border-base-200 pb-3 mb-4">
        <h1 class="text-xl font-bold">{appState.tr('officeTitle')}</h1>
        {#if !appState.needsOfficeSetup}
          <button
            type="button"
            class="btn btn-ghost btn-sm"
            onclick={() => appState.navigate('/')}
          >
            <ArrowLeft class="w-4 h-4" />
            {appState.tr('back')}
          </button>
        {/if}
      </div>

      {#if loading}
        <div class="flex justify-center py-8">
          <span class="loading loading-spinner loading-md text-primary"></span>
        </div>
      {:else}
        <form onsubmit={handleSubmit} class="space-y-4">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="md:col-span-2">
              <label class="label" for="office-name">
                <span class="label-text font-medium">{appState.tr('officeName')} *</span>
              </label>
              <input
                id="office-name"
                type="text"
                class={`input input-bordered w-full ${fieldErrors.name ? 'input-error' : ''}`}
                bind:value={name}
              />
              {#if fieldErrors.name}
                <p class="text-error text-xs mt-1">{fieldErrors.name}</p>
              {/if}
            </div>

            <div class="md:col-span-2">
              <label class="label" for="office-address">
                <span class="label-text font-medium">{appState.tr('officeAddress')} *</span>
              </label>
              <input
                id="office-address"
                type="text"
                class={`input input-bordered w-full ${fieldErrors.address ? 'input-error' : ''}`}
                bind:value={address}
              />
              {#if fieldErrors.address}
                <p class="text-error text-xs mt-1">{fieldErrors.address}</p>
              {/if}
            </div>

            <div>
              <label class="label" for="office-postal">
                <span class="label-text font-medium">{appState.tr('officePostalCode')} *</span>
              </label>
              <input
                id="office-postal"
                type="text"
                class={`input input-bordered w-full ${fieldErrors.postal_code ? 'input-error' : ''}`}
                bind:value={postalCode}
              />
              {#if fieldErrors.postal_code}
                <p class="text-error text-xs mt-1">{fieldErrors.postal_code}</p>
              {/if}
            </div>

            <div>
              <label class="label" for="office-city">
                <span class="label-text font-medium">{appState.tr('officeCity')} *</span>
              </label>
              <input
                id="office-city"
                type="text"
                class={`input input-bordered w-full ${fieldErrors.city ? 'input-error' : ''}`}
                bind:value={city}
              />
              {#if fieldErrors.city}
                <p class="text-error text-xs mt-1">{fieldErrors.city}</p>
              {/if}
            </div>

            <div class="md:col-span-2">
              <label class="label" for="office-country">
                <span class="label-text font-medium">{appState.tr('officeCountry')} *</span>
              </label>
              <SearchSelect
                id="office-country"
                options={countryOptions}
                bind:value={countryId}
                error={fieldErrors.country_id}
              />
              {#if fieldErrors.country_id}
                <p class="text-error text-xs mt-1">{fieldErrors.country_id}</p>
              {/if}
            </div>

            <div>
              <label class="label" for="office-phone">
                <span class="label-text font-medium">{appState.tr('officePhone')} *</span>
              </label>
              <input
                id="office-phone"
                type="text"
                class={`input input-bordered w-full ${fieldErrors.phone ? 'input-error' : ''}`}
                bind:value={phone}
              />
              {#if fieldErrors.phone}
                <p class="text-error text-xs mt-1">{fieldErrors.phone}</p>
              {/if}
            </div>

            <div>
              <label class="label" for="office-email">
                <span class="label-text font-medium">{appState.tr('officeEmail')} *</span>
              </label>
              <input
                id="office-email"
                type="email"
                class={`input input-bordered w-full ${fieldErrors.email ? 'input-error' : ''}`}
                bind:value={email}
              />
              {#if fieldErrors.email}
                <p class="text-error text-xs mt-1">{fieldErrors.email}</p>
              {/if}
            </div>

            <div class="md:col-span-2">
              <label class="label" for="office-website">
                <span class="label-text font-medium">{appState.tr('officeWebsite')}</span>
              </label>
              <input
                id="office-website"
                type="url"
                placeholder="https://..."
                class={`input input-bordered w-full ${fieldErrors.website ? 'input-error' : ''}`}
                bind:value={website}
              />
              {#if fieldErrors.website}
                <p class="text-error text-xs mt-1">{fieldErrors.website}</p>
              {/if}
            </div>
          </div>

          <div class="flex justify-end pt-2">
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
