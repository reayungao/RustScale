<script lang="ts">
    import { open } from "@tauri-apps/plugin-dialog";
    import { appState } from "../state.svelte";

    import { getCurrentWindow } from "@tauri-apps/api/window";
    import { listen } from "@tauri-apps/api/event";
    import { onMount } from "svelte";

    let isDragging = $state(false);

    onMount(() => {
        // Listen for Tauri file drop events (gives full paths)
        const unlistenDrop = listen<{ paths: string[] }>(
            "tauri://drag-drop",
            (event) => {
                if (event.payload.paths && event.payload.paths.length > 0) {
                    handleFiles(event.payload.paths);
                    isDragging = false;
                }
            },
        );

        // Listen for drag enter to show UI feedback
        const unlistenEnter = listen("tauri://drag-enter", () => {
            isDragging = true;
        });

        // Listen for drag leave to hide UI feedback
        const unlistenLeave = listen("tauri://drag-leave", () => {
            isDragging = false;
        });

        return () => {
            unlistenDrop.then((f) => f());
            unlistenEnter.then((f) => f());
            unlistenLeave.then((f) => f());
        };
    });

    // HTML5 Drag & Drop (Visual only, or fallback)
    function handleDragOver(e: DragEvent) {
        e.preventDefault();
        // We rely on Tauri events for the actual drop, but this prevents browser default behavior
    }

    async function selectFile() {
        try {
            const selected = await open({
                multiple: true,
                directory: false, // For now, let's stick to files. Folder picker is usually separate.
                filters: [
                    {
                        name: "Images",
                        extensions: ["png", "jpg", "jpeg", "webp"],
                    },
                ],
            });

            if (selected) {
                const paths = Array.isArray(selected) ? selected : [selected];
                await handleFiles(paths);
            }
        } catch (err) {
            console.error(err);
            appState.addToast("Failed to open file dialog", "error");
        }
    }

    async function handleFiles(paths: string[]) {
        try {
            const { invoke } = await import("@tauri-apps/api/core");
            const scannedPaths = await invoke<string[]>("scan_paths", {
                paths,
            });

            if (scannedPaths.length === 0) {
                appState.addToast("No supported images found", "info");
                return;
            }

            if (scannedPaths.length === 1 && appState.queue.length === 0) {
                // Single file mode
                appState.currentImage = scannedPaths[0];
                appState.addToast(
                    `Loaded ${getFilename(scannedPaths[0])}`,
                    "success",
                );
            } else {
                // Batch mode
                const newItems = scannedPaths.map((p) => ({
                    path: p,
                    status: "pending" as const,
                    progress: 0,
                }));

                // Append to queue
                appState.queue = [...appState.queue, ...newItems];
                appState.addToast(
                    `Added ${newItems.length} files to queue`,
                    "success",
                );

                // If we were in single mode, clear it to show batch view?
                // Actually, if queue > 0, batchMode becomes true.
                // But we should probably clear currentImage if we are switching to batch mode explicitly?
                // Or just let batchMode take precedence in the UI.
            }
        } catch (e) {
            console.error("Failed to scan paths:", e);
            appState.addToast("Failed to process files", "error");
        }
    }

    function getFilename(path: string) {
        let name = path.split(/[\\/]/).pop() || "Image";
        if (name.length > 25) {
            name = name.slice(0, 15) + "..." + name.slice(-7);
        }
        return name;
    }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
    class="w-full h-full flex flex-col items-center justify-center border-2 border-dashed rounded-xl transition-all duration-200 cursor-pointer"
    class:border-zinc-700={!isDragging}
    class:border-blue-500={isDragging}
    class:bg-blue-500_10={isDragging}
    class:hover:border-zinc-600={!isDragging}
    class:hover:bg-zinc-800_50={!isDragging}
    ondragover={handleDragOver}
    ondrop={(e) => e.preventDefault()}
    onclick={selectFile}
>
    <div class="text-zinc-500 mb-4">
        <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-16 w-16 mx-auto mb-4 opacity-50"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="1.5"
                d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
            />
        </svg>
        <p class="text-lg font-medium text-zinc-300">
            Click or Drag Image to Upscale
        </p>
        <p class="text-sm mt-2">Supports JPG, PNG, WEBP</p>
    </div>
</div>
