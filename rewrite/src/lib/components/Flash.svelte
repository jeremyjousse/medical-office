<script lang="ts">
  import { X, CheckCircle2, AlertCircle, Info, AlertTriangle } from 'lucide-svelte';
  import { appState, type FlashType } from '../state.svelte';

  function alertClass(type: FlashType): string {
    switch (type) {
      case 'success':
        return 'alert-success';
      case 'error':
        return 'alert-error';
      case 'warning':
        return 'alert-warning';
      case 'info':
      default:
        return 'alert-info';
    }
  }
</script>

{#if appState.flashes.length > 0}
  <div class="max-w-7xl mx-auto px-4 pt-4 space-y-2" role="status" aria-live="polite">
    {#each appState.flashes as flash (flash.id)}
      <div class={`alert ${alertClass(flash.type)} shadow-sm flex items-center justify-between`}>
        <div class="flex items-center gap-2">
          {#if flash.type === 'success'}
            <CheckCircle2 class="w-5 h-5 shrink-0" />
          {:else if flash.type === 'error'}
            <AlertCircle class="w-5 h-5 shrink-0" />
          {:else if flash.type === 'warning'}
            <AlertTriangle class="w-5 h-5 shrink-0" />
          {:else}
            <Info class="w-5 h-5 shrink-0" />
          {/if}
          <span>{flash.message}</span>
        </div>
        <button
          type="button"
          class="btn btn-ghost btn-xs btn-square"
          aria-label={appState.tr('close')}
          onclick={() => appState.dismissFlash(flash.id)}
        >
          <X class="w-4 h-4" />
        </button>
      </div>
    {/each}
  </div>
{/if}
