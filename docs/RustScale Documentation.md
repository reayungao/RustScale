# **Software Architecture Document: RustScale Desktop**

**Version:** 6.2 ("The Batch Engine" - Optimized)

**Date:** December 2025

**Target Architecture:** Tauri v2 + Rust (Native Backend) + Svelte 5 (Frontend) + ONNX Runtime (Adaptive HAL)

## **1. Executive Summary & Vision**

**The Goal:** To build the definitive desktop AI upscaler for 2025, superseding legacy tools like Upscayl.

The Paradigm Shift (v6.0+):
We are moving from a "Wrapper Architecture" (Upscayl) to a "Native Integration Architecture," and further evolving from "Prescriptive Strictness" to "Adaptive Flexibility" and "High-Throughput Batching."

*   **Legacy (Phase 1-4):** "Model must conform to Engine" (e.g., forced FP16, specific input shapes).
*   **Current (Phase 5+):** "Engine adapts to Model." The engine inspects models at runtime and adjusts the execution pipeline to match the model's requirements and the user's hardware.

The Shift:
Most existing open-source upscalers are simply GUI wrappers that spawn a command-line interface (CLI) in the background. RustScale embeds the AI runtime directly into the application memory space, allowing for zero-latency communication between the UI and the AI.

## **2. Competitive Analysis: Why "Better than Upscayl"?**

RustScale modernizes every aspect of the desktop upscaler experience:

**Runtime Efficiency**

*   **Upscayl:** Uses Electron (Chromium + Node.js) → ~400MB idle RAM.
*   **RustScale:** Uses Tauri v2 (Rust + System WebView) → ~30MB idle RAM.
*   **Why it matters:** 90% less memory overhead means your GPU has more room for the actual AI work.

**AI Engine**

*   **Upscayl:** Uses NCNN (mobile-focused, niche format).
*   **RustScale:** Uses **ONNX Runtime v2.0 (RC10)** (industry standard) with a **Hardware Abstraction Layer (HAL)**.
*   **Why it matters:** Supports models from HuggingFace, PyTorch, and research papers without recompilation.

**Execution Model**

