<script lang="ts">
    import { appState } from "../state.svelte";
    import { fade } from "svelte/transition";
    import { flip } from "svelte/animate";

    function getFilename(path: string) {
        return path.split(/[\\/]/).pop() || "Image";
    }

    function clearAll() {
        if (appState.status === "processing") return;
        appState.queue = [];
        appState.currentImage = null; // Reset single mode too
    }

    function clearCompleted() {
        if (appState.status === "processing") return;
        appState.queue = appState.queue.filter(
            (item) => item.status !== "done" && item.status !== "error",
        );
        if (appState.queue.length === 0) {
            appState.currentImage = null;
        }
    }

    function remove(index: number) {
        if (appState.status === "processing") return;
        appState.queue = appState.queue.filter((_, i) => i !== index);
        if (appState.queue.length === 0) {
            appState.currentImage = null;
        }
    }
</script>

<div
    class="w-full h-full flex flex-col bg-(--bg-surface)/80 backdrop-blur-xl rounded-xl border border-(--border-main) overflow-hidden shadow-2xl transition-all duration-300"
    transition:fade
>
    <!-- Header -->
    <div
        class="flex items-center justify-between p-4 border-b border-(--border-main) bg-(--bg-surface-alt)/50"
    >
        <div class="flex items-center gap-3">
            <h2 class="text-lg font-bold text-(--text-primary) tracking-tight">
                Batch Queue
            </h2>
            <span
                class="px-2.5 py-0.5 text-xs font-bold rounded-full bg-(--accent-primary)/10 text-(--accent-primary) border border-(--accent-primary)/20"
            >
                {appState.queue.length} files
            </span>
        </div>
        <div class="flex items-center gap-2">
            {#if appState.status === "processing"}
                <button
                    class="px-3 py-1.5 text-sm font-medium text-white bg-red-500 hover:bg-red-600 rounded-lg shadow-lg shadow-red-500/20 transition-all active:scale-95"
                    onclick={() => appState.cancelJob()}
                >
                    Cancel Batch
                </button>
            {:else}
                <button
                    class="px-3 py-1.5 text-sm font-medium text-(--text-secondary) hover:text-(--text-primary) hover:bg-(--bg-highlight) rounded-lg transition-colors"
                    onclick={clearCompleted}
                >
                    Clear Completed
                </button>
                <button
                    class="px-3 py-1.5 text-sm font-medium text-red-400 hover:text-red-300 hover:bg-red-500/10 rounded-lg transition-colors"
                    onclick={clearAll}
                >
                    Clear All
                </button>
            {/if}
        </div>
    </div>

    <!-- List -->
    <div class="flex-1 overflow-y-auto p-3 space-y-2 custom-scrollbar">
        {#each appState.queue as item, i (item.path)}
            <div
                class="flex items-center gap-4 p-3 rounded-xl bg-(--bg-canvas)/50 border border-(--border-subtle) group hover:border-(--border-main) transition-all duration-200"
                animate:flip={{ duration: 300 }}
            >
                <!-- Icon/Status -->
                <div
                    class="w-10 h-10 flex items-center justify-center rounded-full shrink-0 transition-colors duration-300
                    {item.status === 'done'
                        ? 'bg-green-500/20 text-green-500'
                        : item.status === 'error'
                          ? 'bg-red-500/20 text-red-500'
                          : item.status === 'processing'
                            ? 'bg-(--accent-primary)/20 text-(--accent-primary)'
                            : 'bg-(--bg-surface) text-(--text-tertiary)'}"
                >
                    {#if item.status === "done"}
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-5 w-5"
                            viewBox="0 0 20 20"
                            fill="currentColor"
                        >
                            <path
                                fill-rule="evenodd"
                                d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                                clip-rule="evenodd"
                            />
                        </svg>
                    {:else if item.status === "error"}
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-5 w-5"
                            viewBox="0 0 20 20"
                            fill="currentColor"
                        >
                            <path
                                fill-rule="evenodd"
                                d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                                clip-rule="evenodd"
                            />
                        </svg>
                    {:else if item.status === "processing"}
                        <div
                            class="w-5 h-5 border-2 border-current border-t-transparent rounded-full animate-spin"
                        ></div>
                    {:else}
                        <span class="text-xs font-mono font-bold opacity-50"
                            >{i + 1}</span
                        >
                    {/if}
                </div>

                <!-- Info -->
                <div class="flex-1 min-w-0">
                    <div class="flex items-center justify-between mb-1.5">
                        <p
                            class="text-sm font-medium text-(--text-primary) truncate"
                            title={item.path}
                        >
                            {getFilename(item.path)}
                        </p>
                        {#if item.status === "error"}
                            <span
                                class="text-xs font-medium text-red-400 truncate max-w-[200px]"
                                title={item.error}>{item.error}</span
                            >
                        {:else if item.status === "processing"}
                            <span
                                class="text-xs font-medium text-(--accent-primary) animate-pulse"
                                >Processing...</span
                            >
                        {/if}
                    </div>

                    <!-- Progress Bar -->
                    <div
                        class="h-2 w-full bg-(--bg-surface) rounded-full overflow-hidden border border-(--border-subtle)"
                    >
                        <div
                            class="h-full transition-all duration-300 ease-out rounded-full shadow-sm
                                {item.status === 'done'
                                ? 'bg-green-500'
                                : item.status === 'error'
                                  ? 'bg-red-500'
                                  : item.status === 'processing'
                                    ? 'bg-linear-to-r from-(--accent-primary) to-blue-400'
                                    : 'bg-(--bg-highlight)'}"
                            style="width: {item.status === 'done'
                                ? 100
                                : item.status === 'processing'
                                  ? item.progress * 100
                                  : 0}%"
                        ></div>
                    </div>
                </div>

                <!-- Actions -->
                {#if appState.status !== "processing"}
                    <button
                        class="p-2 text-(--text-tertiary) hover:text-red-400 hover:bg-red-500/10 rounded-lg transition-all opacity-0 group-hover:opacity-100"
                        onclick={() => remove(i)}
                        title="Remove from queue"
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-4 w-4"
                            viewBox="0 0 20 20"
                            fill="currentColor"
                        >
                            <path
                                fill-rule="evenodd"
                                d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                                clip-rule="evenodd"
                            />
                        </svg>
                    </button>
                {/if}
            </div>
        {/each}
    </div>
</div>
