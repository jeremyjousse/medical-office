<script lang="ts">
  import { Building2, AlertCircle, CheckCircle2 } from 'lucide-svelte';
  import { getHomeSummary, parseApiError, type HomeSummary } from '../api';
  import { appState } from '../state.svelte';

  let summary = $state<HomeSummary | null>(null);
  let loading = $state(true);

  $effect(() => {
    const activeUserId = appState.currentUser?.id;
    if (!appState.isInitialized || !activeUserId) return;

    let cancelled = false;
    loading = true;

    getHomeSummary()
      .then((res) => {
        if (cancelled) return;
        summary = res;
        if (res.needs_office_setup) {
          appState.addFlash('info', appState.tr('homeOfficeRequiredNotice'));
          appState.navigate('/office/edit');
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
</script>

<div class="max-w-5xl mx-auto px-4 py-6">
  {#if loading}
    <div class="flex justify-center py-12">
      <span class="loading loading-spinner loading-md text-primary"></span>
    </div>
  {:else if summary}
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div class="card bg-base-100 border border-base-300 shadow-xs">
        <div class="card-body">
          <div class="flex items-center gap-3">
            <div class="p-2.5 rounded-lg bg-primary/10 text-primary">
              <Building2 class="w-5 h-5" />
            </div>
            <div>
              <h1 class="text-lg font-semibold">
                {summary.office.name
                  ? appState.tr('homeWelcomeOffice', {
                      email: appState.currentUser?.email ?? '',
                      office: summary.office.name,
                    })
                  : appState.tr('homeWelcome', {
                      email: appState.currentUser?.email ?? '',
                    })}
              </h1>
              {#if summary.office.city}
                <p class="text-sm text-base-content/70">{summary.office.city}</p>
              {/if}
            </div>
          </div>
        </div>
      </div>

      <div class="card bg-base-100 border border-base-300 shadow-xs">
        <div class="card-body flex-row items-center gap-3">
          {#if summary.unpaid_medical_treatments_count > 0}
            <div class="p-2.5 rounded-lg bg-warning/15 text-warning">
              <AlertCircle class="w-5 h-5" />
            </div>
            <a
              href="#/medical-treatments?status=0"
              class="link link-primary font-medium text-base"
              onclick={(e) => {
                e.preventDefault();
                appState.navigate('/medical-treatments?status=0');
              }}
            >
              {summary.unpaid_medical_treatments_count === 1
                ? appState.tr('homeUnpaidSingular')
                : appState.tr('homeUnpaidPlural', {
                    count: summary.unpaid_medical_treatments_count,
                  })}
            </a>
          {:else}
            <div class="p-2.5 rounded-lg bg-success/15 text-success">
              <CheckCircle2 class="w-5 h-5" />
            </div>
            <span class="text-base-content/80 text-sm font-medium">
              {appState.tr('homeAllPaid')}
            </span>
          {/if}
        </div>
      </div>
    </div>
  {/if}
</div>
