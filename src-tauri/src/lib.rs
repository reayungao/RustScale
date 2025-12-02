pub mod error;
// Updated to match the new filename (image_processing.rs)
pub mod commands;
pub mod engine;
pub mod gpu;
pub mod image_processing;
pub mod inference;
pub mod logging;
pub mod metadata;
pub mod models;
pub mod state;

#[cfg(test)]
mod tests;

use state::AppState;
use tauri::Emitter;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::upscale_image,
            commands::cancel_job,
            commands::get_system_info,
            commands::generate_preview,
            commands::get_models,
            commands::update_model_info,
            commands::reset_model_info,
            commands::preload_model,
            commands::scan_paths,
            commands::upscale_multiple
        ])
        .setup(|app| {
            // Initialize Logging
            logging::init_logging(app.handle());

            let app_handle = app.handle().clone();

            // Initialize AppState
            app.manage(AppState::new(&app_handle));

            // Spawn background task to detect GPU info
            let app_handle_clone = app_handle.clone();
            tauri::async_runtime::spawn_blocking(move || {
                if let Ok(info) = crate::gpu::GpuInfo::detect() {
                    let state = app_handle_clone.state::<AppState>();
                    let mut gpu_cache = state.gpu_info.lock().unwrap();
                    *gpu_cache = Some(info);
                }
            });

            // Spawn background task to watch models directory
            let app_handle_clone = app_handle.clone();
            tauri::async_runtime::spawn_blocking(move || {
                use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
                use std::sync::mpsc::channel;
                use std::time::Duration;

                let models_dir = std::env::current_dir()
                    .unwrap_or_else(|_| std::path::PathBuf::from("."))
                    .join("models");

                if !models_dir.exists() {
                    return;
                }

                let (tx, rx) = channel();

                // Debounce events slightly to avoid double-firing
                let config = Config::default().with_poll_interval(Duration::from_secs(2));
                let mut watcher: RecommendedWatcher = Watcher::new(tx, config).unwrap();

                if let Err(e) = watcher.watch(&models_dir, RecursiveMode::NonRecursive) {
                    eprintln!("Failed to watch models directory: {}", e);
                    return;
                }

                // Loop forever handling events
                for res in rx {
                    match res {
                        Ok(event) => {
                            // Filter for relevant events (Create, Remove, Modify, Rename)
                            // and ignore temp files if possible
                            match event.kind {
                                notify::EventKind::Create(_)
                                | notify::EventKind::Remove(_)
                                | notify::EventKind::Modify(_) => {
                                    // 1. Invalidate Scale Cache for this file
                                    // This ensures that if a user replaces a broken model, we re-detect it.
                                    let state = app_handle_clone.state::<AppState>();
                                    for path in &event.paths {
                                        state.invalidate_cache(path);
                                    }

                                    // 2. Emit event to frontend
                                    // We use emit_all because this is a global app state change
                                    let _ = app_handle_clone.emit("models-changed", ());
                                }
                                _ => {}
                            }
                        }
                        Err(e) => eprintln!("Watch error: {}", e),
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
