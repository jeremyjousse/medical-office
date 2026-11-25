<script lang="ts">
  import { onMount } from 'svelte';
  import Flash from './lib/components/Flash.svelte';
  import Navbar from './lib/components/Navbar.svelte';
  import { appState } from './lib/state.svelte';
  import BankAccounts from './lib/views/BankAccounts.svelte';
  import Home from './lib/views/Home.svelte';
  import OfficeEdit from './lib/views/OfficeEdit.svelte';
  import Practitioners from './lib/views/Practitioners.svelte';
  import SetupDb from './lib/views/SetupDb.svelte';
  import TreatmentTypes from './lib/views/TreatmentTypes.svelte';

  onMount(() => {
    const onHashChange = () => {
      appState.hash = window.location.hash || '#/';
    };
    window.addEventListener('hashchange', onHashChange);
    appState.init();
    return () => window.removeEventListener('hashchange', onHashChange);
  });

  function deferredModuleLabel(
    mod: 'patients' | 'medical-treatments' | 'payments' | 'bank-deposits',
  ): string {
    switch (mod) {
      case 'patients':
        return appState.tr('navPatients');
      case 'medical-treatments':
        return appState.tr('navMedicalTreatments');
      case 'payments':
        return appState.tr('navPayments');
      case 'bank-deposits':
        return appState.tr('navBankDeposits');
    }
  }
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
    {:else if appState.route.view === 'practitioners'}
      <Practitioners mode={appState.route.mode} id={appState.route.id} />
    {:else if appState.route.view === 'treatment-types'}
      <TreatmentTypes mode={appState.route.mode} id={appState.route.id} />
    {:else if appState.route.view === 'bank-accounts'}
      <BankAccounts mode={appState.route.mode} id={appState.route.id} />
    {:else if appState.route.view === 'deferred'}
      <div class="max-w-2xl mx-auto px-4 py-10">
        <div class="card bg-base-100 border border-base-300 shadow-xs">
          <div class="card-body">
            <h1 class="card-title text-lg">{appState.tr('deferredTitle')}</h1>
            <p class="text-sm text-base-content/75">
              {appState.tr('deferredDesc', {
                module: deferredModuleLabel(appState.route.module),
              })}
            </p>
            {#if Object.keys(appState.route.query).length > 0}
              <p class="text-xs font-mono bg-base-200 px-3 py-2 rounded mt-2">
                {appState.tr('deferredFilterInfo', {
                  filter: new URLSearchParams(appState.route.query).toString(),
                })}
              </p>
            {/if}
            <div class="card-actions justify-end mt-4">
              <button
                type="button"
                class="btn btn-sm btn-primary"
                onclick={() => appState.navigate('/')}
              >
                {appState.tr('navHome')}
              </button>
            </div>
          </div>
        </div>
      </div>
    {/if}
  </main>
</div>
