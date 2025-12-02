use crate::error::{AppError, AppResult};
use crate::image_processing::{self, TilingConfig};
use crate::inference::OrtSession;
use crate::metadata;
use crate::state::AppState;
use image::GenericImageView;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct UpscaleConfig {
    pub model: String,
    pub scale: u32,
    pub batch_size: Option<u32>,
    pub format: Option<String>,
    pub compression: Option<String>,
    pub prefer_npu: Option<bool>,
    pub output_dir: Option<String>,
}

pub struct EngineCallbacks<P, W>
where
    P: Fn(f32, String) + Send + Sync + 'static,
    W: Fn(String) + Send + Sync + 'static,
{
    pub on_progress: P,
    pub on_warning: W,
}

pub struct UpscaleEngine;

#[derive(Clone)]
pub struct LoadedModel {
    pub session: Arc<OrtSession>,
    pub scale: u32,
    pub path: PathBuf,
    pub filename: String,
    pub recommended_tile_size: u32,
}

impl UpscaleEngine {
    pub fn load_model(config: &UpscaleConfig, app_state: Arc<AppState>) -> AppResult<LoadedModel> {
        let models_dir = std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("models");

        // 1. Resolve Model Path (Fast)
        let model_filename = if config.model.ends_with(".onnx") {
            config.model.clone()
        } else {
            format!("{}.onnx", config.model)
        };

        let model_path = models_dir.join(&model_filename);
        if !model_path.exists() {
            return Err(AppError::Unknown(format!(
                "Model file not found: {:?}",
                model_path
            )));
        }

        // 2. Load Session (Cached)
        tracing::info!("Loading model session: {}", model_filename);
        let prefer_npu = config.prefer_npu.unwrap_or(false);
        let session = app_state.get_or_load_model(&model_path, prefer_npu)?;

        // Detect Model Scale (Source of Truth - Cached)
        let scale = app_state
            .get_model_scale(&model_path, &session)
            .unwrap_or_else(|_| if model_filename.contains("x2") { 2 } else { 4 });

        // Get Recommended Tile Size
        let gpu_cache = app_state.gpu_info.lock().unwrap();
        let recommended_tile_size = if let Some(info) = &*gpu_cache {
            info.recommended_tile_size()
        } else {
            256
        };

        Ok(LoadedModel {
            session,
            scale,
            path: model_path,
            filename: model_filename,
            recommended_tile_size,
        })
    }

