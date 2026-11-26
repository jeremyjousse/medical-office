<script lang="ts">
  import { ArrowLeft, Pencil } from 'lucide-svelte';
  import { getTreatment, parseApiError, type MedicalTreatmentDetail } from '../api';
  import { locationLabel, treatmentStatusLabel } from '../i18n';
  import { appState } from '../state.svelte';

  interface Props {
    id: number;
  }

  let { id }: Props = $props();

  let detail = $state<MedicalTreatmentDetail | null>(null);
  let loading = $state(false);

  $effect(() => {
    const activeUserId = appState.currentUser?.id;
    const currentId = id;
    if (!appState.isInitialized || !activeUserId || !currentId) return;

    let cancelled = false;
    loading = true;
    getTreatment(currentId)
      .then((res) => {
        if (!cancelled) detail = res;
      })
      .catch((err) => {
        if (cancelled) return;
        appState.addFlash('error', parseApiError(err, appState.locale).message);
        appState.navigate('/medical-treatments');
      })
      .finally(() => {
        if (!cancelled) loading = false;
      });

    return () => {
      cancelled = true;
    };
  });

  function patientLabel(d: MedicalTreatmentDetail): string {
    const tr = d.treatment;
    const name = [tr.patient_first_name, tr.patient_last_name].filter(Boolean).join(' ').trim();
    return tr.patient_city ? `${name} (${tr.patient_city})` : name || '—';
  }
</script>

<div class="max-w-3xl mx-auto px-4 py-6">
  <div class="card bg-base-100 border border-base-300 shadow-xs">
    <div class="card-body">
      <div class="flex items-center justify-between border-b border-base-200 pb-3 mb-4">
        <h1 class="text-xl font-bold">{appState.tr('treatmentDetailTitle')}</h1>
        <div class="flex items-center gap-2">
          {#if detail}
            <button
              type="button"
              class="btn btn-sm btn-outline"
              onclick={() => appState.navigate(`/medical-treatments/${detail!.treatment.id}/edit`)}
            >
              <Pencil class="w-4 h-4" />
              {appState.tr('edit')}
            </button>
          {/if}
          <button
            type="button"
            class="btn btn-sm btn-ghost"
            onclick={() => appState.navigate('/medical-treatments')}
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
      {:else if detail}
        {@const tr = detail.treatment}
        <div class="overflow-x-auto">
          <table class="table table-bordered">
            <tbody>
              <tr>
                <th class="w-52 bg-base-200/50">{appState.tr('treatmentPatient')}</th>
                <td>
                  {#if tr.patient_id}
                    <a
                      href={`#/patients/${tr.patient_id}`}
                      class="link link-primary"
                      onclick={(e) => {
                        e.preventDefault();
                        appState.navigate(`/patients/${tr.patient_id}`);
                      }}
                    >
                      {patientLabel(detail)}
                    </a>
                  {:else}
                    {patientLabel(detail)}
                  {/if}
                </td>
              </tr>
              <tr>
                <th class="bg-base-200/50">{appState.tr('treatmentType')}</th>
                <td>{tr.treatment_type_name ?? '—'}</td>
              </tr>
              <tr>
                <th class="bg-base-200/50">{appState.tr('treatmentDate')}</th>
                <td>{tr.date ?? ''}</td>
              </tr>
              <tr>
                <th class="bg-base-200/50">{appState.tr('treatmentLocation')}</th>
                <td>{locationLabel(appState.locale, tr.location_id)}</td>
              </tr>
              <tr>
                <th class="bg-base-200/50">{appState.tr('treatmentPrice')}</th>
                <td>
                  {tr.price !== null && tr.price !== undefined ? tr.price.toFixed(2) : '—'}
                </td>
              </tr>
              <tr>
                <th class="bg-base-200/50">{appState.tr('treatmentNote')}</th>
                <td class="whitespace-pre-wrap">{tr.note ?? ''}</td>
              </tr>
              <tr>
                <th class="bg-base-200/50">{appState.tr('treatmentStatus')}</th>
                <td>
                  <span class={`badge badge-sm ${tr.status === 1 ? 'badge-success' : 'badge-warning'}`}>
                    {treatmentStatusLabel(appState.locale, tr.status)}
                  </span>
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        {#if detail.payments.length > 0}
          <div class="mt-6">
            <h2 class="text-base font-semibold mb-2">{appState.tr('treatmentPaymentsHeading')}</h2>
            <div class="overflow-x-auto">
              <table class="table table-zebra table-sm border border-base-200">
                <thead>
                  <tr>
                    <th>{appState.tr('treatmentPaymentPaidAt')}</th>
                    <th>{appState.tr('treatmentPaymentType')}</th>
                    <th>{appState.tr('treatmentPaymentAmount')}</th>
                  </tr>
                </thead>
                <tbody>
                  {#each detail.payments as pay (pay.id)}
                    <tr>
                      <td>{pay.paid_at ?? ''}</td>
                      <td>{pay.payment_type ?? '—'}</td>
                      <td>
                        {pay.amount !== null && pay.amount !== undefined
                          ? pay.amount.toFixed(2)
                          : '—'}
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          </div>
        {/if}
      {/if}
    </div>
  </div>
</div>