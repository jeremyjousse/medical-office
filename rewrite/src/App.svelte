<script lang="ts">
  import { Stethoscope } from 'lucide-svelte';

  let ready = $state(true);
</script>

<main class="min-h-screen bg-base-200 flex items-center justify-center p-6">
  <div class="card bg-base-100 shadow-sm border border-base-300 max-w-md w-full">
    <div class="card-body items-center text-center">
      <Stethoscope class="w-10 h-10 text-primary" />
      <h1 class="card-title text-xl">Medical Office</h1>
      <p class="text-sm text-base-content/70">
        {ready ? 'Tauri + Svelte 5 + Tailwind v4 + DaisyUI v5 initialized.' : ''}
      </p>
    </div>
  </div>
</main>