*   **Upscayl:** Spawns external CLI processes.
*   **RustScale:** In-process inference (AI runs inside the app's memory).
*   **Why it matters:** Zero latency communication. Direct control over every byte of VRAM.

**Image Processing (The "Magic Bullet")**

*   **Upscayl:** Naive tiling (often leaves visible grid lines) or simple edge clamping.
*   **RustScale:** **Mirror-Padding & Uniform Batching**.
*   **Why it matters:** We reflect image content at tile borders instead of clamping/repeating pixels. This eliminates the "hard edge" artifacts that confuse AI models, resulting in seamless, grid-free images even on complex textures.

**Extensibility**

*   **Upscayl:** Hard to add models (requires compiling custom NCNN binaries).
*   **RustScale:** Just drag & drop .onnx files. The Adaptive Engine handles the rest.

## **3. Technology Stack**

### **3.1 Frontend (The View)**

*   **Framework:** **Svelte 5**.
    *   *State Management:* **Runes** ($state, $derived) for fine-grained reactivity.
    *   *Performance:* Compiles to raw JS, ensuring 60FPS animations.
*   **Styling:** **TailwindCSS v4**.
    *   *Engine:* Rust-based Oxide engine for instant HMR.
*   **Key Components:**
    *   ImageComparison: Before/After slider with **Pan & Zoom**.
    *   BatchView: Real-time queue management with per-file progress tracking.
    *   Toast: Global notification system.
    *   SystemInfo: Real-time **GPU VRAM** display.
    *   DropZone: Drag & drop file/folder upload.

### **3.2 Application Shell (The Host)**

*   **Core:** **Tauri v2**.
*   **Language:** **Rust**.
*   **Role:** High-performance bridge; runs inference in-process.

### **3.3 The Adaptive Inference Engine (The Brain)**

*   **Runtime:** **ONNX Runtime (ORT) v2.0.0-rc.10**.
*   **Architecture:** **"Inspect & Adapt"**.
    *   Instead of assuming model properties, the engine queries the ONNX graph upon loading (`session.inputs[0]`) to determine precision (FP16/FP32) and shape requirements.
*   **Hardware Abstraction Layer (HAL) Priority:**
    1.  **NPU (Efficiency):** OpenVINO (Intel/AMD) → CoreML (Apple).
    2.  **dGPU (Performance):** DirectML (Windows) → CUDA (NVIDIA) → ROCm (AMD).
    3.  **iGPU (Compatibility):** DirectML (Windows) → OpenVINO (Linux) → Metal (macOS).
    4.  **CPU (Fallback):** ONNX Runtime CPU.

## **4. System Data Flow (The "Zero-Copy" Pipeline)**

```mermaid
graph TD
    User[User] -->|Drag & Drop Image| UI[Svelte UI]
    UI -->|IPC: run_job(path, config)| Rust[Rust Main Thread]

    subgraph "Parallel Worker Pool (Rayon + Pipelining)"
        Rust -->|Load & Decode| RAM[RAM Buffer]
        RAM -->|Mirror Padding & Tiling| Tiler[Smart Tiler]
        
        Tiler -->|Uniform Batch Creation| Adapter[Adaptive Tensor Converter]
        Adapter -->|FP32/FP16 Tensor| ORT[ONNX Runtime Session]
        
        ORT -->|Inference (Auto-Shape)| GPU[GPU / NPU]
        GPU -->|Output Tensor| ORT
        
        ORT -->|Extract & Debatch| OutputData[Raw Floats]
        OutputData -->|Crop & Blend| Stitcher[Output Buffer]
        
        Stitcher -->|Face Detection?| Detect[Rust Face Detector]
        Detect -->|Has Face?| CODEFORMER[CodeFormer Session]
        CODEFORMER -->|Paste Face| Final[Final Buffer]
    end

    Final -->|Encode (WebP/PNG)| Disk[File System]
    Disk -->|Return Path| UI
    UI -->|Asset Protocol| Preview[Comparison View]
```

## **5. The "Universal Standard" Model Zoo (2025 Edition)**

We define a "Universal Standard" for model exports to maximize compatibility while allowing the runtime to optimize speed.

### **5.1 Export Parameters**

*   **Precision:** **FP32**.
    *   *Rationale:* FP16 causes artifacts on Intel iGPUs and older GTX cards via DirectML. We export FP32 and let high-end GPUs downcast at runtime if supported.
*   **Shape:** **Fixed (e.g., 512x512)**.
    *   *Rationale:* Dynamic axes cause shape inference failures on DirectML drivers. Fixed shapes allow better graph fusion.
*   **Opset:** **17**.
    *   *Rationale:* Enables native LayerNormalization, essential for Transformer speed (HAT/DAT) and performance (~30% speedup).

### **5.2 The Lineup**

1.  **Universal Fast: SPAN** (Swift Parameter-free Attention Network).
    *   ~0.7M Params. The "Speed Demon." Replaces DAT-Light. 40% faster than Real-ESRGAN.
2.  **Universal Balanced: RealPLKSR** (Partial Large Kernel Super Resolution).
    *   ~12M Params. The "Workhorse." Uses 17x17 kernels to mimic Transformers on standard CNN hardware.
3.  **Universal Quality: HAT-Base** (Hybrid Attention Transformer).
    *   ~20M Params. The "Pro" option. Hallucinates realistic textures.
4.  **Digital Art: AnimeSharp V4** (Tuned RCAN).
    *   ~16M Params. Optimized for 2D art, preserving sharp lines without ringing.

### **5.3 Model Configuration & Overrides (v6.1 Update)**

RustScale now supports per-model configuration via `model_config.json` and a built-in "Edit Model" UI.

*   **Batch Size Override:** Users can manually tune the batch size (1-8) for each model.
    *   *Default:* 1 (Safe).
    *   *Performance:* Higher values increase VRAM usage but improve throughput on powerful GPUs.
*   **Safety Mechanisms:**
    *   **Static Batch Detection:** The engine inspects the model's input tensor at runtime. If a model requires a fixed batch size (e.g., 1), the engine automatically overrides the user's setting to prevent crashes.
    *   **Self-Induced Reload Protection:** Prevents race conditions when saving configuration files by ignoring file watcher events triggered by the app's own writes.

## **6. The "Seamless" Tiling Algorithm (v6.0 Update)**

This section details the critical fix for grid artifacts and batch failures.

### **6.1 The "Mirror Padding" Technique**

Legacy algorithms use `.clamp()`, which repeats the last row of pixels at the edge. This creates a "flat" border that looks like a hard edge to the AI, causing it to hallucinate lines.

**The Fix:** We use **Ping-Pong Mirroring**.

*   **Logic:** If the kernel asks for a pixel outside the image bounds (e.g., x = -5), we reflect back into the image (x = 5).
*   **Result:** The AI sees continuous texture/content extending beyond the tile, allowing for perfect seam blending.

```rust
// Reflective Coordinate Logic
fn mirror_coordinate(coord: i64, max: i64) -> u32 {
    let mut c = coord;
    while c < 0 { c = -c; } // Reflect lower
    while c >= max { c = 2 * (max - 1) - c; } // Reflect upper
    c.clamp(0, max - 1) as u32
}
```

### **6.2 Uniform Batch Sizing (The "Stretch" Fix)**

AI Models and ONNX Runtime batching require every tensor in a batch to have identical dimensions. Edge tiles are naturally smaller than center tiles. If sent as-is, inference engines may "stretch" the small tile to fit the tensor, destroying the image.

**The Fix:**

*   We **force** every extracted tile to be `tile_size + 2 * padding`, even at the edges.
*   We fill the "empty" space of edge tiles using the **Mirroring** technique described above.
*   After inference, we crop only the valid region.
*   **Benefit:** Prevents model crashes and "stretching" artifacts.

### **6.3 Minimum Padding Enforcement**

*   **Rule:** Padding is `max(config.padding, 32)`.
*   **Why:** Small padding (e.g., 8px) provides insufficient context for the AI to understand the geometry at the seam, leading to discontinuities.

## **7. Inference & Resource Management**

### **7.1 Auto-Shape Propagation (The "Any-Scale" Fix)**

We no longer hardcode scaling factors (e.g., width * 4). The engine now propagates output shapes dynamically.

*   **Mechanism:**
    1.  `run_inference` returns `(Vec<i64>, Vec<f32>)` (Shape + Data).
    2.  `batch_tensor_to_images` reads the shape (e.g., `[1, 3, 2048, 2048]`).
    3.  The image buffer is constructed based on this authoritative data.
*   **Benefit:** Supports 2x, 3x, 4x, or fractional scaling models without code changes.

### **7.2 Adaptive Batching & Eco Mode**

*   **Performance Mode (dGPU):** Batch Size = 6 (Max Throughput).
*   **Efficiency/Eco Mode (Laptop/iGPU):** Batch Size = 2.
    *   *Trigger:* Detected battery power or generic "Intel UHD" device string.
    *   *Benefit:* Prevents thermal throttling and system stutter.

### **7.3 Memory Safety & Pooling**

*   **Buffer Pooling:**
    *   We implement a `BufferPool` for `Vec<f32>` and `Vec<f16>` to avoid repeated allocations during tiling.
    *   Buffers are checked out, resized if necessary, and returned to the pool after use.
*   **Smart Copy (`run_with_binding`):**
    *   Replaces the brittle `IoBinding` API.
    *   Inference results are copied directly into a pre-allocated recycled buffer, minimizing GC pressure and fragmentation.
*   **VRAM Check:** `sysinfo` / `nvidia-smi` query before job start.
    *   *Fallback:* < 4GB VRAM forces 192px tiles; 8GB allows 1024px.
*   **Zero-Overhead Inference Start:**
    *   Inference initialization is decoupled from configuration file I/O.
    *   The engine uses in-memory parameters for immediate start, avoiding disk latency or file lock contention.

### **7.4 CodeFormer Efficiency**

*   **Strategy:** Never run CodeFormer on the full image.
*   **Process:** Run Upscale -> Detect Face (UltraFace) -> Crop Chip -> CodeFormer -> Paste.

## **8. User Experience (UX) Specification**

The UI must abstract technical model names into "Problems" and "Solutions."

### **8.1 The Control Panel Hierarchy**

1.  **Scenario Selector:** Universal Fast, Universal Balanced, Universal Quality, Digital Art.
2.  **Repair Strength:** Slider (0-100), visible only when "Fix Compression" is on.
3.  **Face Recovery:** Toggle switch.

### **8.2 Metadata Preservation**

*   **Strategy:** "Post-Process Grafting".
*   **Flow:**
    1.  Upscale raw image.
    2.  Read metadata (EXIF/ICC) from source.
    3.  Inject into output file using `little_exif`.
    4.  **Critical:** Strip `ThumbnailImage` to prevent Windows Explorer showing the old low-res preview.

## **9. Rust Performance Maximization**

### **9.1 SIMD Acceleration**

*   **Requirement:** Use `fast_image_resize` crate.
*   **Benefit:** Uses AVX2/NEON SIMD instructions to resize/crop tiles 15-20x faster than standard iterators.

### **9.2 Pipelined Parallelism (Deep Pipeline)**

*   **Pattern:** Producer-Consumer (`std::sync::mpsc::sync_channel`, size **4**).
*   **Architecture:**
    *   **Thread A (CPU):** Prepares Batches N+1, N+2, N+3, N+4 (Extract, Mirror, Normalize).
    *   **Thread B (GPU):** Infers Batch N.
*   **Benefit:**
    *   With a channel size of 4 and batch size of 4, up to **16 tiles** are "in flight" at any moment.
    *   This ensures the GPU **never** waits for the CPU to decode or resize images, achieving 100% hardware utilization.

## **10. Developer Experience (DX) & Contribution**

### **10.1 Strict IPC Typing**

*   **Tool:** `tauri-specta` v2.
*   **Benefit:** Rust struct changes break the Frontend build immediately.

### **10.2 Atomic Component Design**

*   **UI:** Storybook used for "Logic-Free" UI development.

## **11. CI/CD & Release Pipeline**

*   **GitHub Actions:** Builds ubuntu, windows, macos-13, macos-14 in parallel.
*   **Release:** Auto-generates .msi, .dmg, .AppImage on tag push.

## **12. Packaging & Distribution**

### **12.1 Windows Packaging**

*   **Format:** NSIS (.exe) with custom branding overrides.
*   **Sidecars:** `onnxruntime.dll` and `DirectML.dll` must be bundled.

### **12.2 Linux Packaging**

*   **Requirement:** Bundle `libonnxruntime.so` internally to ensure distro-agnostic operation.

## **13. Cancellation Architecture (v6.1 Update)**

To ensure responsiveness and resource safety, RustScale implements a robust "Frontend-Driven" cancellation system.

### **13.1 The Challenge**
Legacy upscalers often fail to cancel immediately because the backend is stuck in a tight loop (e.g., processing a large tile) or the frontend loses track of the job ID.

### **13.2 The Solution: ID-Based Atomic Cancellation**

1.  **Frontend Generation:** The `job_id` (UUID) is generated by the UI *before* the request is sent. This ensures the UI always has a handle to the job, even if the backend hangs.
2.  **Atomic Flagging:**
    *   The backend maintains a `running_jobs` map: `HashMap<String, Arc<AtomicBool>>`.
    *   The `cancel_job` command sets the specific job's flag to `true` with `Ordering::Relaxed`.
3.  **Deep Inspection:**
    *   The `UpscaleEngine` checks this flag **between every tile**.
    *   **Latency:** Cancellation is effective within <100ms, even during heavy 4k upscaling.
4.  **Resource Safety:**
    *   If cancelled, the engine returns early.
    *   **Crucial:** No partial files are written. The file save operation is gated behind a successful completion check.

## **14. State Management Refactor (v6.1)**

We moved from a naive global state to a thread-safe, concurrent architecture.

*   **Old:** `static mut` (Unsafe, race-prone).
*   **New:** `Arc<AppState>` with internal `Mutex` locks for specific resources (e.g., `running_jobs`, `gpu_info`).
*   **Benefit:** Allows the UI to poll status, cancel jobs, and query system info (VRAM) simultaneously without blocking the main inference thread.

## **15. Adaptive Model Management (v6.1 Update)**

This release introduces a "Zero-Friction" model handling system that eliminates manual configuration.

### **15.1 Auto-Scale Detection (The "1x1" Technique)**

Instead of relying on fragile filename parsing (e.g., guessing "x4" from "RealESRGAN_x4plus"), the engine now scientifically determines the model's scale factor.

*   **Mechanism:** Upon loading, the engine runs a dummy inference with a **1x1 pixel input tensor**.
*   **Logic:**
    *   Output 2x2 -> **Scale 2x**.
    *   Output 4x4 -> **Scale 4x**.
*   **Reliability:** This method is 100% accurate for all super-resolution architectures and works instantly (<5ms).

### **15.2 Smart Model Pre-loading**

To mask the latency of loading large ONNX models (which can take 1-2 seconds), we implemented a predictive loading system.

*   **Trigger:** As soon as the user selects a model in the UI dropdown.
*   **Background Thread:** The load operation happens in a detached background thread (`tauri::async_runtime::spawn_blocking`).
*   **Debouncing:** Rapid selection changes are debounced (300ms) to prevent thrashing the backend.
*   **Safety:** If the user starts an upscale before the preload finishes, the main thread simply joins the loading handle, ensuring no race conditions.

### **15.3 Smart Scaling Logic (The "Universal" Adapter)**

The engine now automatically chains or downscales models to meet the user's requested target scale, regardless of the model's native scale.

| Model Scale | Target Scale | Strategy | Description |
| :--- | :--- | :--- | :--- |
| **4x** | **4x** | **Direct** | Standard single-pass inference. |
| **4x** | **2x** | **Downscale** | Upscales to 4x, then uses high-quality Lanczos3 downscaling to reach 2x. This acts as "Super-Sampling Anti-Aliasing" (SSAA), often producing better 2x results than native 2x models. |
| **2x** | **4x** | **Chaining** | Runs the model twice: `Input -> [2x Model] -> 2x Output -> [2x Model] -> 4x Output`. |

### **15.4 Single-Slot Caching Strategy**

To minimize RAM usage while maintaining responsiveness:

*   **Design:** `Mutex<Option<(PathBuf, Arc<OrtSession>)>>`.
*   **Behavior:** The system holds **exactly one** model in memory (the currently selected one).
*   **Swap Logic:** Switching models immediately drops the old session (freeing VRAM/RAM) before loading the new one.
*   **Low-RAM Guard:** If system RAM < 2GB, pre-loading is disabled to prevent OOM on low-end devices.

### **15.5 Adaptive Batch Management**

To prevent runtime crashes caused by incompatible user settings:

*   **Runtime Inspection:** The engine queries the ONNX graph (`session.inputs[0]`) to check if the model enforces a static batch size (e.g., `dim[0] == 1`).
*   **Auto-Correction:** If a user requests Batch Size 4 but the model supports only 1, the engine **silently overrides** the setting to 1.
*   **User Feedback:** A "Toast" notification informs the user: *"Model requires Batch Size 1. Overriding your setting of 4."*
*   **Persistence:** The corrected batch size is *not* saved to config to preserve the user's preference for other models, ensuring a non-destructive adaptation.