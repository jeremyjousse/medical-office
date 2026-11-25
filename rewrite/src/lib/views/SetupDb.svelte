<script lang="ts">
  import { Database, FolderOpen, PlusCircle } from 'lucide-svelte';
  import { createEmptyDb, importSqliteDb, parseApiError, pickSqliteFile } from '../api';
  import { appState } from '../state.svelte';

  let busyAction = $state<'import' | 'create' | null>(null);

  async function handleImport() {
    busyAction = 'import';
    try {
      const sourcePath = await pickSqliteFile();
      if (!sourcePath) {
        busyAction = null;
        return;
      }
      const status = await importSqliteDb(sourcePath);
      await appState.applyDbStatus(status);
      appState.addFlash('success', appState.tr('setupImportedSuccess'));
      if (status.needs_office_setup) {
        appState.navigate('/office/edit');
      } else {
        appState.navigate('/');
      }
    } catch (err) {
      appState.addFlash('error', parseApiError(err, appState.locale).message);
    } finally {
      busyAction = null;
    }
  }

  async function handleCreateEmpty() {
    busyAction = 'create';
    try {
      const status = await createEmptyDb();
      await appState.applyDbStatus(status);
      appState.addFlash('success', appState.tr('setupCreatedSuccess'));
      appState.navigate('/office/edit');
    } catch (err) {
      appState.addFlash('error', parseApiError(err, appState.locale).message);
    } finally {
      busyAction = null;
    }
  }
</script>

<div class="max-w-3xl mx-auto px-4 py-10">
  <div class="text-center mb-8">
    <div class="inline-flex items-center justify-center w-14 h-14 rounded-2xl bg-primary/10 mb-3">
      <Database class="w-7 h-7 text-primary" />
    </div>
    <h1 class="text-2xl font-bold">{appState.tr('setupTitle')}</h1>
    <p class="text-base-content/70 mt-2 max-w-xl mx-auto">{appState.tr('setupSubtitle')}</p>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
    <div class="card bg-base-100 border border-base-300 shadow-xs">
      <div class="card-body">
        <div class="flex items-center gap-2 text-primary font-semibold">
          <FolderOpen class="w-5 h-5" />
          <h2 class="card-title text-lg">{appState.tr('setupImportTitle')}</h2>
        </div>
        <p class="text-sm text-base-content/75 flex-1">{appState.tr('setupImportDesc')}</p>
        <div class="card-actions justify-end mt-4">
          <button
            type="button"
            class="btn btn-primary w-full"
            disabled={busyAction !== null}
            onclick={handleImport}
          >
            {#if busyAction === 'import'}
              <span class="loading loading-spinner loading-xs"></span>
            {/if}
            {appState.tr('setupImportBtn')}
          </button>
        </div>
      </div>
    </div>

    <div class="card bg-base-100 border border-base-300 shadow-xs">
      <div class="card-body">
        <div class="flex items-center gap-2 text-secondary font-semibold">
          <PlusCircle class="w-5 h-5" />
          <h2 class="card-title text-lg">{appState.tr('setupCreateTitle')}</h2>
        </div>
        <p class="text-sm text-base-content/75 flex-1">{appState.tr('setupCreateDesc')}</p>
        <div class="card-actions justify-end mt-4">
          <button
            type="button"
            class="btn btn-outline btn-secondary w-full"
            disabled={busyAction !== null}
            onclick={handleCreateEmpty}
          >
            {#if busyAction === 'create'}
              <span class="loading loading-spinner loading-xs"></span>
            {/if}
            {appState.tr('setupCreateBtn')}
          </button>
        </div>
      </div>
    </div>
  </div>
</div>
