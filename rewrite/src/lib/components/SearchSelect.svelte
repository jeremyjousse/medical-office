<script lang="ts">
  import { ChevronDown, X } from 'lucide-svelte';
  import { appState } from '../state.svelte';

  export interface SelectOption {
    value: number;
    label: string;
  }

  interface Props {
    id?: string;
    options: SelectOption[];
    value: number | null;
    placeholder?: string;
    disabled?: boolean;
    error?: string;
    onChange?: (value: number | null) => void;
  }

  let {
    id,
    options,
    value = $bindable<number | null>(null),
    placeholder,
    disabled = false,
    error,
    onChange,
  }: Props = $props();

  let open = $state(false);
  let query = $state('');
  let highlightedIndex = $state(0);
  let containerEl = $state<HTMLDivElement | null>(null);
  let searchInputEl = $state<HTMLInputElement | null>(null);

  let selectedOption = $derived(options.find((o) => o.value === value) ?? null);

  let filteredOptions = $derived.by(() => {
    const q = query.trim().toLowerCase();
    if (!q) return options;
    return options.filter((o) => o.label.toLowerCase().includes(q));
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

  function openDropdown() {
    if (disabled) return;
    open = true;
    query = '';
    highlightedIndex = 0;
    queueMicrotask(() => searchInputEl?.focus());
  }

  function selectOption(opt: SelectOption | null) {
    const nextVal = opt ? opt.value : null;
    value = nextVal;
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
      if (filteredOptions.length > 0) {
        highlightedIndex = (highlightedIndex + 1) % filteredOptions.length;
      }
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      if (filteredOptions.length > 0) {
        highlightedIndex =
          (highlightedIndex - 1 + filteredOptions.length) % filteredOptions.length;
      }
    } else if (e.key === 'Enter') {
      e.preventDefault();
      const target = filteredOptions[highlightedIndex];
      if (target) selectOption(target);
    }
  }
</script>

<div class="relative w-full" bind:this={containerEl}>
  <div
    class={`input input-bordered w-full flex items-center justify-between gap-2 cursor-pointer ${
      error ? 'input-error' : ''
    } ${disabled ? 'input-disabled pointer-events-none opacity-60' : ''}`}
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
      {#if selectedOption}
        <span>{selectedOption.label}</span>
      {:else}
        <span class="text-base-content/50">
          {placeholder ?? appState.tr('searchPlaceholder')}
        </span>
      {/if}
    </button>

    <div class="flex items-center gap-1 shrink-0">
      {#if selectedOption && !disabled}
        <button
          type="button"
          class="btn btn-ghost btn-xs btn-circle"
          aria-label={appState.tr('clearSelection')}
          onclick={(e) => {
            e.stopPropagation();
            selectOption(null);
          }}
        >
          <X class="w-3.5 h-3.5" />
        </button>
      {/if}
      <ChevronDown class="w-4 h-4 text-base-content/60" />
    </div>
  </div>

  {#if open}
    <div
      class="absolute z-50 mt-1 w-full bg-base-100 border border-base-300 rounded-box shadow-lg overflow-hidden"
    >
      <div class="p-2 border-b border-base-200">
        <input
          bind:this={searchInputEl}
          type="text"
          class="input input-bordered input-sm w-full"
          placeholder={appState.tr('searchPlaceholder')}
          bind:value={query}
          oninput={() => (highlightedIndex = 0)}
          onkeydown={handleKeyDown}
        />
      </div>

      <ul class="max-h-60 overflow-y-auto py-1" role="listbox">
        {#if filteredOptions.length === 0}
          <li class="px-3 py-2 text-sm text-base-content/60">
            {appState.tr('noMatches')}
          </li>
        {:else}
          {#each filteredOptions as opt, idx (opt.value)}
            <li role="option" aria-selected={opt.value === value}>
              <button
                type="button"
                class={`w-full text-left px-3 py-2 text-sm hover:bg-base-200 ${
                  idx === highlightedIndex ? 'bg-base-200' : ''
                } ${opt.value === value ? 'font-semibold text-primary' : ''}`}
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
