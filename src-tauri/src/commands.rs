use crate::error::AppError;
use crate::image_processing;
use crate::state::AppState;
use image::GenericImageView;
use std::fs::File;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{Emitter, Manager, State};
use uuid::Uuid;

use crate::engine::{EngineCallbacks, UpscaleConfig, UpscaleEngine};

use crate::models::{ModelManifest, ModelScanner, ModelUserConfig};

#[derive(serde::Serialize, Clone)]
struct ProgressPayload {
    job_id: String,
    progress: f32,
    status: String,
    execution_provider: String,
    current_file: Option<String>,
}

#[derive(serde::Serialize)]
pub struct PreloadResponse {
    pub scale: u32,
    pub batch_size: Option<u32>,
}

#[derive(serde::Serialize)]
pub struct BatchReport {
    pub successful: Vec<String>,
    pub failed: Vec<(String, String)>,
}

#[tauri::command]
pub async fn preload_model(
    app_handle: tauri::AppHandle,
    _state: State<'_, AppState>,
    model_filename: String,
    prefer_npu: bool,
    execution_provider: Option<String>,
) -> Result<PreloadResponse, AppError> {
    let app_handle_clone = app_handle.clone();

    // Spawn blocking to avoid freezing UI during heavy model load
    tauri::async_runtime::spawn_blocking(move || {
        let state = app_handle_clone.state::<AppState>();
        let models_dir = std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("models");
        let model_path = models_dir.join(&model_filename);

        if !model_path.exists() {
            return Err(AppError::Unknown("Model file not found".to_string()));
        }

        // Load or Get from Cache
        let session = state.get_or_load_model(&model_path, prefer_npu, execution_provider)?;

        // Detect Scale (Cached)
        let scale = match state.get_model_scale(&model_path, &session) {
            Ok(s) => s,
            Err(e) => {
                tracing::warn!(
                    "Failed to detect scale for {}: {}. Fallback to filename/manifest.",
                    model_filename,
                    e
                );
                // Fallback: Try to guess from filename
                if model_filename.contains("x2") {
                    2
                } else if model_filename.contains("x3") {
                    3
                } else {
                    4
                }
            }
        };

        Ok(PreloadResponse {
            scale,
            batch_size: None, // Frontend has manifest for this
        })
    })
    .await
    .map_err(|e| AppError::Unknown(e.to_string()))?
}

#[tauri::command]
pub async fn upscale_image(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    path: String,
    config: UpscaleConfig,
    id: Option<String>,
) -> Result<String, AppError> {
    let job_id = id.unwrap_or_else(|| Uuid::new_v4().to_string());
    let cancel_flag = Arc::new(AtomicBool::new(false));

    tracing::info!("Starting upscale job {}: {}", job_id, path);

    // Register job for cancellation
    {
        let mut running_jobs = state.running_jobs.lock().unwrap();
        running_jobs.insert(job_id.clone(), cancel_flag.clone());
    }

    let app_handle_clone = app_handle.clone();
    let job_id_clone = job_id.clone();

    // Define Callbacks
    let callbacks = EngineCallbacks {
        on_progress: move |progress, provider| {
            let _ = app_handle_clone.emit(
                "upscale-progress",
                ProgressPayload {
                    job_id: job_id_clone.clone(),
                    progress,
                    status: "processing".to_string(),
                    execution_provider: provider,
                    current_file: None,
                },
            );
        },
        on_warning: move |msg| {
            let _ = app_handle.emit("batch-size-warning", msg);
        },
    };

    let path_buf = PathBuf::from(path);
    // Clone AppState for the thread (now possible as AppState derives Clone)
    let app_state = Arc::new(state.inner().clone());

    // Spawn blocking task via the Engine
    // We use spawn_blocking here to keep the command async but run heavy work on thread pool
    let result = tauri::async_runtime::spawn_blocking(move || {
        UpscaleEngine::run(config, path_buf, app_state, callbacks, cancel_flag)
    })
    .await
    .map_err(|e| AppError::Unknown(e.to_string()))??;

    // Cleanup
    {
        let mut running_jobs = state.running_jobs.lock().unwrap();
        running_jobs.remove(&job_id);
    }

    Ok(result)
}

