<script lang="ts">
    import { appState, type ModelManifest } from "../state.svelte";
    import { fade, scale } from "svelte/transition";

    let { modelId, onClose } = $props<{
        modelId: string;
        onClose: () => void;
    }>();

    let model = $derived(appState.models.find((m) => m.id === modelId));
    let name = $state("");
    let description = $state("");
    let batchSize = $state(1);
    let isSaving = $state(false);

    $effect(() => {
        if (model) {
            name = model.name;
            description = model.description;
            batchSize = model.batch_size || 1;
        }
    });

    async function handleSave() {
        if (!model) return;
        isSaving = true;

        // DISABLE PROTECTION FOR DIAGNOSIS
        // appState.markSaved();

        try {
            const { invoke } = await import("@tauri-apps/api/core");

            // Update backend
            const updatedManifest = await invoke<ModelManifest>(
                "update_model_info",
                {
                    id: model.id,
                    name,
                    description,
                    batchSize: Number(batchSize), // Send as camelCase
                },
            );

            // Update local state immediately with authoritative response
            const idx = appState.models.findIndex(
                (m) => m.id === updatedManifest.id,
            );
            if (idx !== -1) {
                appState.models[idx] = updatedManifest;
            }

            // If this is the active model, update config
            if (appState.config.model === model.id) {
                appState.config.batchSize = updatedManifest.batch_size || 1;
            }

            appState.addToast("Model info updated", "success");
            onClose();
        } catch (e) {
            console.error("[DIAGNOSTIC] [ModelEditor] Save failed:", e);
            appState.addToast("Failed to update model info", "error");
        } finally {
            isSaving = false;
        }
    }

    async function handleReset() {
        if (!model) return;
        if (!confirm("Reset to default name and description?")) return;

        isSaving = true;
        try {
            const { invoke } = await import("@tauri-apps/api/core");
            await invoke("reset_model_info", { id: model.id });
            await appState.loadModels();
            appState.addToast("Model info reset", "success");
            onClose();
        } catch (e) {
            console.error(e);
            appState.addToast("Failed to reset model info", "error");
        } finally {
            isSaving = false;
        }
    }
</script>

<div
    class="fixed inset-0 z-100 flex items-center justify-center bg-black/50 backdrop-blur-sm"
    transition:fade={{ duration: 200 }}
    onclick={(e) => {
        if (e.target === e.currentTarget) onClose();
    }}
    onkeydown={(e) => {
        if (e.key === "Escape") onClose();
    }}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
>
    <div
        class="w-full max-w-md bg-(--bg-surface) border border-(--border-main) rounded-xl shadow-2xl p-6"
        transition:scale={{ duration: 200, start: 0.95 }}
    >
        <h2 class="text-lg font-bold text-(--text-primary) mb-4">
            Edit Model Info
        </h2>

        <div class="space-y-4">
            <div>
                <label
                    for="model-name"
                    class="block text-xs font-bold text-(--text-secondary) uppercase mb-1"
                >
                    Name
                </label>
                <input
                    id="model-name"
                    type="text"
                    bind:value={name}
                    class="w-full p-2 rounded-lg bg-(--bg-canvas) border border-(--border-main) text-(--text-primary) focus:border-(--accent-primary) focus:ring-1 focus:ring-(--accent-primary) outline-none transition-all"
                    placeholder="My Custom Model"
                />
            </div>

            <div>
                <label
                    for="model-desc"
                    class="block text-xs font-bold text-(--text-secondary) uppercase mb-1"
                >
                    Description
                </label>
                <input
                    id="model-desc"
                    type="text"
                    bind:value={description}
                    class="w-full p-2 rounded-lg bg-(--bg-canvas) border border-(--border-main) text-(--text-primary) focus:border-(--accent-primary) focus:ring-1 focus:ring-(--accent-primary) outline-none transition-all"
                    placeholder="Best for..."
                />
            </div>

            <div>
                <div class="flex justify-between items-center mb-1">
                    <label
                        for="model-batch"
                        class="block text-xs font-bold text-(--text-secondary) uppercase"
                    >
                        Batch Size
                    </label>
                    <span class="text-xs font-mono text-(--accent-primary)"
                        >{batchSize}</span
                    >
                </div>
                <input
                    id="model-batch"
                    type="range"
                    min="1"
                    max="8"
                    step="1"
                    bind:value={batchSize}
                    class="w-full h-2 bg-(--bg-canvas) rounded-lg appearance-none cursor-pointer accent-(--accent-primary)"
                />
                <p class="text-[10px] text-(--text-secondary) mt-1">
                    Higher values improve speed but use more VRAM. Only works if
                    the model supports dynamic batching. Default: 1.
                </p>
            </div>

            <div class="pt-2 text-xs text-(--text-secondary)">
                <p>
                    Filename: <span class="font-mono">{model?.filename}</span>
                </p>
                <p>Scale: {model?.scale}x</p>
            </div>
        </div>

        <div class="flex justify-end gap-2 mt-6">
            <button
                class="px-4 py-2 rounded-lg text-sm font-medium text-(--text-secondary) hover:bg-(--bg-canvas) transition-colors"
                onclick={handleReset}
                disabled={isSaving}
            >
                Reset Default
            </button>
            <button
                class="px-4 py-2 rounded-lg text-sm font-medium bg-(--accent-primary) text-white hover:brightness-110 transition-all shadow-lg shadow-orange-500/20"
                onclick={handleSave}
                disabled={isSaving}
            >
                {isSaving ? "Saving..." : "Save Changes"}
            </button>
        </div>
    </div>
</div>
