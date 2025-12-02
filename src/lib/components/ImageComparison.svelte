<script lang="ts">
    import { appState } from "../state.svelte";
    import { readFile } from "@tauri-apps/plugin-fs";

    let sliderValue = $state(50);
    let container: HTMLDivElement;
    let containerWidth = $state(0);
    let containerHeight = $state(0);
    let isDraggingSlider = $state(false);
    let isHoveringHandle = $state(false);

    // Zoom State
    let scale = $state(1);

    // Image Blob URLs
    let imageUrl = $state("");
    let resultUrl = $state("");

    // Load images as Blobs to bypass asset protocol issues
    $effect(() => {
        let activeUrl = "";
        if (appState.currentImage) {
            loadFile(appState.currentImage).then((url) => {
                imageUrl = url;
                activeUrl = url;
            });
        } else {
            imageUrl = "";
        }

        return () => {
            if (activeUrl) URL.revokeObjectURL(activeUrl);
        };
    });

    $effect(() => {
        let activeUrl = "";
        if (appState.resultImage) {
            loadFile(appState.resultImage).then((url) => {
                resultUrl = url;
                activeUrl = url;
            });
        } else {
            resultUrl = imageUrl; // Fallback to original if no result
        }

        return () => {
            if (activeUrl) URL.revokeObjectURL(activeUrl);
        };
    });

    async function loadFile(path: string): Promise<string> {
        try {
            const data = await readFile(path);
            const blob = new Blob([data]);
            return URL.createObjectURL(blob);
        } catch (e) {
            console.error("Failed to load image:", path, e);
            return "";
        }
    }

    // Slider Logic
    function startDrag(e: MouseEvent) {
        e.preventDefault();
        isDraggingSlider = true;
    }

    function stopDrag() {
        isDraggingSlider = false;
    }

    function handleMove(e: MouseEvent) {
        if (!isDraggingSlider || !container) return;

        const rect = container.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const w = rect.width;
        
        // Calculate percentage based on zoomed coordinate system
        // When zoomed, the content is larger than the viewport (rect.width)
        // We need to map the mouse position (relative to viewport) to the image percentage
        // Formula: ((x - w/2) / (w * scale) + 0.5) * 100
        const val = ((x - w / 2) / (w * scale) + 0.5) * 100;
        sliderValue = Math.max(0, Math.min(val, 100));
    }

    // Zoom Logic (No Pan)
    function handleWheel(e: WheelEvent) {
        e.preventDefault();
        const delta = e.deltaY * -0.001;
        // Limit zoom between 1x and 5x
        scale = Math.min(Math.max(1, scale + delta), 5);
    }
</script>

<svelte:window onmouseup={stopDrag} onmousemove={handleMove} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
    class="relative w-full h-full bg-zinc-950 flex items-center justify-center overflow-hidden"
    bind:this={container}
    bind:clientWidth={containerWidth}
    bind:clientHeight={containerHeight}
    onwheel={handleWheel}
>
    <!-- Container for Zoom Transform -->
    <div
        class="relative w-full h-full flex items-center justify-center"
        style="transform: scale({scale}); transition: transform 0.1s ease-out;"
    >
        <!-- Background Image (Original) -->
        <!-- We use absolute positioning and object-contain to center it -->
        <img
            src={imageUrl}
            alt="Original"
            class="absolute inset-0 w-full h-full object-contain pointer-events-none select-none"
            draggable="false"
        />

        <!-- Comparison Overlay (Only if result exists) -->
        {#if appState.resultImage}
            <!-- Wrapper clips the upscaled image -->
            <div
                class="absolute top-0 left-0 h-full overflow-hidden pointer-events-none select-none"
                style="width: {sliderValue}%"
            >
                <!-- Upscaled Image -->
                <!-- CRITICAL: Must be same size as container to align with background -->
                <img
                    src={resultUrl}
                    alt="Upscaled"
                    class="absolute top-0 left-0 h-full object-contain max-w-none"
                    style="width: {containerWidth}px;"
                />
            </div>

            <!-- Slider Handle -->
            <!-- Counter-scaled to maintain visual size -->
            <div
                class="absolute inset-y-0 bg-white cursor-ew-resize z-10"
                style="
                    left: {sliderValue}%; 
                    width: {4 / scale}px; 
                    box-shadow: {isHoveringHandle ? `0 0 ${10 / scale}px rgba(255,255,255,0.5)` : 'none'};
                "
                onmousedown={startDrag}
                onmouseenter={() => isHoveringHandle = true}
                onmouseleave={() => isHoveringHandle = false}
                onwheel={(e) => e.stopPropagation()}
            >
                <div
                    class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-white rounded-full flex items-center justify-center text-zinc-900 text-xs font-bold"
                    style="
                        width: {32 / scale}px; 
                        height: {32 / scale}px;
                        box-shadow: 0 {4 / scale}px {6 / scale}px rgba(0,0,0,0.3);
                    "
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                        style="width: {16 / scale}px; height: {16 / scale}px;"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M8 9l4-4 4 4m0 6l-4 4-4-4"
                            transform="rotate(90 12 12)"
                        />
                    </svg>
                </div>
            </div>
        {/if}
    </div>

    <!-- Progress Overlay -->
    {#if appState.status === "processing" || appState.status === "cancelling"}
        <div
            class="absolute inset-0 bg-black/60 z-50 flex flex-col items-center justify-center backdrop-blur-sm"
        >
            <div class="w-64 bg-zinc-800 rounded-full h-2 mb-4 overflow-hidden">
                <div
                    class="bg-blue-500 h-full transition-all duration-300"
                    style="width: {appState.progress * 100}%"
                ></div>
            </div>
            <p class="text-white font-medium animate-pulse">
                {#if appState.status === "cancelling"}
                    Cancelling...
                {:else}
                    Upscaling... {Math.round(appState.progress * 100)}%
                {/if}
            </p>
            {#if appState.executionProvider}
                <p class="text-zinc-400 text-sm mt-1">
                    Running on {appState.executionProvider}
                </p>
            {/if}
            <button
                class="mt-6 px-6 py-2 bg-red-500/20 text-red-400 border border-red-500/50 rounded-lg hover:bg-red-500/30 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                onclick={() => {
                    appState.cancelJob();
                }}
                disabled={appState.status === "cancelling"}
            >
                Cancel
            </button>
        </div>
    {/if}
</div>
