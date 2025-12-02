// Global State using Svelte 5 Runes

type ToastType = "success" | "error" | "info";
type Theme = "light" | "dark" | "system";

interface Toast {
    id: number;
    message: string;
    type: ToastType;
}

export interface ModelManifest {
    id: string;
    name: string;
    description: string;
    filename: string;
    scale: number;
    alignment: number;
    batch_size?: number;
}

interface UpscaleConfig {
    model: string;
    scale: number;
    batchSize: number;

    format: "png" | "webp" | "jpeg";
    compression: "lossy" | "lossless";
    preferNpu: boolean;
}

interface ProgressPayload {
    job_id: string;
    progress: number;
    status: string;
    execution_provider: string;
    current_file?: string;
}

interface ImageMetadata {
    width: number;
    height: number;
    format: string;
    [key: string]: string | number;
}

export interface QueueItem {
    path: string;
    status: "pending" | "processing" | "done" | "error";
    progress: number;
    result?: string;
    error?: string;
}

class AppState {
    // Image State
    currentImage = $state<string | null>(null);
    resultImage = $state<string | null>(null);
    currentImageMetadata = $state<ImageMetadata | null>(null);

    // Batch State
    queue = $state<QueueItem[]>([]);
    batchMode = $derived(this.queue.length > 1);

    // Processing State
    status = $state<"idle" | "processing" | "cancelling" | "done" | "error">("idle");
    progress = $state(0);
    executionProvider = $state<string>("");
    currentJobId = $state<string | null>(null);

    // UI State
    activePopover = $state<string | null>(null);

    // Configuration (persisted)
    config = $state<UpscaleConfig>({
        model: "dat_light_x4",
        scale: 4,
        batchSize: 1,

        format: "jpeg",
        compression: "lossy",
        preferNpu: false
    });

    // Theme (persisted)
    theme = $state<Theme>("dark");

    // System Info
    vramUsage = $state(0);
    totalVram = $state(0);
    vramFree = $derived(this.totalVram - this.vramUsage);

    // Optimization Mode (Derived)
    optimizationMode = $derived.by(() => {
        if (this.vramFree > 12288) return "Performance";
        if (this.vramFree > 6144) return "Balanced";
        return "Efficiency";
    });

    // Toasts
    toasts = $state<Toast[]>([]);

    // Models
    models = $state<ModelManifest[]>([]);
    private lastSaveTime = 0;
    private loadTimeout: any = null; // For debouncing model loads

    constructor() {
        this.loadPersistedState();
        this.applyTheme();
        this.initKeyboardShortcuts();
        this.initEventListeners();
        this.loadModels();
    }

    markSaved() {
        this.lastSaveTime = Date.now();
        console.log(`[AppState] MARK SAVED at ${this.lastSaveTime}. Ignoring file watcher events for 3s.`);
    }

    async loadModels() {
        if (typeof window === 'undefined') return;
        // console.log("[DIAGNOSTIC] [AppState] loadModels START");
        try {
            const { invoke } = await import("@tauri-apps/api/core");
            const fetchedModels = await invoke<ModelManifest[]>("get_models");
            // console.log(`[DIAGNOSTIC] [AppState] Fetched ${fetchedModels.length} models.`);

            // Log the batch size of the currently active model from the fetch
            const activeInFetch = fetchedModels.find(m => m.id === this.config.model);
            /*
            if (activeInFetch) {
                console.log(`[DIAGNOSTIC] [AppState] Active model (${this.config.model}) in fetch has batch_size: ${activeInFetch.batch_size}`);
            } else {
                console.log(`[DIAGNOSTIC] [AppState] Active model (${this.config.model}) NOT found in fetch.`);
            }
            */

            this.models = fetchedModels;

            // Ensure selected model is valid
            if (this.models.length > 0 && !this.models.find(m => m.id === this.config.model)) {
                // console.log("[DIAGNOSTIC] [AppState] Active model invalid, switching to first available.");
                this.updateConfig({ model: this.models[0].id });
            } else {
                // EAGER LOADING: Trigger pre-load for the saved model immediately.
                // This ensures the model is ready (and scale detected) before the user even clicks "Upscale".
                // This also handles syncing batch_size from the manifest.
                this.setModel(this.config.model);
            }
        } catch (e) {
            console.error("Failed to load models:", e);
            this.addToast("Failed to load models", "error");
        }
    }

