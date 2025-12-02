<script lang="ts">
    import { appState } from "../state.svelte";
    import { onMount, onDestroy } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    let gpuName = $state("Detecting...");
    let detectionMethod = $state("");
    let isGpuVram = $state(false);
    let isNpu = $state(false);
    let intervalId: any;

    async function fetchSystemInfo() {
        try {
            const info: any = await invoke("get_system_info");
            appState.vramUsage = info.vram_usage;
            appState.totalVram = info.total_vram;
            gpuName = info.gpu_name;
            detectionMethod = info.detection_method;
            detectionMethod = info.detection_method;
            isGpuVram = info.is_gpu_vram;
            isNpu = info.is_npu;
        } catch (e) {
            console.error("Failed to get system info", e);
            gpuName = "Detection Failed";
        }
    }

    onMount(() => {
        fetchSystemInfo();
        // Poll every 3 seconds for live VRAM usage
        intervalId = setInterval(fetchSystemInfo, 3000);
    });

    onDestroy(() => {
        if (intervalId) clearInterval(intervalId);
    });

    // Calculate percentage
    let usagePercent = $derived(
        (appState.vramUsage / Math.max(appState.totalVram, 1)) * 100,
    );

    // Get appropriate label
    let memoryLabel = $derived(
        isNpu ? "NPU Memory" : isGpuVram ? "GPU VRAM" : "System Memory",
    );

    // Health Color
    let healthColor = $derived.by(() => {
        if (usagePercent >= 90) return "bg-red-500";
        if (usagePercent >= 75) return "bg-orange-500";
        return "bg-[var(--accent-primary)]";
    });
</script>

<div
    class="p-4 bg-(--bg-surface) rounded-lg border border-(--border-main) transition-colors duration-200"
>
    <h3
        class="text-xs font-bold text-(--text-secondary) uppercase mb-3 flex justify-between items-center"
    >
        <span>System Status</span>
        <div class="relative group">
            <span
                class="text-[10px] px-2 py-1 rounded border font-bold uppercase tracking-wider cursor-help transition-colors duration-200"
                class:bg-[var(--accent-primary)]={true}
                class:text-white={true}
                class:border-[var(--accent-primary)]={true}
                class:bg-opacity-20={true}
                class:border-opacity-20={true}
            >
                {appState.optimizationMode}
            </span>

            <!-- Tooltip -->
            <div
                class="absolute right-0 top-6 w-48 p-2 rounded-lg bg-zinc-800 border border-zinc-700 shadow-xl text-xs text-zinc-300 opacity-0 group-hover:opacity-100 transition-opacity duration-200 pointer-events-none z-50"
            >
                {#if appState.optimizationMode === "Performance"}
                    <div class="font-bold text-(--accent-primary) mb-1">
                        Performance Mode
                    </div>
                    <div>&gt; 12GB Free VRAM.</div>
                    <div class="text-zinc-500 mt-1">Max Speed (Batch 4).</div>
                {:else if appState.optimizationMode === "Balanced"}
                    <div class="font-bold text-(--accent-primary) mb-1">
                        Balanced Mode
                    </div>
                    <div>6-12GB Free VRAM.</div>
                    <div class="text-zinc-500 mt-1">
                        Stable Speed (Batch 2).
                    </div>
                {:else}
                    <div class="font-bold text-(--accent-primary) mb-1">
                        Efficiency Mode
                    </div>
                    <div>&lt; 6GB Free VRAM.</div>
                    <div class="text-zinc-500 mt-1">
                        Low Memory (Batch 1) to prevent crashes.
                    </div>
                {/if}
            </div>
        </div>
    </h3>

    <!-- GPU Name -->
    <div class="flex justify-between items-center mb-2">
        <span class="text-xs text-(--text-secondary)">GPU</span>
        <span
            class="text-xs font-mono text-(--text-primary) truncate max-w-[180px]"
            title={gpuName}
        >
            {gpuName}
        </span>
    </div>

    <!-- Memory Usage -->
    <div class="flex justify-between items-center mb-1.5">
        <span class="text-sm text-(--text-primary)">{memoryLabel}</span>
        <span class="text-xs font-mono text-(--text-secondary)">
            {appState.vramUsage} / {appState.totalVram} MB
        </span>
    </div>

    <!-- Progress Bar -->
    <div
        class="w-full bg-zinc-800 dark:bg-zinc-900 rounded-full h-2 overflow-hidden mb-2"
    >
        <div
            class="h-2 rounded-full transition-all duration-500 {healthColor}"
            style="width: {usagePercent}%"
        ></div>
    </div>

    <!-- Detection Method (small note) -->
    {#if detectionMethod}
        <div class="text-[10px] text-(--text-secondary) opacity-60 mb-2">
            via {detectionMethod}
        </div>
    {/if}
</div>
