<script lang="ts">
  import {
    Stethoscope,
    Settings,
    Building2,
    Landmark,
    ClipboardList,
    Upload,
    Download,
    Users,
    Globe,
  } from 'lucide-svelte';
  import {
    exportSqliteDb,
    importSqliteDb,
    parseApiError,
    pickSaveSqliteFile,
    pickSqliteFile,
    switchUser,
  } from '../api';
  import { appState } from '../state.svelte';
  import Modal from './Modal.svelte';

  let pendingImportPath = $state<string | null>(null);
  let busy = $state(false);

  function closeMenus() {
    if (document.activeElement instanceof HTMLElement) {
      document.activeElement.blur();
    }
  }

  function navLinkClass(active: boolean): string {
    return active ? 'btn btn-sm btn-primary' : 'btn btn-sm btn-ghost';
  }

  async function handleStartImport() {
    closeMenus();
    try {
      const selected = await pickSqliteFile();
      if (!selected) return;
      pendingImportPath = selected;
    } catch (err) {
      appState.addFlash('error', parseApiError(err, appState.locale).message);
    }
  }

  async function confirmImport() {
    if (!pendingImportPath) return;
    busy = true;
    try {
      const status = await importSqliteDb(pendingImportPath);
      await appState.applyDbStatus(status);
      pendingImportPath = null;
      appState.addFlash('success', appState.tr('setupImportedSuccess'));
      if (status.needs_office_setup) {
        appState.navigate('/office/edit');
      } else {
        appState.navigate('/');
      }
    } catch (err) {
      appState.addFlash('error', parseApiError(err, appState.locale).message);
    } finally {
      busy = false;
    }
  }

  async function handleExportDb() {
    closeMenus();
    try {
      const dest = await pickSaveSqliteFile();
      if (!dest) return;
      await exportSqliteDb(dest);
      appState.addFlash('success', appState.tr('setupExportSuccess', { path: dest }));
    } catch (err) {
      appState.addFlash('error', parseApiError(err, appState.locale).message);
    }
  }

  async function handleSwitchUser(userId: number) {
    closeMenus();
    try {
      const status = await switchUser(userId);
      await appState.applyDbStatus(status);
      if (status.active_user) {
        appState.addFlash(
          'info',
          appState.tr('setupUserSwitched', { email: status.active_user.email }),
        );
      }
      if (status.needs_office_setup) {
        appState.navigate('/office/edit');
      } else {
        appState.navigate('/');
      }
    } catch (err) {
      appState.addFlash('error', parseApiError(err, appState.locale).message);
    }
  }
</script>

