<script lang="ts">
  import type { Snippet } from 'svelte';
  import { appState } from '../state.svelte';

  interface Props {
    open: boolean;
    title: string;
    confirmLabel?: string;
    cancelLabel?: string;
    confirmVariant?: 'primary' | 'error';
    loading?: boolean;
    children?: Snippet;
    onConfirm: () => void;
    onCancel: () => void;
  }

  let {
    open,
    title,
    confirmLabel,
    cancelLabel,
    confirmVariant = 'primary',
    loading = false,
    children,
    onConfirm,
    onCancel,
  }: Props = $props();

  let dialogEl = $state<HTMLDialogElement | null>(null);

  $effect(() => {
    const el = dialogEl;
    if (!el) return;
    if (open && !el.open) {
      el.showModal();
    } else if (!open && el.open) {
      el.close();
    }
  });

  function handleDialogCancel(e: Event) {
    e.preventDefault();
    if (!loading) {
      onCancel();
    }
  }
</script>

<dialog
  bind:this={dialogEl}
  class="modal"
  oncancel={handleDialogCancel}
  onclick={(e) => {
    if (e.target === dialogEl && !loading) onCancel();
  }}
>
  <div class="modal-box">
    <h3 class="font-bold text-lg">{title}</h3>
    <div class="py-4 text-sm text-base-content/80">
      {@render children?.()}
    </div>
    <div class="modal-action">
      <button type="button" class="btn btn-ghost" disabled={loading} onclick={onCancel}>
        {cancelLabel ?? appState.tr('cancel')}
      </button>
      <button
        type="button"
        class={confirmVariant === 'error' ? 'btn btn-error' : 'btn btn-primary'}
        disabled={loading}
        onclick={onConfirm}
      >
        {#if loading}
          <span class="loading loading-spinner loading-xs"></span>
        {/if}
        {confirmLabel ?? appState.tr('confirm')}
      </button>
    </div>
  </div>
</dialog>
