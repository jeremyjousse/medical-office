<script lang="ts">
  import { ArrowLeft, FileSpreadsheet, Upload } from 'lucide-svelte';
  import {
    importPatientsCsv,
    parseApiError,
    pickCsvFile,
    type ImportPatientsResult,
  } from '../api';
  import { appState } from '../state.svelte';

  let selectedPath = $state<string | null>(null);
  let importing = $state(false);
  let result = $state<ImportPatientsResult | null>(null);

  async function handleChooseFile() {
    try {
      const path = await pickCsvFile();
      if (path) {
        selectedPath = path;
        result = null;
      }
    } catch (err) {
      appState.addFlash('error', parseApiError(err, appState.locale).message);
    }
  }

  async function handleImport(e: SubmitEvent) {
    e.preventDefault();
    if (!selectedPath) return;
    importing = true;
    result = null;
    try {
      const res = await importPatientsCsv(selectedPath);
      result = res;
      appState.patientFilters.page = 1;
      appState.addFlash(
        'success',
        appState.tr('patientImportSuccess', { count: res.imported_count }),
      );
    } catch (err) {
      appState.addFlash('error', parseApiError(err, appState.locale).message);
    } finally {
      importing = false;
    }
  }
</script>

<div class="max-w-3xl mx-auto px-4 py-6">
  <div class="card bg-base-100 border border-base-300 shadow-xs">
    <div class="card-body">
      <div class="flex items-center justify-between border-b border-base-200 pb-3 mb-4">
        <h1 class="text-xl font-bold">{appState.tr('patientImportTitle')}</h1>
        <button
          type="button"
          class="btn btn-sm btn-ghost"
          onclick={() => appState.navigate('/patients')}
        >
          <ArrowLeft class="w-4 h-4" />
          {appState.tr('back')}
        </button>
      </div>

      <p class="text-sm text-base-content/75 mb-4">
        {appState.tr('patientImportSubtitle')}
      </p>

      <form onsubmit={handleImport} class="space-y-4">
        <div class="flex flex-wrap items-center gap-3">
          <button
            type="button"
            class="btn btn-outline btn-sm"
            disabled={importing}
            onclick={handleChooseFile}
          >
            <FileSpreadsheet class="w-4 h-4" />
            {appState.tr('patientImportChooseFile')}
          </button>

          <button
            type="submit"
            class="btn btn-primary btn-sm"
            disabled={!selectedPath || importing}
          >
            {#if importing}
              <span class="loading loading-spinner loading-xs"></span>
              {appState.tr('loading')}
            {:else}
              <Upload class="w-4 h-4" />
              {appState.tr('patientImportSubmit')}
            {/if}
          </button>
        </div>

        {#if selectedPath}
          <div class="bg-base-200/70 rounded-lg p-3 text-xs">
            <span class="font-semibold">{appState.tr('patientImportSelectedFile')}:</span>
            <span class="font-mono break-all ml-1">{selectedPath}</span>
          </div>
        {/if}

        {#if result}
          <div class="alert alert-success text-sm">
            <span>
              {appState.tr('patientImportSummary', {
                imported: result.imported_count,
                skipped: result.skipped_count,
                total: result.total_rows,
              })}
            </span>
          </div>
        {/if}
      </form>
    </div>
  </div>
</div>