    // ===== Toast Management =====
    addToast(message: string, type: ToastType = "info") {
        const id = Date.now();
        this.toasts.push({ id, message, type });
        setTimeout(() => {
            this.toasts = this.toasts.filter(t => t.id !== id);
        }, 3000);
    }

    // ===== Processing State =====
    setProcessing(isProcessing: boolean) {
        this.status = isProcessing ? "processing" : "idle";
        if (!isProcessing) {
            this.executionProvider = "";
        }
    }

    setProgress(value: number) {
        this.progress = value;
    }

    // ===== Theme Management =====
    setTheme(newTheme: Theme) {
        this.theme = newTheme;
        this.persistTheme();
        this.applyTheme();
    }

    private applyTheme() {
        if (typeof window === 'undefined') return;

        const isDark = this.theme === 'dark' ||
            (this.theme === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches);

        if (isDark) {
            document.documentElement.classList.add('dark');
        } else {
            document.documentElement.classList.remove('dark');
        }
    }

    // ===== Persistence =====
    private loadPersistedState() {
        if (typeof window === 'undefined') return;

        try {
            const savedConfig = localStorage.getItem('rustscale_config');
            if (savedConfig) {
                // Merge saved config but ensure model is valid
                const parsed = JSON.parse(savedConfig);
                // Reset model if it's an old one
                if (["upscale_fast", "upscale_pro", "restore_soft", "restore_heavy"].includes(parsed.model)) {
                    parsed.model = "dat_light_x4";
                }
                Object.assign(this.config, parsed);
            }

            const savedTheme = localStorage.getItem('rustscale_theme') as Theme;
            if (savedTheme) {
                this.theme = savedTheme;
            }
        } catch (e) {
            console.error('Failed to load persisted state:', e);
        }
    }

    private persistConfig() {
        if (typeof window === 'undefined') return;
        try {
            localStorage.setItem('rustscale_config', JSON.stringify(this.config));
        } catch (e) {
            console.error('Failed to persist config:', e);
        }
    }

    private persistTheme() {
        if (typeof window === 'undefined') return;
        try {
            localStorage.setItem('rustscale_theme', this.theme);
        } catch (e) {
            console.error('Failed to persist theme:', e);
        }
    }

    // Call this when config changes
    async updateConfig(updates: Partial<UpscaleConfig>) {
        // Delegate specific actions
        if (updates.model && updates.model !== this.config.model) {
            this.setModel(updates.model);
            return;
        }

        // Apply other updates
        Object.assign(this.config, updates);
        this.persistConfig();
    }

    // Explicitly switch model
    async setModel(modelId: string) {
        const newModel = this.models.find(m => m.id === modelId);
        if (!newModel) return;

        // Update config immediately for UI responsiveness
        this.config.model = modelId;
        // Load the batch size saved for this model (or default to 1)
        this.config.batchSize = newModel.batch_size || 1;
        this.persistConfig();

        // DEBOUNCE: Wait 300ms before triggering heavy backend load
        if (this.loadTimeout) {
            clearTimeout(this.loadTimeout);
        }

        this.loadTimeout = setTimeout(async () => {
            // Preload Model & Detect Scale (Background)
            try {
                const { invoke } = await import("@tauri-apps/api/core");
                // Capture current model ID to detect race conditions
                const requestedModelId = modelId;

                // Call backend
                const response = await invoke<{ scale: number; batch_size?: number }>("preload_model", {
                    modelFilename: newModel.filename,
                    preferNpu: this.config.preferNpu,
                });

                // Race Condition Check: If the user switched models again, ignore this result
                if (this.config.model !== requestedModelId) {
                    console.log(`[AppState] Ignoring preload result for ${modelId} because model changed to ${this.config.model}`);
                    return;
                }

                // Update Scale if different
                if (response.scale !== this.config.scale) {
                    console.log(`[AppState] Auto-detected scale change: ${this.config.scale} -> ${response.scale}`);
                    this.config.scale = response.scale;
                    this.persistConfig();
                    this.addToast(`Model Scale Detected: ${response.scale}x`, "info");
                }

            } catch (e) {
                console.error("Failed to preload model:", e);
                // Don't show error toast for preload failure, it's a background optimization
            }
        }, 300);
    }

