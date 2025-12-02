<script lang="ts">
    import { appState } from "../state.svelte";
    import SystemInfo from "./SystemInfo.svelte";
    import ModelEditor from "./ModelEditor.svelte";
    import { slide } from "svelte/transition";
    import { clickOutside } from "../actions/clickOutside";

    let editingModelId = $state<string | null>(null);

    const themes = [
        { id: "light", label: "Light", icon: "☀️" },
        { id: "dark", label: "Dark", icon: "🌙" },
        { id: "system", label: "System", icon: "⚙️" },
    ] as const;

    function handleNpuChange(e: Event) {
        const target = e.target as HTMLInputElement;
        appState.updateConfig({ preferNpu: target.checked });
    }
</script>

{#if editingModelId}
    <ModelEditor
        modelId={editingModelId}
        onClose={() => (editingModelId = null)}
    />
{/if}

<aside
    class="w-80 bg-(--bg-surface) border-r border-(--border-main) flex flex-col h-screen p-4 select-none transition-colors duration-200"
>
    <div class="mb-8">
        <h1
            class="text-2xl font-bold bg-linear-to-r from-(--accent-primary) to-orange-600 bg-clip-text text-transparent"
        >
            RustScale
        </h1>
        <p class="text-xs text-(--text-secondary)">Native AI Upscaler</p>
    </div>

    <div class="flex-1 overflow-y-auto space-y-6 pr-2">
        <!--Theme Selection -->
        <section>
            <h2
                class="text-xs font-bold text-(--text-secondary) uppercase mb-3"
            >
                Theme
            </h2>
            <div class="grid grid-cols-3 gap-2">
                {#each themes as theme}
                    <button
                        class="p-2 rounded-lg border text-xs font-medium transition-all duration-200 ease-out hover:scale-105"
                        class:border-(--accent-primary)={appState.theme ===
                            theme.id}
                        class:text-(--accent-primary)={appState.theme ===
                            theme.id}
                        class:bg-rust-950={appState.theme === theme.id}
                        class:dark:bg-rust-950={appState.theme === theme.id}
                        class:shadow-sm={appState.theme === theme.id}
                        class:border-(--border-main)={appState.theme !==
                            theme.id}
                        class:text-(--text-secondary)={appState.theme !==
                            theme.id}
                        class:hover:border-(--accent-primary)={appState.theme !==
                            theme.id}
                        class:hover:text-(--accent-primary)={appState.theme !==
                            theme.id}
                        onclick={() => appState.setTheme(theme.id)}
                    >
                        <div>{theme.icon}</div>
                        <div class="mt-1">{theme.label}</div>
                    </button>
                {/each}
            </div>
        </section>

        <!-- Model Selection -->
        <section>
            <h2
                class="text-xs font-bold text-(--text-secondary) uppercase mb-3"
            >
                AI Model
            </h2>
            <div
                class="relative"
                use:clickOutside={() =>
                    appState.activePopover === "model" &&
                    appState.togglePopover("model")}
            >
                <!-- Dropdown Trigger -->
                <button
                    class="w-full text-left p-3 rounded-lg border border-(--border-main) bg-(--bg-canvas)/50 hover:border-(--accent-primary) hover:shadow-sm transition-all duration-200 ease-out flex items-center justify-between group"
                    onclick={() => appState.togglePopover("model")}
                >
                    <div class="flex-1 min-w-0 pr-2">
                        {#if appState.models.find((m) => m.id === appState.config.model)}
                            {@const activeModel = appState.models.find(
                                (m) => m.id === appState.config.model,
                            )}
                            <div
                                class="font-medium text-sm text-(--accent-primary) truncate"
                            >
                                {activeModel?.name}
                            </div>
                            <div
                                class="text-xs text-(--text-secondary) truncate"
                            >
                                {activeModel?.description}
                            </div>
                        {:else}
                            <div class="text-sm text-(--text-secondary)">
                                Select Model
                            </div>
                        {/if}
                    </div>
                    <div
                        class="text-(--text-secondary) transition-transform duration-200"
                        class:rotate-180={appState.activePopover === "model"}
                    >
                        ▼
                    </div>
                </button>

                <!-- Dropdown Body -->
                {#if appState.activePopover === "model"}
                    <div
                        class="absolute top-full left-0 right-0 mt-2 bg-(--bg-surface) border border-(--border-main) rounded-lg shadow-xl z-50 overflow-hidden max-h-96 overflow-y-auto"
                        transition:slide={{ duration: 200 }}
                    >
                        {#each appState.models as model}
                            <div
                                class="w-full flex items-center border-b border-(--border-main) last:border-0 hover:bg-(--bg-canvas) transition-colors duration-150 group/item"
                                class:bg-orange-50={appState.config.model ===
                                    model.id && appState.theme === "light"}
                                class:dark:bg-rust-950={appState.config
                                    .model === model.id &&
                                    appState.theme === "dark"}
                            >
                                <button
                                    class="flex-1 text-left p-3 min-w-0"
                                    onclick={() => {
                                        appState.updateConfig({
                                            model: model.id,
                                        });
                                        appState.closePopovers();
                                    }}
                                >
                                    <div
                                        class="font-medium text-sm truncate"
                                        class:text-(--accent-primary)={appState
                                            .config.model === model.id}
                                        class:text-(--text-primary)={appState
                                            .config.model !== model.id}
                                    >
                                        {model.name}
                                    </div>
                                    <div
                                        class="text-xs text-(--text-secondary) truncate"
                                    >
                                        {model.description}
                                    </div>
                                </button>

                                <!-- Edit Button -->
                                <button
                                    class="p-2 mr-2 text-(--text-secondary) hover:text-(--accent-primary) opacity-0 group-hover/item:opacity-100 focus:opacity-100 transition-opacity"
                                    title="Edit Model Info"
                                    onclick={(e) => {
                                        e.stopPropagation();
                                        editingModelId = model.id;
                                        appState.closePopovers();
                                    }}
                                >
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        class="h-4 w-4"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        stroke="currentColor"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"
                                        />
                                    </svg>
                                </button>
                            </div>
                        {/each}
                        {#if appState.models.length === 0}
                            <div
                                class="text-xs text-(--text-secondary) p-3 text-center"
                            >
                                No models found.
                            </div>
                        {/if}
                    </div>
                {/if}
            </div>
        </section>

        <!-- Scale Selection & Settings -->
        <section>
            <div class="grid grid-cols-[auto_1fr] gap-2">
                <!-- Scale Buttons -->
                <div class="grid grid-cols-2 gap-2 col-span-2">
                    {#each [2, 4] as scale}
                        <button
                            class="p-2 rounded-lg border text-sm font-medium transition-all duration-200 ease-out hover:scale-105"
                            class:border-(--accent-primary)={appState.config
                                .scale === scale}
                            class:text-(--accent-primary)={appState.config
                                .scale === scale}
                            class:shadow-sm={appState.config.scale === scale}
                            class:border-(--border-main)={appState.config
                                .scale !== scale}
                            class:text-(--text-secondary)={appState.config
                                .scale !== scale}
                            class:hover:border-(--accent-primary)={appState
                                .config.scale !== scale}
                            class:hover:text-(--accent-primary)={appState.config
                                .scale !== scale}
                            onclick={() => appState.updateConfig({ scale })}
                        >
                            {scale}x
                        </button>
                    {/each}
                </div>
            </div>
        </section>

        <!-- Output Format -->
        <section>
            <h2
                class="text-xs font-bold text-(--text-secondary) uppercase mb-3"
            >
                Output Format
            </h2>
            <div class="grid grid-cols-3 gap-2 mb-3">
                {#each [{ id: "jpeg", label: "JPEG" }, { id: "webp", label: "WebP" }, { id: "png", label: "PNG" }] as format}
                    <button
                        class="p-2 rounded-lg border text-sm font-medium transition-all duration-200 ease-out hover:scale-105"
                        class:border-(--accent-primary)={appState.config
                            .format === format.id}
                        class:text-(--accent-primary)={appState.config
                            .format === format.id}
                        class:shadow-sm={appState.config.format === format.id}
                        class:border-(--border-main)={appState.config.format !==
                            format.id}
                        class:text-(--text-secondary)={appState.config
                            .format !== format.id}
                        class:hover:border-(--accent-primary)={appState.config
                            .format !== format.id}
                        class:hover:text-(--accent-primary)={appState.config
                            .format !== format.id}
                        onclick={() => {
                            appState.updateConfig({ format: format.id as any });
                            // Auto-set compression based on format
                            if (format.id === "jpeg")
                                appState.updateConfig({ compression: "lossy" });
                            if (format.id === "png")
                                appState.updateConfig({
                                    compression: "lossless",
                                });
                        }}
                    >
                        {format.label}
                    </button>
                {/each}
            </div>

            <!-- Compression Mode -->
            <div
                class="grid grid-cols-2 gap-2 bg-(--bg-canvas)/50 p-1 rounded-lg border border-(--border-main)"
            >
                {#each [{ id: "lossy", label: "Efficient (Lossy)" }, { id: "lossless", label: "Perfect (Lossless)" }] as mode}
                    <button
                        class="p-1.5 rounded text-xs font-medium transition-all duration-200"
                        class:bg-(--bg-surface)={appState.config.compression ===
                            mode.id}
                        class:text-(--text-primary)={appState.config
                            .compression === mode.id}
                        class:shadow-sm={appState.config.compression ===
                            mode.id}
                        class:text-[var(--text-secondary)]={appState.config
                            .compression !== mode.id}
                        class:opacity-50={(appState.config.format === "jpeg" &&
                            mode.id === "lossless") ||
                            (appState.config.format === "png" &&
                                mode.id === "lossy")}
                        disabled={(appState.config.format === "jpeg" &&
                            mode.id === "lossless") ||
                            (appState.config.format === "png" &&
                                mode.id === "lossy")}
                        onclick={() =>
                            appState.updateConfig({
                                compression: mode.id as any,
                            })}
                    >
                        {mode.label}
                    </button>
                {/each}
            </div>
        </section>

        <!-- Face Enhance -->
        <section>
            <label
                class="flex items-center justify-between p-3 rounded-lg border border-(--border-main) bg-(--bg-canvas)/50 cursor-pointer hover:border-(--accent-primary) hover:shadow-sm transition-all duration-200 ease-out mt-2"
            >
                <div class="flex flex-col">
                    <span class="text-sm text-(--text-primary)">Eco Mode</span>
                    <span class="text-[10px] text-(--text-secondary)"
                        >Prefer NPU (Save Battery)</span
                    >
                </div>
                <input
                    type="checkbox"
                    checked={appState.config.preferNpu}
                    onchange={handleNpuChange}
                    class="w-4 h-4 rounded border-(--border-main) bg-(--bg-surface) text-(--accent-primary) focus:ring-2 focus:ring-(--accent-primary) focus:ring-offset-2 focus:ring-offset-(--bg-canvas) transition-all cursor-pointer"
                />
            </label>
        </section>

        <!-- Warnings -->
        {#if appState.currentImageMetadata}
            {@const width = appState.currentImageMetadata.width}
            {@const targetWidth = width * appState.config.scale}

            {#if width > 2048}
                <div
                    class="p-3 rounded-lg bg-yellow-500/10 border border-yellow-500/20 text-yellow-600 dark:text-yellow-400 text-xs"
                >
                    <div class="font-bold mb-1">High Res Input</div>
                    Upscaling large images may take longer.
                </div>
            {/if}

            {#if targetWidth > 8192}
                <div
                    class="p-3 rounded-lg bg-orange-500/10 border border-orange-500/20 text-orange-600 dark:text-orange-400 text-xs"
                >
                    <div class="font-bold mb-1">Massive Output (>8K)</div>
                    Result will be >100MP. Ensure disk space.
                </div>
            {/if}
        {/if}
    </div>

    <!-- Upscale Button -->
    <div class="mt-auto pt-4 border-t border-(--border-main) space-y-4">
        <button
            class="w-full py-3 rounded-lg font-bold text-white transition-all duration-200 ease-out transform hover:scale-[1.02] active:scale-98 disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none"
            class:bg-(--accent-primary)={(appState.currentImage ||
                appState.batchMode) &&
                appState.status === "idle"}
            class:shadow-lg={(appState.currentImage || appState.batchMode) &&
                appState.status === "idle"}
            class:hover:shadow-xl={(appState.currentImage ||
                appState.batchMode) &&
                appState.status === "idle"}
            class:hover:brightness-110={(appState.currentImage ||
                appState.batchMode) &&
                appState.status === "idle"}
            class:bg-zinc-300={(!appState.currentImage &&
                !appState.batchMode) ||
                appState.status !== "idle"}
            class:dark:bg-zinc-800={(!appState.currentImage &&
                !appState.batchMode) ||
                appState.status !== "idle"}
            class:text-zinc-500={!appState.currentImage && !appState.batchMode}
            disabled={(!appState.currentImage && !appState.batchMode) ||
                appState.status !== "idle"}
            onclick={() => appState.upscaleImage()}
        >
            {#if appState.status === "processing"}
                Processing...
            {:else}
                Upscale Image
            {/if}
        </button>

        <div>
            <SystemInfo />
        </div>
    </div>
</aside>
