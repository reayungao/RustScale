<script lang="ts">
  import { appState } from "$lib/state.svelte";
  import DropZone from "$lib/components/DropZone.svelte";
  import ImageComparison from "$lib/components/ImageComparison.svelte";
  import BatchView from "$lib/components/BatchView.svelte";
</script>

<div
  class="w-full h-full bg-(--bg-canvas) relative transition-colors duration-200"
>
  {#if appState.batchMode}
    <div class="absolute inset-0 p-8">
      <BatchView />
    </div>
  {:else if !appState.currentImage}
    <div class="absolute inset-0 p-8">
      <DropZone />
    </div>
  {:else}
    <ImageComparison />

    <!-- Close Button (Floating) -->
    {#if appState.status !== "processing"}
      <button
        class="absolute top-4 right-4 p-2 bg-(--bg-surface) text-(--text-secondary) hover:text-(--text-primary) rounded-full backdrop-blur border border-(--border-main) z-50 transition-all duration-200 ease-out"
        onclick={() => appState.clearImage()}
        title="Close Image (Esc)"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="h-5 w-5"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M6 18L18 6M6 6l12 12"
          />
        </svg>
      </button>
    {/if}
  {/if}
</div>
