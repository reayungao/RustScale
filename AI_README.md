# AI Developer Guidelines for RustScale

This document is designed to help AI agents (and humans) work effectively with the RustScale codebase.

## 1. The "Hidden Error" Problem
Rust compiler errors can be extremely verbose. When running `cargo check` or `cargo build`, the output often exceeds the context window or terminal buffer of an AI agent.

**Symptoms:**
- You see `error: could not compile...` but no specific error message above it.
- You see a list of warnings but the actual error is scrolled off the top.

**Solution:**
- **DO NOT** guess the error.
- **ALWAYS** use the diagnostic script: `npm run rust:diagnose`.
- This script runs `cargo check` and captures the **FULL** output to `src-tauri/compilation_log.txt`.
- Read that file to find the root cause.

## 2. Project Structure
- `app/src-tauri/src/`: Core Rust logic.
    - `commands.rs`: IPC commands called by frontend. **Heavy logic here.**
    - `inference.rs`: ONNX Runtime wrapper. **Strict FP16 enforcement.**
    - `image_processing.rs`: Image manipulation. **Memory safe.**
    - `models.rs`: Model registry and validation.
- `app/src/`: SvelteKit Frontend.

## 3. Key Architectural Rules
1.  **Strict FP16**: All inference must use `Float16`. Do not introduce `Float32` paths unless absolutely necessary for CPU fallback.
2.  **Non-Blocking**: Never block the main thread. Use `spawn_blocking` for heavy CPU tasks (image processing) and `async` for I/O.
3.  **Ownership**: When using closures in `spawn_blocking`, **always** clone `Arc` and `String` variables *before* moving them into the closure.
    ```rust
    // GOOD
    let job_id = job_id.clone();
    spawn_blocking(move || { use(job_id); })
    
    // BAD
    spawn_blocking(move || { use(job_id); }) // Moves original, might cause issues
    ```

## 5. Logging & Diagnostics
- **Logs Location**: `app/logs/` (Daily rotation: `rustscale.log.YYYY-MM-DD`) - *Moved outside src-tauri to prevent rebuild loops.*
- **Panic Capture**: Panics are automatically logged to the file and stderr.
- **Viewing Logs**: Run `npm run rust:logs` to tail the latest log file.
- **Performance**: File logging is **non-blocking** and runs on a separate thread. It is safe to use in production.

## 6. Common Pitfalls & Lessons Learned
### 1. The "Log Loop" Crash
- **Issue**: Writing logs to `src-tauri/logs` triggers `tauri dev` to rebuild the app, which writes a startup log, triggering another rebuild.
- **Solution**: Always write logs to `app/logs/` (outside `src-tauri`) or configure the watcher to ignore the logs directory.

### 2. ORT (Onnx Runtime) Panics
- **Issue**: The `ort` crate panics (crashes the thread) if it tries to load an invalid file (e.g., a text file renamed to .onnx).
- **Solution**: Always wrap `ort` loading calls in `std::panic::catch_unwind` to handle invalid user files gracefully.

### 3. Truncated Error Output
- **Issue**: Standard `cargo check` output is often truncated in the terminal, hiding the actual error.
- **Solution**: Use `npm run rust:diagnose` to capture the full error output to a file.

## 6. Useful Commands
- `npm run rust:check`: Quick syntax check.
- `npm run rust:diagnose`: **Use this when you are stuck.** Captures full logs.
- `npm run rust:logs`: Tail the live application logs.
- `npm run rust:clippy`: Linting.
- `npm run rust:format`: Formatting.