    pub fn process_with_session<P, W>(
        model: &LoadedModel,
        config: UpscaleConfig,
        path: PathBuf,
        callbacks: EngineCallbacks<P, W>,
        cancel_flag: Arc<AtomicBool>,
    ) -> AppResult<image::DynamicImage>
    where
        P: Fn(f32, String) + Send + Sync + 'static,
        W: Fn(String) + Send + Sync + 'static,
    {
        let job_id = Uuid::new_v4().to_string();
        tracing::info!("Starting upscale job {}: {:?}", job_id, path);

        let img_path = path.clone();
        // Load image early to fail fast
        let image = Arc::new(image_processing::load_image(&img_path)?);

        let model_scale = model.scale;
        let target_scale = config.scale;

        tracing::info!(
            "Model Scale: {}x | Target Scale: {}x",
            model_scale,
            target_scale
        );

        // 3. Just-In-Time Constraints Check
        let (fixed_input_size, is_static_batch) = {
            let s = model
                .session
                .session
                .lock()
                .map_err(|_| AppError::Unknown("Failed to lock session".to_string()))?;

            // Check Input
            let input = &s.inputs[0];
            let (fixed_size, static_batch) =
                if let ort::value::ValueType::Tensor { shape, .. } = &input.input_type {
                    let f_size = if shape.len() >= 3 && shape[2] > 0 {
                        Some(shape[2] as u32)
                    } else {
                        None
                    };

                    let s_batch = if !shape.is_empty() && shape[0] > 0 {
                        Some(shape[0] as usize)
                    } else {
                        None
                    };

                    (f_size, s_batch)
                } else {
                    (None, None)
                };

            (fixed_size, static_batch)
        };

        // 4. Configuration
        let mut batch_size = config.batch_size.unwrap_or(1).clamp(1, 8) as usize;

        if let Some(static_b) = is_static_batch {
            if static_b == 1 && batch_size > 1 {
                tracing::warn!(
                    "Model has static batch size of 1. Overriding requested batch size {} to 1.",
                    batch_size
                );
                // TRIGGER WARNING CALLBACK
                (callbacks.on_warning)(format!(
                    "Model requires Batch Size 1. Overriding your setting of {}.",
                    batch_size
                ));
                batch_size = 1;
            }
        }

        let rec_tile = model.recommended_tile_size;

        let padding = 32;
        let tile_size = if let Some(fixed_size) = fixed_input_size {
            tracing::info!("Model Constraint: Fixed input size {}", fixed_size);
            if fixed_size <= padding * 2 {
                return Err(AppError::Unknown(format!(
                    "Model input size {} is too small for padding {}",
                    fixed_size, padding
                )));
            }
            fixed_size - (padding * 2)
        } else {
            rec_tile
        };

        let tiling_config = TilingConfig {
            tile_size,
            padding,
            batch_size,
        };

        // Progress Handler
        let mut last_update = std::time::Instant::now();
        let provider_str = model.session.execution_provider.clone();

        // We wrap the generic callback to handle throttling
        let mut progress_callback = move |progress: f32| {
            let now = std::time::Instant::now();
            if progress >= 1.0 || now.duration_since(last_update).as_millis() >= 100 {
                (callbacks.on_progress)(progress, provider_str.clone());
                last_update = now;
            }
        };

        let buffer_pool = std::sync::Arc::new(crate::inference::BufferPool::new(6));
        let total_batches_processed = Arc::new(AtomicUsize::new(0));
        let batches_counter = total_batches_processed.clone();
        let batches_counter_clone = batches_counter.clone();
        let inference_start_time = std::time::Instant::now();

        // Inference Callback Factory
        let create_inference_callback = |session: Arc<OrtSession>| {
            let buffer_pool = buffer_pool.clone();
            let batches_counter_clone = batches_counter_clone.clone();
            let inference_start_time = inference_start_time.clone();

            move |tiles: Vec<image::DynamicImage>| -> AppResult<Vec<image::DynamicImage>> {
                if tiles.is_empty() {
                    return Ok(Vec::new());
                }

                let current_batch_idx = batches_counter_clone.fetch_add(1, Ordering::Relaxed) + 1;

                // Log occasionally
                if current_batch_idx == 1 || current_batch_idx % 10 == 0 {
                    let elapsed = inference_start_time.elapsed().as_secs_f32();
                    let tps = current_batch_idx as f32 / elapsed;
                    tracing::info!(
                        "Pipeline Active | Batch #{} | TPS: {:.1}",
                        current_batch_idx,
                        tps
                    );
                }

                let target_type = {
                    let s = session
                        .session
                        .lock()
                        .map_err(|_| AppError::Unknown("Lock fail".to_string()))?;
                    match &s.inputs[0].input_type {
                        ort::value::ValueType::Tensor { ty, .. } => ty.clone(),
                        _ => ort::tensor::TensorElementType::Float32,
                    }
                };

                // Buffer Management
                let est_size = tiles[0].width() as usize * tiles[0].height() as usize * 3;
                let mut input_buffer = match target_type {
                    ort::tensor::TensorElementType::Float16 => {
                        crate::inference::TensorData::Float16(buffer_pool.get_f16(est_size))
                    }
                    _ => crate::inference::TensorData::Float32(buffer_pool.get_f32(est_size)),
                };
                let mut output_buffer = match target_type {
                    ort::tensor::TensorElementType::Float16 => {
                        crate::inference::TensorData::Float16(buffer_pool.get_f16(est_size * 16))
                    }
                    _ => crate::inference::TensorData::Float32(buffer_pool.get_f32(est_size * 16)),
                };

                let shape =
                    crate::inference::images_to_buffer(&tiles, target_type, &mut input_buffer)?;

                // Standard Run (Safe - No I/O Binding)
                let result = session.run_with_binding(shape, &input_buffer, &mut output_buffer);

                let (out_shape, valid_len) = match result {
                    Ok(res) => res,
                    Err(e) => {
                        // Check for OOM
                        let err_str = e.to_string().to_lowercase();
                        if err_str.contains("memory")
                            || err_str.contains("allocate")
                            || err_str.contains("vram")
                        {
                            return Err(AppError::Unknown(format!(
                                "Out of Memory. Try closing other apps. (Technical: {})",
                                e
                            )));
                        } else {
                            return Err(e);
                        }
                    }
                };

                let result_images = crate::inference::batch_buffer_to_images(
                    &out_shape,
                    &output_buffer,
                    valid_len,
                )?;

                // Return buffers to pool
                match input_buffer {
                    crate::inference::TensorData::Float32(b) => buffer_pool.return_f32(b),
                    crate::inference::TensorData::Float16(b) => buffer_pool.return_f16(b),
                }
                match output_buffer {
                    crate::inference::TensorData::Float32(b) => buffer_pool.return_f32(b),
                    crate::inference::TensorData::Float16(b) => buffer_pool.return_f16(b),
                }
                Ok(result_images)
            }
        };

        // --- SMART SCALING LOGIC ---
        let final_image = if model_scale == target_scale {
            // CASE A: Match (e.g. 4x -> 4x)
            tracing::info!("Scaling Match. Running single pass.");
            image_processing::process_tiled(
                &image,
                tiling_config,
                model_scale,
                &cancel_flag,
                &mut progress_callback,
                create_inference_callback(model.session.clone()),
            )?
        } else if model_scale > target_scale {
            // CASE B: Downscale (e.g. 4x -> 2x)
            tracing::info!(
                "Downscaling required (Model {}x -> Target {}x).",
                model_scale,
                target_scale
            );
            let upscaled = image_processing::process_tiled(
                &image,
                tiling_config,
                model_scale,
                &cancel_flag,
                &mut progress_callback,
                create_inference_callback(model.session.clone()),
            )?;

            let (w, h) = upscaled.dimensions();
            let target_w = (w as f32 * (target_scale as f32 / model_scale as f32)) as u32;
            let target_h = (h as f32 * (target_scale as f32 / model_scale as f32)) as u32;

            tracing::info!(
                "Resizing output from {}x{} to {}x{}",
                w,
                h,
                target_w,
                target_h
            );
            // Log the color type of the image being resized
            tracing::info!("Image Color Type before resize: {:?}", upscaled.color());
            image_processing::resize_image(&upscaled, target_w, target_h)?
        } else {
            // CASE C: Double Upscale (e.g. 2x -> 4x)
            tracing::info!(
                "Double Upscale required (Model {}x -> Target {}x).",
                model_scale,
                target_scale
            );

            // Pass 1
            tracing::info!("Pass 1/2: Upscaling {}x...", model_scale);
            let pass1 = image_processing::process_tiled(
                &image,
                tiling_config.clone(), // Clone for first pass
                model_scale,
                &cancel_flag,
                // We should probably wrap progress to show 0-50%
                |_| { /* ignore for now or implement split progress */ },
                create_inference_callback(model.session.clone()),
            )?;

            // Pass 2
            tracing::info!("Pass 2/2: Upscaling {}x again...", model_scale);
            image_processing::process_tiled(
                &pass1,        // Use output of pass 1
                tiling_config, // Move original here
                model_scale,
                &cancel_flag,
                &mut progress_callback, // Show 50-100%? Or just let it jump.
                create_inference_callback(model.session.clone()),
            )?
        };

        let total_batches = batches_counter.load(Ordering::Relaxed);
        let total_duration = inference_start_time.elapsed().as_secs_f32();
        let avg_tps = total_batches as f32 / total_duration;

        tracing::info!(
            "Inference Complete | Total Batches: {} | Duration: {:.2}s | Avg TPS: {:.2}",
            total_batches,
            total_duration,
            avg_tps
        );

        Ok(final_image)
    }

