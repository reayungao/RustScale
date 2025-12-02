<script lang="ts">
    import { appState } from "../state.svelte";
    import { fade, fly } from "svelte/transition";
</script>

<div class="fixed bottom-4 right-4 z-50 flex flex-col gap-2 items-end">
    {#each appState.toasts as toast (toast.id)}
        <div
            in:fly={{ y: 20, duration: 300 }}
            out:fade
            class="px-4 py-3 rounded-lg shadow-lg text-white font-medium min-w-[200px] flex items-center gap-2"
            class:bg-green-600={toast.type === "success"}
            class:bg-red-600={toast.type === "error"}
            class:bg-blue-600={toast.type === "info"}
        >
            <span>{toast.message}</span>
        </div>
    {/each}
</div>
