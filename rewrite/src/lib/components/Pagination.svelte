<script lang="ts">
  import { ChevronLeft, ChevronRight } from 'lucide-svelte';
  import { appState } from '../state.svelte';

  interface Props {
    page: number;
    perPage: number;
    filteredTotal: number;
    total: number;
    label: string;
    onPageChange: (page: number) => void;
  }

  let { page, perPage, filteredTotal, total, label, onPageChange }: Props = $props();

  let totalPages = $derived(Math.max(1, Math.ceil(filteredTotal / Math.max(1, perPage))));

  let visiblePages = $derived.by(() => {
    const pages: (number | 'ellipsis')[] = [];
    if (totalPages <= 7) {
      for (let i = 1; i <= totalPages; i++) pages.push(i);
      return pages;
    }
    pages.push(1);
    if (page > 3) pages.push('ellipsis');
    const start = Math.max(2, page - 1);
    const end = Math.min(totalPages - 1, page + 1);
    for (let i = start; i <= end; i++) pages.push(i);
    if (page < totalPages - 2) pages.push('ellipsis');
    pages.push(totalPages);
    return pages;
  });
</script>

<div class="flex flex-col sm:flex-row items-center justify-between gap-3 py-2">
  <div class="text-sm text-base-content/70">
    {appState.tr('selectedOfTotal', {
      selected: filteredTotal,
      label,
      total,
    })}
  </div>

  {#if totalPages > 1}
    <div class="join">
      <button
        type="button"
        class="join-item btn btn-sm"
        disabled={page <= 1}
        onclick={() => onPageChange(page - 1)}
      >
        <ChevronLeft class="w-4 h-4" />
        {appState.tr('previous')}
      </button>

      {#each visiblePages as item, idx (idx)}
        {#if item === 'ellipsis'}
          <span class="join-item btn btn-sm btn-disabled">…</span>
        {:else}
          <button
            type="button"
            class={`join-item btn btn-sm ${item === page ? 'btn-active btn-primary' : ''}`}
            onclick={() => onPageChange(item)}
          >
            {item}
          </button>
        {/if}
      {/each}

      <button
        type="button"
        class="join-item btn btn-sm"
        disabled={page >= totalPages}
        onclick={() => onPageChange(page + 1)}
      >
        {appState.tr('next')}
        <ChevronRight class="w-4 h-4" />
      </button>
    </div>
  {/if}
</div>