#[tauri::command]
pub async fn scan_paths(paths: Vec<String>) -> Result<Vec<String>, AppError> {
    let mut collected_paths = Vec::new();
    for path_str in paths {
        let path = PathBuf::from(path_str);
        if path.is_dir() {
            // Recursive scan with depth limit
            scan_dir(&path, &mut collected_paths, 0, 20)?;
        } else if is_supported_image(&path) {
            collected_paths.push(path.to_string_lossy().to_string());
        }
    }
    Ok(collected_paths)
}

fn scan_dir(
    dir: &PathBuf,
    collector: &mut Vec<String>,
    depth: u32,
    max_depth: u32,
) -> std::io::Result<()> {
    if depth > max_depth {
        return Ok(());
    }
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    scan_dir(&path, collector, depth + 1, max_depth)?;
                } else if is_supported_image(&path) {
                    collector.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    Ok(())
}

fn is_supported_image(path: &PathBuf) -> bool {
    if let Some(ext) = path.extension() {
        if let Some(ext_str) = ext.to_str() {
            let ext_lower = ext_str.to_lowercase();
            return matches!(ext_lower.as_str(), "png" | "jpg" | "jpeg" | "webp");
        }
    }
    false
}

#[tauri::command]
pub async fn upscale_multiple(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    paths: Vec<String>,
    config: UpscaleConfig,
    id: Option<String>,
) -> Result<BatchReport, AppError> {
    let job_id = id.unwrap_or_else(|| Uuid::new_v4().to_string());
    let cancel_flag = Arc::new(AtomicBool::new(false));

    tracing::info!("Starting batch job {}: {} files", job_id, paths.len());

    // Register job for cancellation
    {
        let mut running_jobs = state.running_jobs.lock().unwrap();
        running_jobs.insert(job_id.clone(), cancel_flag.clone());
    }

    let mut save_handles = Vec::new();
    let mut successful = Vec::new();
    let mut failed = Vec::new();
    let total = paths.len();

    // 1. Load Model ONCE (Optimization)
    let loaded_model = UpscaleEngine::load_model(&config, Arc::new(state.inner().clone()))
        .map_err(|e| AppError::Unknown(e.to_string()))?;

    tracing::info!("Batch Model Loaded: {}", loaded_model.filename);

    for (index, path_str) in paths.into_iter().enumerate() {
        // Check cancellation
        if cancel_flag.load(Ordering::Relaxed) {
            tracing::info!("Batch job {} cancelled at index {}", job_id, index);
            break;
        }

        let path_buf = PathBuf::from(&path_str);

        // Emit progress for batch
        let _ = app_handle.emit(
            "batch-progress",
            serde_json::json!({
                "job_id": job_id,
                "current": index + 1,
                "total": total,
                "current_file": path_str
            }),
        );

        // Determine Output Directory per file (Safety: Handles mixed sources and collisions)
        let mut file_config = config.clone();
        if let Some(parent) = path_buf.parent() {
            let out_dir = parent.join("upscaled");
            file_config.output_dir = Some(out_dir.to_string_lossy().to_string());
        }

        // Run inference
        let model_clone = loaded_model.clone(); // Cheap clone (Arc)
        let config_clone = file_config.clone(); // Clone for inference
        let config_for_save = file_config; // Move for save
        let path_clone = path_buf.clone();
        let path_for_save = path_buf;
        let cancel_flag_clone = cancel_flag.clone();
        let job_id_clone = job_id.clone();
        let app_handle_clone_progress = app_handle.clone();
        let app_handle_clone_warning = app_handle.clone();

        let current_file_path = path_str.clone();
        let callbacks = EngineCallbacks {
            on_progress: move |progress, provider| {
                let _ = app_handle_clone_progress.emit(
                    "upscale-progress",
                    ProgressPayload {
                        job_id: job_id_clone.clone(),
                        progress,
                        status: "processing".to_string(),
                        execution_provider: provider,
                        current_file: Some(current_file_path.clone()),
                    },
                );
            },
            on_warning: move |msg| {
                let _ = app_handle_clone_warning.emit("batch-size-warning", msg);
            },
        };

        // STEP 1: INFERENCE (Blocking/Awaited)
        // We wait for this to finish so the GPU is free for the next one.
        let inference_result = tauri::async_runtime::spawn_blocking(move || {
            UpscaleEngine::process_with_session(
                &model_clone,
                config_clone,
                path_clone,
                callbacks,
                cancel_flag_clone,
            )
        })
        .await
        .map_err(|e| AppError::Unknown(e.to_string()));

        match inference_result {
            Ok(Ok(final_image)) => {
                // STEP 2: SAVE (Async/Background)
                // We spawn this and DO NOT await it immediately.
                let app_handle_for_save = app_handle.clone();
                let job_id_for_save = job_id.clone();
                let path_str_for_save = path_str.clone();

                let save_handle = tauri::async_runtime::spawn_blocking(move || {
                    match UpscaleEngine::save_result(final_image, config_for_save, path_for_save) {
                        Ok(out_path) => {
                            // Emit success event HERE (async)
                            let _ = app_handle_for_save.emit(
                                "file-complete",
                                serde_json::json!({
                                    "job_id": job_id_for_save,
                                    "file": path_str_for_save,
                                    "status": "success"
                                }),
                            );
                            Ok(out_path)
                        }
                        Err(e) => {
                            tracing::error!("Failed to save {}: {}", path_str_for_save, e);
                            let _ = app_handle_for_save.emit(
                                "file-error",
                                serde_json::json!({
                                    "job_id": job_id_for_save,
                                    "file": path_str_for_save,
                                    "error": e.to_string()
                                }),
                            );
                            Err(e)
                        }
                    }
                });
                save_handles.push((path_str.clone(), save_handle));
            }
            Ok(Err(e)) => {
                // Inference failed
                tracing::error!("Failed to process {}: {}", path_str, e);
                failed.push((path_str.clone(), e.to_string()));
                let _ = app_handle.emit(
                    "file-error",
                    serde_json::json!({
                        "job_id": job_id,
                        "file": path_str,
                        "error": e.to_string()
                    }),
                );
            }
            Err(e) => {
                // Join error
                tracing::error!("Task join failed for {}: {}", path_str, e);
                failed.push((path_str.clone(), e.to_string()));
            }
        }
    }

    // STEP 3: FINALIZE
    // Await all save tasks to ensure report is accurate
    for (path_str, handle) in save_handles {
        match handle.await {
            Ok(Ok(out_path)) => {
                successful.push(out_path);
            }
            Ok(Err(e)) => {
                failed.push((path_str, e.to_string()));
            }
            Err(e) => {
                failed.push((path_str, e.to_string()));
            }
        }
    }

    // Cleanup
    {
        let mut running_jobs = state.running_jobs.lock().unwrap();
        running_jobs.remove(&job_id);
    }

    Ok(BatchReport { successful, failed })
}