<header class="navbar bg-base-100 border-b border-base-300 px-4 shadow-xs no-print">
  <div class="navbar-start gap-2">
    <a
      href="#/"
      class="btn btn-ghost text-base font-bold gap-2"
      onclick={(e) => {
        e.preventDefault();
        appState.navigate(appState.isInitialized ? '/' : '/setup');
      }}
    >
      <Stethoscope class="w-5 h-5 text-primary" />
      <span>{appState.tr('appTitle')}</span>
    </a>

    {#if appState.isInitialized}
      <div class="dropdown lg:hidden">
        <button
          type="button"
          tabindex="0"
          class="btn btn-ghost btn-sm"
          aria-label="Menu"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-5 w-5"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M4 6h16M4 12h16M4 18h16"
            />
          </svg>
        </button>
        <ul
          class="dropdown-content menu bg-base-100 rounded-box z-50 w-56 p-2 shadow-lg border border-base-300 mt-2"
        >
          <li>
            <a
              href="#/"
              onclick={(e) => {
                e.preventDefault();
                closeMenus();
                appState.navigate('/');
              }}
            >
              {appState.tr('navHome')}
            </a>
          </li>
          <li>
            <a
              href="#/patients"
              onclick={(e) => {
                e.preventDefault();
                closeMenus();
                appState.navigate('/patients');
              }}
            >
              {appState.tr('navPatients')}
            </a>
          </li>
          <li>
            <a
              href="#/practitioners"
              onclick={(e) => {
                e.preventDefault();
                closeMenus();
                appState.navigate('/practitioners');
              }}
            >
              {appState.tr('navPractitioners')}
            </a>
          </li>
          <li>
            <a
              href="#/medical-treatments"
              onclick={(e) => {
                e.preventDefault();
                closeMenus();
                appState.navigate('/medical-treatments');
              }}
            >
              {appState.tr('navMedicalTreatments')}
            </a>
          </li>
          <li>
            <a
              href="#/payments"
              onclick={(e) => {
                e.preventDefault();
                closeMenus();
                appState.navigate('/payments');
              }}
            >
              {appState.tr('navPayments')}
            </a>
          </li>
          <li>
            <a
              href="#/bank-deposits"
              onclick={(e) => {
                e.preventDefault();
                closeMenus();
                appState.navigate('/bank-deposits');
              }}
            >
              {appState.tr('navBankDeposits')}
            </a>
          </li>
        </ul>
      </div>

      <nav class="hidden lg:flex items-center gap-1 ml-2">
        <a
          href="#/"
          class={navLinkClass(appState.route.view === 'home')}
          onclick={(e) => {
            e.preventDefault();
            appState.navigate('/');
          }}
        >
          {appState.tr('navHome')}
        </a>
        <a
          href="#/patients"
          class={navLinkClass(appState.route.view === 'patients')}
          onclick={(e) => {
            e.preventDefault();
            appState.navigate('/patients');
          }}
        >
          {appState.tr('navPatients')}
        </a>
        <a
          href="#/practitioners"
          class={navLinkClass(appState.route.view === 'practitioners')}
          onclick={(e) => {
            e.preventDefault();
            appState.navigate('/practitioners');
          }}
        >
          {appState.tr('navPractitioners')}
        </a>
        <a
          href="#/medical-treatments"
          class={navLinkClass(appState.route.view === 'medical-treatments')}
          onclick={(e) => {
            e.preventDefault();
            appState.navigate('/medical-treatments');
          }}
        >
          {appState.tr('navMedicalTreatments')}
        </a>
        <a
          href="#/payments"
          class={navLinkClass(appState.route.view === 'payments')}
          onclick={(e) => {
            e.preventDefault();
            appState.navigate('/payments');
          }}
        >
          {appState.tr('navPayments')}
        </a>
        <a
          href="#/bank-deposits"
          class={navLinkClass(appState.route.view === 'bank-deposits')}
          onclick={(e) => {
            e.preventDefault();
            appState.navigate('/bank-deposits');
          }}
        >
          {appState.tr('navBankDeposits')}
        </a>
      </nav>
    {/if}
  </div>

  <div class="navbar-end gap-2">
    <div class="join" role="group" aria-label={appState.tr('navLanguage')}>
      <button
        type="button"
        class={`join-item btn btn-xs ${appState.locale === 'en' ? 'btn-primary' : 'btn-ghost'}`}
        onclick={() => appState.setLocale('en')}
      >
        EN
      </button>
      <button
        type="button"
        class={`join-item btn btn-xs ${appState.locale === 'fr' ? 'btn-primary' : 'btn-ghost'}`}
        onclick={() => appState.setLocale('fr')}
      >
        FR
      </button>
    </div>

    {#if appState.isInitialized}
      <div class="dropdown dropdown-end">
        <button
          type="button"
          tabindex="0"
          class="btn btn-ghost btn-sm gap-1"
          aria-label={appState.tr('navSettings')}
        >
          <Settings class="w-4 h-4" />
          <span class="hidden sm:inline">{appState.tr('navSettings')}</span>
        </button>
        <ul
          class="dropdown-content menu bg-base-100 rounded-box z-50 w-64 p-2 shadow-lg border border-base-300 mt-2"
        >
          <li>
            <a
              href="#/office/edit"
              onclick={(e) => {
                e.preventDefault();
                closeMenus();
                appState.navigate('/office/edit');
              }}
            >
              <Building2 class="w-4 h-4" />
              {appState.tr('navMyOffice')}
            </a>
          </li>
          <li>
            <a
              href="#/bank-accounts"
              onclick={(e) => {
                e.preventDefault();
                closeMenus();
                appState.navigate('/bank-accounts');
              }}
            >
              <Landmark class="w-4 h-4" />
              {appState.tr('navBankAccounts')}
            </a>
          </li>
          <li>
            <a
              href="#/treatment-types"
              onclick={(e) => {
                e.preventDefault();
                closeMenus();
                appState.navigate('/treatment-types');
              }}
            >
              <ClipboardList class="w-4 h-4" />
              {appState.tr('navTreatmentTypes')}
            </a>
          </li>

          <li class="menu-title mt-1"><span>DB</span></li>
          <li>
            <button type="button" onclick={handleStartImport}>
              <Upload class="w-4 h-4" />
              {appState.tr('navImportDb')}
            </button>
          </li>
          <li>
            <button type="button" onclick={handleExportDb}>
              <Download class="w-4 h-4" />
              {appState.tr('navExportDb')}
            </button>
          </li>

          {#if appState.users.length > 1}
            <li class="menu-title mt-1">
              <span class="flex items-center gap-1">
                <Users class="w-3.5 h-3.5" />
                {appState.tr('navProfile')}
              </span>
            </li>
            {#each appState.users as user (user.id)}
              <li>
                <button
                  type="button"
                  class={user.id === appState.currentUser?.id ? 'active font-semibold' : ''}
                  onclick={() => handleSwitchUser(user.id)}
                >
                  <Globe class="w-3.5 h-3.5 opacity-60" />
                  <span class="truncate">{user.email}</span>
                </button>
              </li>
            {/each}
          {/if}
        </ul>
      </div>
    {/if}
  </div>
</header>

<Modal
  open={pendingImportPath !== null}
  title={appState.tr('setupImportConfirmTitle')}
  confirmLabel={appState.tr('confirm')}
  loading={busy}
  onConfirm={confirmImport}
  onCancel={() => (pendingImportPath = null)}
>
  <p>{appState.tr('setupImportConfirmBody')}</p>
  {#if pendingImportPath}
    <p class="mt-2 font-mono text-xs break-all bg-base-200 p-2 rounded">{pendingImportPath}</p>
  {/if}
</Modal>
