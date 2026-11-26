<script lang="ts">
  import { ChevronDown, X } from 'lucide-svelte';
  import { finderPatients, getPatient, type PatientChoice } from '../api';
  import { appState } from '../state.svelte';

  interface Props {
    id?: string;
    value: number | null;
    placeholder?: string;
    disabled?: boolean;
    error?: string;
    compact?: boolean;
    onChange?: (value: number | null) => void;
  }

  let {
    id,
    value = $bindable<number | null>(null),
    placeholder,
    disabled = false,
    error,
    compact = false,
    onChange,
  }: Props = $props();

  let open = $state(false);
  let query = $state('');
  let loading = $state(false);
  let options = $state<PatientChoice[]>([]);
  let selectedLabel = $state<string | null>(null);
  let highlightedIndex = $state(0);
  let containerEl = $state<HTMLDivElement | null>(null);
  let searchInputEl = $state<HTMLInputElement | null>(null);
  let debounceTimer: ReturnType<typeof setTimeout> | undefined;

  $effect(() => {
    const currentId = value;
    if (currentId === null || currentId === undefined) {
      selectedLabel = null;
      return;
    }
    const match = options.find((o) => o.id === currentId);
    if (match) {
      selectedLabel = match.label;
      return;
    }
    let cancelled = false;
    getPatient(currentId)
      .then((p) => {
        if (cancelled) return;
        const name = [p.first_name, p.last_name].filter(Boolean).join(' ').trim();
        selectedLabel = p.city ? `${name} (${p.city})` : name || `#${p.id}`;
      })
      .catch(() => {
        if (!cancelled) selectedLabel = `#${currentId}`;
      });
    return () => {
      cancelled = true;
    };
  });

  $effect(() => {
    if (!open) return;
    const handleOutside = (e: MouseEvent) => {
      if (containerEl && !containerEl.contains(e.target as Node)) {
        open = false;
      }
    };
    window.addEventListener('mousedown', handleOutside);
    return () => window.removeEventListener('mousedown', handleOutside);
  });

  async function runSearch(q: string) {
    loading = true;
    try {
      options = await finderPatients(q, 25);
      highlightedIndex = 0;
    } catch {
      options = [];
    } finally {
      loading = false;
    }
  }

  function openDropdown() {
    if (disabled) return;
    open = true;
    query = '';
    highlightedIndex = 0;
    runSearch('');
    queueMicrotask(() => searchInputEl?.focus());
  }

  function handleQueryInput() {
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      runSearch(query);
    }, 180);
  }

  function selectOption(opt: PatientChoice | null) {
    const nextVal = opt ? opt.id : null;
    value = nextVal;
    selectedLabel = opt ? opt.label : null;
    onChange?.(nextVal);
    open = false;
    query = '';
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (!open) {
      if (e.key === 'Enter' || e.key === ' ' || e.key === 'ArrowDown') {
        e.preventDefault();
        openDropdown();
      }
      return;
    }

    if (e.key === 'Escape') {
      e.preventDefault();
      open = false;
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      if (options.length > 0) {
        highlightedIndex = (highlightedIndex + 1) % options.length;
      }
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      if (options.length > 0) {
        highlightedIndex = (highlightedIndex - 1 + options.length) % options.length;
      }
    } else if (e.key === 'Enter') {
      e.preventDefault();
      const target = options[highlightedIndex];
      if (target) selectOption(target);
    }
  }
</script>

<div class="relative w-full" bind:this={containerEl}>
  <div
    class={`input input-bordered w-full flex items-center justify-between gap-1 cursor-pointer ${
      compact ? 'input-xs min-w-44' : ''
    } ${error ? 'input-error' : ''} ${disabled ? 'input-disabled pointer-events-none opacity-60' : ''}`}
  >
    <button
      {id}
      type="button"
      class="flex-1 h-full text-left truncate bg-transparent border-0 outline-none cursor-pointer"
      aria-expanded={open}
      aria-haspopup="listbox"
      {disabled}
      onclick={() => (open ? (open = false) : openDropdown())}
      onkeydown={handleKeyDown}
    >
      {#if selectedLabel}
        <span>{selectedLabel}</span>
      {:else}
        <span class="text-base-content/50">
          {placeholder ?? appState.tr('patientSearchPlaceholder')}
        </span>
      {/if}
    </button>

    <div class="flex items-center gap-0.5 shrink-0">
      {#if value !== null && !disabled}
        <button
          type="button"
          class="btn btn-ghost btn-xs btn-circle"
          aria-label={appState.tr('clearSelection')}
          onclick={(e) => {
            e.stopPropagation();
            selectOption(null);
          }}
        >
          <X class="w-3 h-3" />
        </button>
      {/if}
      <ChevronDown class="w-3.5 h-3.5 text-base-content/60" />
    </div>
  </div>

  {#if open}
    <div
      class="absolute z-50 mt-1 w-full min-w-64 bg-base-100 border border-base-300 rounded-box shadow-lg overflow-hidden"
    >
      <div class="p-2 border-b border-base-200">
        <input
          bind:this={searchInputEl}
          type="text"
          class="input input-bordered input-sm w-full"
          placeholder={appState.tr('patientSearchPlaceholder')}
          bind:value={query}
          oninput={handleQueryInput}
          onkeydown={handleKeyDown}
        />
      </div>

      <ul class="max-h-60 overflow-y-auto py-1" role="listbox">
        {#if loading}
          <li class="px-3 py-2 text-sm text-base-content/60 flex items-center gap-2">
            <span class="loading loading-spinner loading-xs"></span>
            {appState.tr('loading')}
          </li>
        {:else if options.length === 0}
          <li class="px-3 py-2 text-sm text-base-content/60">
            {appState.tr('noMatches')}
          </li>
        {:else}
          {#each options as opt, idx (opt.id)}
            <li role="option" aria-selected={opt.id === value}>
              <button
                type="button"
                class={`w-full text-left px-3 py-2 text-sm hover:bg-base-200 ${
                  idx === highlightedIndex ? 'bg-base-200' : ''
                } ${opt.id === value ? 'font-semibold text-primary' : ''}`}
                onclick={() => selectOption(opt)}
              >
                {opt.label}
              </button>
            </li>
          {/each}
        {/if}
      </ul>
    </div>
  {/if}
</div>