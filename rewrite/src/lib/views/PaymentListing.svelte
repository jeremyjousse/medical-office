<script lang="ts">
  import { untrack } from 'svelte';
  import { ArrowLeft, Printer, Search, RotateCcw } from 'lucide-svelte';
  import {
    listPaymentListing,
    parseApiError,
    type PaymentListingResult,
    type PaymentView,
  } from '../api';
  import { formatEur, paymentTypeLabel, paymentTypeOptions } from '../i18n';
  import { appState, defaultPaymentListingFilters } from '../state.svelte';

  let listing = $state<PaymentListingResult | null>(null);
  let loading = $state(false);

  let typeOptions = $derived(paymentTypeOptions(appState.locale));

  async function loadListing() {
    loading = true;
    try {
      const f = appState.paymentListingFilters;
      listing = await listPaymentListing({
        paid_at_gteq: f.paid_at_gteq || undefined,
        paid_at_lteq: f.paid_at_lteq || undefined,
        payment_type: f.payment_type,
      });
    } catch (err) {
      appState.addFlash('error', parseApiError(err, appState.locale).message);
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    const activeUserId = appState.currentUser?.id;
    if (!appState.isInitialized || !activeUserId) return;
    untrack(() => {
      loadListing();
    });
  });

  function handleFilterSubmit(e: SubmitEvent) {
    e.preventDefault();
    loadListing();
  }

  function handleResetFilters() {
    appState.paymentListingFilters = defaultPaymentListingFilters();
    loadListing();
  }

  function formatTreatmentLabel(p: PaymentView): string {
    const name = [p.patient_last_name, p.patient_first_name].filter(Boolean).join(' ').trim();
    const tInfo = [p.treatment_type_name, p.treatment_date ? p.treatment_date.slice(0, 10) : '']
      .filter(Boolean)
      .join(' ');
    if (name && tInfo) return `${name} (${tInfo})`;
    if (name) return name;
    if (tInfo) return tInfo;
    return p.medical_treatment_id ? `#${p.medical_treatment_id}` : '—';
  }
</script>

<div class="max-w-7xl mx-auto px-4 py-6">
  <div class="card bg-base-100 border border-base-300 shadow-xs">
    <div class="card-body">
      <div class="flex flex-wrap items-center justify-between gap-3 border-b border-base-200 pb-3">
        <h1 class="text-xl font-bold">{appState.tr('paymentListingTitle')}</h1>
        <div class="flex items-center gap-2 no-print">
          <button
            type="button"
            class="btn btn-ghost btn-sm"
            onclick={() => appState.navigate('/payments')}
          >
            <ArrowLeft class="w-4 h-4" />
            {appState.tr('back')}
          </button>
          <button
            type="button"
            class="btn btn-primary btn-sm"
            onclick={() => window.print()}
          >
            <Printer class="w-4 h-4" />
            {appState.tr('print')}
          </button>
        </div>
      </div>

      {#if listing}
        <div class="text-sm text-base-content/70 py-1 no-print">
          {appState.tr('selectedOfTotal', {
            selected: listing.filtered_total,
            label: appState.tr('paymentPlural'),
            total: listing.total,
          })}
        </div>
      {/if}

      <form onsubmit={handleFilterSubmit} class="overflow-x-auto">
        <table class="table table-sm border border-base-200">
          <thead>
            <tr>
              <th>{appState.tr('paymentMedicalTreatment')}</th>
              <th>{appState.tr('paymentPaidAt')}</th>
              <th>{appState.tr('paymentType')}</th>
              <th class="text-right">{appState.tr('paymentAmount')}</th>
              <th class="no-print"></th>
            </tr>
            <tr class="bg-base-200/60 no-print">
              <td></td>
              <td>
                <div class="flex items-center gap-1 min-w-60">
                  <input
                    type="date"
                    class="input input-bordered input-xs w-full"
                    aria-label="Paid at from"
                    bind:value={appState.paymentListingFilters.paid_at_gteq}
                  />
                  <span class="text-xs text-base-content/60">{appState.tr('toRange')}</span>
                  <input
                    type="date"
                    class="input input-bordered input-xs w-full"
                    aria-label="Paid at to"
                    bind:value={appState.paymentListingFilters.paid_at_lteq}
                  />
                </div>
              </td>
              <td>
                <select
                  class="select select-bordered select-xs w-full min-w-36"
                  aria-label={appState.tr('paymentType')}
                  bind:value={appState.paymentListingFilters.payment_type}
                >
                  <option value={null}>{appState.tr('paymentAllTypes')}</option>
                  {#each typeOptions as opt (opt.value)}
                    <option value={opt.value}>{opt.label}</option>
                  {/each}
                </select>
              </td>
              <td></td>
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
                <td colspan="5" class="text-center py-8">
                  <span class="loading loading-spinner loading-md text-primary"></span>
                </td>
              </tr>
            {:else if listing && listing.days.length === 0}
              <tr>
                <td colspan="5" class="text-center py-6 text-base-content/60">
                  {appState.tr('noRecords')}
                </td>
              </tr>
            {:else if listing}
              {#each listing.days as day (day.date)}
                {#each day.payments as item (item.id)}
                  <tr class="hover">
                    <td>{formatTreatmentLabel(item)}</td>
                    <td>{item.paid_at ? item.paid_at.slice(0, 10) : ''}</td>
                    <td>{paymentTypeLabel(appState.locale, item.payment_type)}</td>
                    <td class="text-right font-mono">{formatEur(item.amount)}</td>
                    <td class="no-print"></td>
                  </tr>
                {/each}
                <tr class="bg-base-200/50 font-semibold">
                  <td colspan="5" class="text-right">
                    <span>
                      {appState.tr('paymentListingDayTotal')} ({day.date}) — {appState.tr('paymentListingAll')} :
                      <span class="font-mono">{formatEur(day.total_amount)}</span>
                    </span>
                    {#each day.totals_by_type as tt (tt.payment_type)}
                      <span class="ml-3">
                        · {paymentTypeLabel(appState.locale, tt.payment_type)} :
                        <span class="font-mono">{formatEur(tt.amount)}</span>
                      </span>
                    {/each}
                  </td>
                </tr>
              {/each}

              {#if listing.filtered_total > 0}
                <tr class="bg-base-300/60 font-bold text-base">
                  <td colspan="5" class="text-right">
                    <span>
                      {appState.tr('paymentListingGrandTotal')} — {appState.tr('paymentListingAll')} :
                      <span class="font-mono">{formatEur(listing.grand_total_amount)}</span>
                    </span>
                    {#each listing.grand_totals_by_type as gt (gt.payment_type)}
                      <span class="ml-3">
                        · {paymentTypeLabel(appState.locale, gt.payment_type)} :
                        <span class="font-mono">{formatEur(gt.amount)}</span>
                      </span>
                    {/each}
                  </td>
                </tr>
              {/if}
            {/if}
          </tbody>
        </table>
      </form>
    </div>
  </div>
</div>
