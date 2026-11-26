<script lang="ts">
  import { onMount } from 'svelte';
  import Flash from './lib/components/Flash.svelte';
  import Navbar from './lib/components/Navbar.svelte';
  import { appState } from './lib/state.svelte';
  import BankAccounts from './lib/views/BankAccounts.svelte';
  import BankDeposits from './lib/views/BankDeposits.svelte';
  import Home from './lib/views/Home.svelte';
  import OfficeEdit from './lib/views/OfficeEdit.svelte';
  import PatientForm from './lib/views/PatientForm.svelte';
  import PatientImport from './lib/views/PatientImport.svelte';
  import PatientShow from './lib/views/PatientShow.svelte';
  import PatientsList from './lib/views/PatientsList.svelte';
  import PaymentListing from './lib/views/PaymentListing.svelte';
  import Payments from './lib/views/Payments.svelte';
  import Practitioners from './lib/views/Practitioners.svelte';
  import SetupDb from './lib/views/SetupDb.svelte';
  import TreatmentShow from './lib/views/TreatmentShow.svelte';
  import TreatmentTypes from './lib/views/TreatmentTypes.svelte';
  import Treatments from './lib/views/Treatments.svelte';

  onMount(() => {
    const onHashChange = () => {
      appState.hash = window.location.hash || '#/';
    };
    window.addEventListener('hashchange', onHashChange);
    appState.init();
    return () => window.removeEventListener('hashchange', onHashChange);
  });
</script>

<div class="min-h-screen bg-base-200 flex flex-col">
  <Navbar />
  <Flash />

  <main class="flex-1">
    {#if appState.initializing}
      <div class="flex items-center justify-center py-20">
        <span class="loading loading-spinner loading-lg text-primary"></span>
      </div>
    {:else if !appState.isInitialized || appState.route.view === 'setup'}
      <SetupDb />
    {:else if appState.route.view === 'home'}
      <Home />
    {:else if appState.route.view === 'office-edit'}
      <OfficeEdit />
    {:else if appState.route.view === 'patients'}
      {#if appState.route.mode === 'index'}
        <PatientsList />
      {:else if appState.route.mode === 'import'}
        <PatientImport />
      {:else if appState.route.mode === 'show' && appState.route.id}
        <PatientShow id={appState.route.id} />
      {:else if appState.route.mode === 'new' || appState.route.mode === 'edit'}
        <PatientForm mode={appState.route.mode} id={appState.route.id} />
      {/if}
    {:else if appState.route.view === 'practitioners'}
      <Practitioners mode={appState.route.mode} id={appState.route.id} />
    {:else if appState.route.view === 'medical-treatments'}
      {#if appState.route.mode === 'show' && appState.route.id}
        <TreatmentShow id={appState.route.id} />
      {:else if appState.route.mode !== 'show'}
        <Treatments
          mode={appState.route.mode}
          id={appState.route.id}
          query={appState.route.query}
        />
      {/if}
    {:else if appState.route.view === 'treatment-types'}
      <TreatmentTypes mode={appState.route.mode} id={appState.route.id} />
    {:else if appState.route.view === 'bank-accounts'}
      <BankAccounts mode={appState.route.mode} id={appState.route.id} />
    {:else if appState.route.view === 'payments'}
      {#if appState.route.mode === 'listing'}
        <PaymentListing />
      {:else}
        <Payments
          mode={appState.route.mode}
          id={appState.route.id}
          query={appState.route.query}
        />
      {/if}
    {:else if appState.route.view === 'bank-deposits'}
      <BankDeposits mode={appState.route.mode} id={appState.route.id} />
    {/if}
  </main>
</div>