#[tauri::command]
pub async fn cancel_job(state: State<'_, AppState>, job_id: String) -> Result<(), AppError> {
    let running_jobs = state.running_jobs.lock().unwrap();
    if let Some(flag) = running_jobs.get(&job_id) {
        flag.store(true, Ordering::Relaxed);
    }
    Ok(())
}

#[tauri::command]
pub async fn get_system_info() -> Result<serde_json::Value, AppError> {
    use crate::gpu::GpuInfo;
    let gpu_info = GpuInfo::detect()?;
    Ok(serde_json::json!({
        "gpu_name": gpu_info.name,
        "vram_usage": gpu_info.vram_used_mb,
        "total_vram": gpu_info.vram_total_mb,
        "gpu_vendor": format!("{:?}", gpu_info.vendor),
        "detection_method": gpu_info.detection_method,
        "is_gpu_vram": gpu_info.vendor != crate::gpu::GpuVendor::Unknown,
        "is_npu": gpu_info.is_npu,
        "recommended_tile_size": gpu_info.recommended_tile_size()
    }))
}

#[tauri::command]
pub async fn generate_preview(path: String) -> Result<String, AppError> {
    let img_path = PathBuf::from(&path);
    let image = image_processing::load_image(&img_path)?;
    let (w, h) = image.dimensions();
    if w <= 2000 && h <= 2000 {
        return Ok(path);
    }
    let scale_factor = 1920.0 / w.max(h) as f32;
    let new_w = (w as f32 * scale_factor) as u32;
    let new_h = (h as f32 * scale_factor) as u32;
    let preview = image_processing::resize_image(&image, new_w, new_h)?;
    let temp_dir = std::env::temp_dir().join("rustscale_previews");
    if !temp_dir.exists() {
        std::fs::create_dir_all(&temp_dir).map_err(|e| AppError::Unknown(e.to_string()))?;
    }
    let preview_path = temp_dir.join(format!("preview_{}.webp", Uuid::new_v4()));
    image_processing::save_image(&preview, &preview_path, "lossy")?;
    Ok(preview_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn get_models() -> Result<Vec<crate::models::ModelManifest>, AppError> {
    let models_dir = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("models");

    let manifests = ModelScanner::scan_directory(&models_dir)?;
    Ok(manifests)
}

#[tauri::command]
pub async fn update_model_info(
    _app: tauri::AppHandle, // AppHandle not needed for paths anymore
    id: String,
    name: String,
    description: String,
    batch_size: Option<u32>,
) -> Result<ModelManifest, String> {
    tracing::debug!("[Backend] update_model_info called for id: {}", id);
    tracing::debug!("[Backend] Received batch_size: {:?}", batch_size);

    // FIX: Use the same directory as get_models (current_dir/models)
    let models_dir = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("models");

    // Ensure directory exists
    if !models_dir.exists() {
        std::fs::create_dir_all(&models_dir).map_err(|e| e.to_string())?;
    }

    let config_path = models_dir.join("model_config.json");

    let mut user_config = if config_path.exists() {
        let file = File::open(&config_path).map_err(|e| e.to_string())?;
        serde_json::from_reader(file).unwrap_or_default()
    } else {
        ModelUserConfig::default()
    };

    let info = user_config.overrides.entry(id.clone()).or_default();
    info.name = name;
    info.description = description;
    info.batch_size = batch_size;

    tracing::debug!(
        "[Backend] Saving config to disk. Overrides for {}: {:?}",
        id,
        info
    );

    let file = File::create(&config_path).map_err(|e| e.to_string())?;
    serde_json::to_writer_pretty(file, &user_config).map_err(|e| e.to_string())?;

    tracing::debug!("[Backend] Config saved. Rescanning model...");

    let model_path = models_dir.join(&id);
    if model_path.exists() {
        // CRITICAL FIX: Pass the in-memory 'config' we just updated!
        // This ensures the returned manifest reflects the changes immediately,
        // even if the file system hasn't fully synced or if 'load' would hit a race condition.
        let manifest =
            ModelScanner::scan_file(&model_path, &user_config).map_err(|e| e.to_string())?;
        tracing::debug!(
            "[Backend] Scan complete. Returning manifest with batch_size: {:?}",
            manifest.batch_size
        );
        Ok(manifest)
    } else {
        // Fallback if file not found (shouldn't happen if ID is valid)
        Err("Model file not found after update".to_string())
    }
}

#[tauri::command]
pub async fn reset_model_info(_app: tauri::AppHandle, id: String) -> Result<(), String> {
    // FIX: Use the same directory as get_models (current_dir/models)
    let models_dir = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("models");

    let config_path = models_dir.join("model_config.json");

    let mut config = if config_path.exists() {
        let file = File::open(&config_path).map_err(|e| e.to_string())?;
        serde_json::from_reader(file).unwrap_or_default()
    } else {
        ModelUserConfig::default()
    };

    if config.overrides.remove(&id).is_some() {
        let file = File::create(&config_path).map_err(|e| e.to_string())?;
        serde_json::to_writer_pretty(file, &config).map_err(|e| e.to_string())?;
    }
    Ok(())
}