    // UI Actions
    togglePopover(id: string) {
        if (this.activePopover === id) {
            this.activePopover = null;
        } else {
            this.activePopover = id;
        }
    }

    closePopovers() {
        this.activePopover = null;
    }

    // ===== Keyboard Shortcuts =====
    private initKeyboardShortcuts() {
        if (typeof window === 'undefined') return;

        window.addEventListener('keydown', async (e) => {
            // Ctrl+Enter / Cmd+Enter: Trigger Upscale
            if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
                e.preventDefault();
                this.upscaleImage();
            }

            // Esc: Cancel/Clear
            if (e.key === 'Escape') {
                e.preventDefault();
                if (this.status === 'processing') {
                    await this.cancelJob();
                } else if (this.currentImage) {
                    this.clearImage();
                }
            }
        });
    }

    // ===== Core Actions =====
    async cancelJob() {
        if (this.status !== 'processing') return;

        // Update status to 'cancelling' to update UI immediately
        this.status = "cancelling";

        const { invoke } = await import("@tauri-apps/api/core");

        if (this.currentJobId) {
            console.log(`[AppState] Cancelling job: ${this.currentJobId}`);
            try {
                await invoke("cancel_job", { jobId: this.currentJobId });
            } catch (e) {
                console.error("Failed to cancel job:", e);
            }
        } else {
            console.warn("[AppState] No job ID found to cancel!");
            this.setProcessing(false); // Force reset if no ID
        }
    }

    async upscaleImage() {
        if (this.status !== 'idle') return;

        // BATCH MODE
        if (this.batchMode) {
            await this.upscaleBatch();
            return;
        }

        // SINGLE MODE
        if (!this.currentImage) return;

        try {
            this.setProcessing(true);
            this.addToast("Upscaling started...", "info");

            // Generate Job ID on Frontend
            this.currentJobId = crypto.randomUUID();
            console.log(`[AppState] Starting job: ${this.currentJobId}`);

            const { invoke } = await import("@tauri-apps/api/core");

            const resultPath = await invoke<string>("upscale_image", {
                path: this.currentImage,
                config: {
                    model: this.config.model,
                    scale: this.config.scale,
                    batch_size: this.config.batchSize,

                    format: this.config.format,
                    compression: this.config.compression,
                    prefer_npu: this.config.preferNpu,
                },
                id: this.currentJobId // Pass the ID to backend
            });

            this.resultImage = resultPath;
            this.addToast("Upscale Complete!", "success");
            this.setProcessing(false);
            this.setProgress(1);
        } catch (e: any) {
            console.error(e);
            const errStr = typeof e === 'string' ? e : JSON.stringify(e);
            if (errStr.toLowerCase().includes("cancelled")) {
                this.addToast("Job cancelled", "info");
            } else {
                this.addToast(`Error: ${errStr}`, "error");
            }
            this.setProcessing(false);
        }
    }

    async upscaleBatch() {
        try {
            this.setProcessing(true);
            this.addToast(`Starting batch of ${this.queue.length} files...`, "info");

            this.currentJobId = crypto.randomUUID();
            const paths = this.queue.map(item => item.path);

            // Reset queue statuses
            this.queue = this.queue.map(item => ({
                ...item,
                status: "pending",
                progress: 0,
                result: undefined,
                error: undefined
            }));

            const { invoke } = await import("@tauri-apps/api/core");

            // We don't await the result immediately if we want to track progress via events?
            // Actually upscale_multiple returns a report at the end.
            // Events will update the UI in real-time.

            interface BatchReport {
                successful: string[];
                failed: [string, string][];
            }

            const report = await invoke<BatchReport>("upscale_multiple", {
                paths,
                config: {
                    model: this.config.model,
                    scale: this.config.scale,
                    batch_size: this.config.batchSize,
                    format: this.config.format,
                    compression: this.config.compression,
                    prefer_npu: this.config.preferNpu,
                },
                id: this.currentJobId
            });

            this.addToast(`Batch Complete. ${report.successful.length} success, ${report.failed.length} failed.`, report.failed.length > 0 ? "info" : "success");
            this.setProcessing(false);
            this.currentJobId = null;

        } catch (e: any) {
            console.error(e);
            this.addToast(`Batch Error: ${e}`, "error");
            this.setProcessing(false);
        }
    }

    clearImage() {
        this.currentImage = null;
        this.resultImage = null;
        this.status = "idle";
        this.progress = 0;
        this.executionProvider = "";
        this.currentJobId = null;
    }

    // ===== Helper: Path Normalization =====
    private normalizePath(path: string): string {
        // Normalize separators to forward slashes and lowercase for consistent matching
        return path.replace(/\\/g, '/').toLowerCase();
    }

    // ===== Event Listeners =====
    async initEventListeners() {
        if (typeof window !== 'undefined') {
            const { listen } = await import('@tauri-apps/api/event');
            await listen<ProgressPayload>('upscale-progress', (event) => {
                const { progress, status, current_file } = event.payload;

                // Always update global status/progress (for single file mode)
                this.progress = progress;
                this.status = status as any;

                // Targeted update for batch mode
                if (current_file) {
                    const normalizedTarget = this.normalizePath(current_file);
                    const idx = this.queue.findIndex(item => this.normalizePath(item.path) === normalizedTarget);

                    if (idx !== -1) {
                        this.queue[idx].progress = progress;
                        // Force status to processing if it's not (e.g. if batch-progress lagged)
                        if (this.queue[idx].status !== "processing") {
                            this.queue[idx].status = "processing";
                        }
                    }
                }
            });

            // Batch Progress Events
            await listen<any>('batch-progress', (event) => {
                const { current } = event.payload;
                const idx = current - 1;

                if (this.queue[idx]) {
                    this.queue[idx].status = "processing";
                    // Ensure previous items are marked done (failsafe)
                    if (idx > 0 && this.queue[idx - 1].status === "processing") {
                        this.queue[idx - 1].status = "done";
                        this.queue[idx - 1].progress = 1;
                    }
                }
            });

            // File Complete Events
            await listen<any>('file-complete', (event) => {
                const { file } = event.payload;
                const normalizedTarget = this.normalizePath(file);
                const idx = this.queue.findIndex(item => this.normalizePath(item.path) === normalizedTarget);

                if (idx !== -1) {
                    this.queue[idx].status = "done";
                    this.queue[idx].progress = 1;
                }
            });

            await listen<any>('file-error', (event) => {
                const { file, error } = event.payload;
                const normalizedTarget = this.normalizePath(file);
                const idx = this.queue.findIndex(item => this.normalizePath(item.path) === normalizedTarget);

                if (idx !== -1) {
                    this.queue[idx].status = "error";
                    this.queue[idx].error = error;
                }
            });

            // Listen for batch size warnings
            await listen<string>('batch-size-warning', (event) => {
                this.addToast(event.payload, "info");
            });

            // Listen for model changes (Hot Reloading)
            await listen('models-changed', () => {
                // console.log(`[DIAGNOSTIC] [AppState] 'models-changed' event received at ${Date.now()}`);

                // RESTORE PROTECTION
                const now = Date.now();
                const timeSinceSave = now - this.lastSaveTime;
                if (timeSinceSave < 3000) {
                    console.log("[AppState] IGNORING 'models-changed' event (self-induced protection active).");
                    return;
                }

                console.log("[AppState] Reloading models...");
                this.loadModels();
            });

            // Listen for system theme changes
            window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
                if (this.theme === 'system') {
                    this.applyTheme();
                }
            });
        }
    }
}

export const appState = new AppState();