    pub fn save_result(
        image: image::DynamicImage,
        config: UpscaleConfig,
        source_path: PathBuf,
    ) -> AppResult<String> {
        // Save Result
        let file_stem = source_path.file_stem().unwrap().to_string_lossy();
        let ext = source_path.extension().unwrap().to_string_lossy();
        let target_ext = config.format.clone().unwrap_or(ext.to_string());

        let out_path = if let Some(ref dir) = config.output_dir {
            let out_dir = PathBuf::from(dir);
            if !out_dir.exists() {
                std::fs::create_dir_all(&out_dir).map_err(|e| AppError::Unknown(e.to_string()))?;
            }
            out_dir.join(format!("{}_upscaled.{}", file_stem, target_ext))
        } else {
            source_path.with_file_name(format!("{}_upscaled.{}", file_stem, target_ext))
        };

        let compression = config.compression.clone().unwrap_or("lossy".to_string());

        // OPTIMIZATION: Encode to memory first, then graft metadata, then write to disk ONCE.
        let format = image::ImageFormat::from_path(&out_path).unwrap_or(image::ImageFormat::Png);

        let pp_start = std::time::Instant::now();
        let image_data = image_processing::encode_image(&image, format, &compression)?;
        let encode_duration = pp_start.elapsed().as_secs_f32();

        let meta_start = std::time::Instant::now();
        if let Err(e) = metadata::save_with_metadata(image_data, &out_path, &source_path) {
            eprintln!("Warning: Failed to save with metadata: {}", e);
            // Fallback: Try to save normally if metadata fails
            image_processing::encode_image(&image, format, &compression).and_then(|data| {
                std::fs::write(&out_path, data).map_err(|e| AppError::Unknown(e.to_string()))
            })?;
        }
        let meta_duration = meta_start.elapsed().as_secs_f32();

        tracing::info!(
            "Post-Processing | Encode: {:.2}s | Metadata+Save: {:.2}s | Total: {:.2}s",
            encode_duration,
            meta_duration,
            pp_start.elapsed().as_secs_f32()
        );

        Ok(out_path.to_string_lossy().to_string())
    }

    pub fn run<P, W>(
        config: UpscaleConfig,
        path: PathBuf,
        app_state: Arc<AppState>,
        callbacks: EngineCallbacks<P, W>,
        cancel_flag: Arc<AtomicBool>,
    ) -> AppResult<String>
    where
        P: Fn(f32, String) + Send + Sync + 'static,
        W: Fn(String) + Send + Sync + 'static,
    {
        // 1. Load Model
        let loaded_model = Self::load_model(&config, app_state)?;

        // 2. Process (Inference)
        let final_image = Self::process_with_session(
            &loaded_model,
            config.clone(),
            path.clone(),
            callbacks,
            cancel_flag,
        )?;

        // 3. Save (Synchronous for Single File)
        Self::save_result(final_image, config, path)
    }
}
