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
        <div class="flex items-center gap-2">
            <!-- Loading Pulse -->
            {#if appState.isModelLoading}
                <div
                    class="w-1.5 h-1.5 rounded-full bg-(--accent-primary) animate-pulse"
                    title="Loading Engine..."
                ></div>
            {/if}

            <div class="relative group flex items-center">
                <div class="relative">
                    <select
                        bind:value={appState.config.executionProvider}
                        onchange={() =>
                            appState.updateConfig({
                                executionProvider:
                                    appState.config.executionProvider,
                            })}
                        class="text-[10px] pl-2 pr-5 py-1 rounded border font-bold uppercase tracking-wider cursor-pointer transition-colors duration-200 bg-(--bg-surface) border-(--border-main) text-(--text-primary) hover:border-(--accent-primary) focus:outline-none focus:border-(--accent-primary) appearance-none disabled:opacity-50 disabled:cursor-not-allowed"
                        disabled={appState.isModelLoading}
                    >
                        <option value="auto">Auto</option>
                        <option value="directml">DirectML</option>
                        <option value="openvino">OpenVINO</option>
                        <option value="cpu">CPU</option>
                    </select>
                    <!-- Custom Arrow -->
                    <div
                        class="absolute right-1.5 top-1/2 -translate-y-1/2 pointer-events-none text-(--text-secondary) opacity-70"
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="10"
                            height="10"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            ><path d="m6 9 6 6 6-6" /></svg
                        >
                    </div>
                </div>

                <!-- Tooltip (Question Mark) -->
                <div class="relative group/tooltip ml-2">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="12"
                        height="12"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="text-(--text-secondary) cursor-help opacity-50 hover:opacity-100 transition-opacity"
                        ><circle cx="12" cy="12" r="10" /><path
                            d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"
                        /><path d="M12 17h.01" /></svg
                    >

                    <div
                        class="absolute right-0 top-6 w-64 p-3 rounded-lg bg-zinc-800 border border-zinc-700 shadow-xl text-xs text-zinc-300 opacity-0 group-hover/tooltip:opacity-100 transition-opacity duration-200 pointer-events-none z-50"
                    >
                        <div
                            class="font-bold text-(--accent-primary) mb-2 border-b border-zinc-700 pb-1"
                        >
                            Execution Provider
                        </div>
                        <div class="space-y-2">
                            <div>
                                <span class="font-bold text-white block"
                                    >Auto</span
                                >
                                <span class="text-zinc-400"
                                    >Smart selection. Uses OpenVINO for Intel
                                    GPUs, DirectML for others.</span
                                >
                            </div>
                            <div>
                                <span class="font-bold text-white block"
                                    >DirectML</span
                                >
                                <span class="text-zinc-400"
                                    >Standard GPU acceleration. Works on most
                                    GPUs (NVIDIA, AMD, Intel).</span
                                >
                            </div>
                            <div>
                                <span class="font-bold text-white block"
                                    >OpenVINO</span
                                >
                                <span class="text-zinc-400"
                                    >Highly optimized for Intel GPUs & NPUs.</span
                                >
                            </div>
                            <div>
                                <span class="font-bold text-white block"
                                    >CPU</span
                                >
                                <span class="text-zinc-400"
                                    >Slow software fallback. Use only for
                                    debugging.</span
                                >
                            </div>
                        </div>
                    </div>
                </div>
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
