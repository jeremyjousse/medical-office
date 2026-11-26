<script lang="ts">
  import { ArrowLeft, Eye, Pencil, Plus, Trash2 } from 'lucide-svelte';
  import {
    deletePayment,
    getTreatment,
    parseApiError,
    type MedicalTreatmentDetail,
    type Payment,
  } from '../api';
  import Modal from '../components/Modal.svelte';
  import PaymentForm from '../components/PaymentForm.svelte';
  import { formatEur, locationLabel, paymentTypeLabel, treatmentStatusLabel } from '../i18n';
  import { appState } from '../state.svelte';

  interface Props {
    id: number;
  }

  let { id }: Props = $props();

  let detail = $state<MedicalTreatmentDetail | null>(null);
  let loading = $state(false);
  let paymentModalOpen = $state(false);
  let editingPaymentId = $state<number | undefined>(undefined);
  let deleteTarget = $state<Payment | null>(null);
  let deleting = $state(false);

  async function refreshDetail(targetId: number) {
    try {
      detail = await getTreatment(targetId);
    } catch (err) {
      appState.addFlash('error', parseApiError(err, appState.locale).message);
    }
  }

  $effect(() => {
    const activeUserId = appState.currentUser?.id;
    const currentId = id;
    if (!appState.isInitialized || !activeUserId || !currentId) return;

    let cancelled = false;
    loading = true;
    paymentModalOpen = false;
    editingPaymentId = undefined;
    deleteTarget = null;

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

  function treatmentSummaryLabel(d: MedicalTreatmentDetail): string {
    const tr = d.treatment;
    const name = [tr.patient_last_name, tr.patient_first_name].filter(Boolean).join(' ').trim();
    const tInfo = [tr.treatment_type_name, tr.date ? tr.date.slice(0, 10) : '']
      .filter(Boolean)
      .join(' ');
    return name ? `${name} (${tInfo})` : tInfo;
  }

  function openAddPaymentModal() {
    editingPaymentId = undefined;
    paymentModalOpen = true;
  }

  function openEditPaymentModal(payId: number) {
    editingPaymentId = payId;
    paymentModalOpen = true;
  }

  async function handlePaymentSaved() {
    const wasEdit = editingPaymentId !== undefined;
    paymentModalOpen = false;
    editingPaymentId = undefined;
    appState.addFlash(
      'success',
      appState.tr(wasEdit ? 'paymentUpdatedSuccess' : 'paymentCreatedSuccess'),
    );
    await refreshDetail(id);
  }

  async function confirmDeletePayment() {
    if (!deleteTarget) return;
    deleting = true;
    try {
      await deletePayment(deleteTarget.id);
      deleteTarget = null;
      appState.addFlash('success', appState.tr('paymentDeletedSuccess'));
      await refreshDetail(id);
    } catch (err) {
      deleteTarget = null;
      appState.addFlash('error', parseApiError(err, appState.locale).message);
    } finally {
      deleting = false;
    }
  }
</script>

<div class="max-w-3xl mx-auto px-4 py-6">
  <div class="card bg-base-100 border border-base-300 shadow-xs">
    <div class="card-body">
      <div class="flex flex-wrap items-center justify-between gap-2 border-b border-base-200 pb-3 mb-4">
        <h1 class="text-xl font-bold">{appState.tr('treatmentDetailTitle')}</h1>
        <div class="flex flex-wrap items-center gap-2">
          {#if detail}
            <button
              type="button"
              class="btn btn-sm btn-primary"
              onclick={openAddPaymentModal}
            >
              <Plus class="w-4 h-4" />
              {appState.tr('treatmentAddPayment')}
            </button>
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
                <td class="font-mono">{formatEur(tr.price)}</td>
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
                    <th></th>
                    <th>{appState.tr('treatmentPaymentPaidAt')}</th>
                    <th>{appState.tr('treatmentPaymentType')}</th>
                    <th class="text-right">{appState.tr('treatmentPaymentAmount')}</th>
                    <th>{appState.tr('actions')}</th>
                  </tr>
                </thead>
                <tbody>
                  {#each detail.payments as pay (pay.id)}
                    <tr>
                      <td>
                        <button
                          type="button"
                          class="btn btn-xs btn-outline"
                          onclick={() => appState.navigate(`/payments/${pay.id}`)}
                        >
                          <Eye class="w-3.5 h-3.5" />
                          {appState.tr('show')}
                        </button>
                      </td>
                      <td>{pay.paid_at ? pay.paid_at.slice(0, 10) : ''}</td>
                      <td>{paymentTypeLabel(appState.locale, pay.payment_type)}</td>
                      <td class="text-right font-mono">{formatEur(pay.amount)}</td>
                      <td class="whitespace-nowrap">
                        <div class="flex items-center gap-1">
                          <button
                            type="button"
                            class="btn btn-xs btn-ghost"
                            onclick={() => openEditPaymentModal(pay.id)}
                          >
                            <Pencil class="w-3.5 h-3.5" />
                            {appState.tr('edit')}
                          </button>
                          <button
                            type="button"
                            class="btn btn-xs btn-error btn-outline"
                            disabled={pay.bank_deposit_id !== null}
                            onclick={() => {
                              if (pay.bank_deposit_id === null) deleteTarget = pay;
                            }}
                          >
                            <Trash2 class="w-3.5 h-3.5" />
                            {appState.tr('destroy')}
                          </button>
                        </div>
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

<Modal
  open={paymentModalOpen}
  title={editingPaymentId ? appState.tr('paymentEditTitle') : appState.tr('paymentNewTitle')}
  boxClass="max-w-2xl"
  hideActions
  onCancel={() => {
    paymentModalOpen = false;
    editingPaymentId = undefined;
  }}
>
  {#if paymentModalOpen && detail}
    <PaymentForm
      paymentId={editingPaymentId}
      fixedMedicalTreatmentId={detail.treatment.id}
      fixedTreatmentLabel={treatmentSummaryLabel(detail)}
      defaultAmount={detail.treatment.price}
      defaultAccountOwner={detail.treatment.patient_last_name}
      onSaved={handlePaymentSaved}
      onCancel={() => {
        paymentModalOpen = false;
        editingPaymentId = undefined;
      }}
    />
  {/if}
</Modal>

<Modal
  open={deleteTarget !== null}
  title={appState.tr('paymentDeleteConfirmTitle')}
  confirmLabel={appState.tr('destroy')}
  confirmVariant="error"
  loading={deleting}
  onConfirm={confirmDeletePayment}
  onCancel={() => (deleteTarget = null)}
>
  <p>{appState.tr('paymentDeleteConfirmBody')}</p>
</Modal>
