use crate::error::{AppError, AppResult};
use half::f16;
use image::GenericImageView;
use ort::{
    inputs,
    session::{builder::GraphOptimizationLevel, builder::SessionBuilder, Session},
    tensor::TensorElementType,
    value::{Tensor, ValueType},
};
use rayon::prelude::*;
use std::path::Path;
use std::sync::Mutex;

#[cfg(target_os = "macos")]
use ort::execution_providers::CoreMLExecutionProvider;

// --- Data Types ---
pub enum TensorData {
    Float32(Vec<f32>),
    Float16(Vec<f16>),
}

impl Clone for TensorData {
    fn clone(&self) -> Self {
        match self {
            Self::Float32(v) => Self::Float32(v.clone()),
            Self::Float16(v) => Self::Float16(v.clone()),
        }
    }
}

// --- Session Wrapper ---
pub struct OrtSession {
    pub session: Mutex<Session>,
    pub execution_provider: String,
    pub input_type: TensorElementType,
}

impl OrtSession {
    pub fn new(model_path: &Path, prefer_npu: bool) -> AppResult<Self> {
        let num_threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
            .clamp(1, 16);

        let builder = SessionBuilder::new()
            .map_err(AppError::from)?
            .with_optimization_level(GraphOptimizationLevel::Level3)
            .map_err(AppError::from)?
            .with_intra_threads(num_threads)
            .map_err(AppError::from)?
            .with_memory_pattern(true)
            .map_err(AppError::from)?
            .with_log_level(ort::logging::LogLevel::Error)
            .map_err(AppError::from)?;

        let mut session = None;

        let try_provider = |name: &str, builder: SessionBuilder| -> Option<(Session, String)> {
            match builder.commit_from_file(model_path) {
                Ok(s) => Some((s, name.to_string())),
                Err(_) => None,
            }
        };

        // --- NPU PRIORITY PATH ---
        if prefer_npu {
            #[cfg(target_os = "windows")]
            {
                if session.is_none() {
                    if let Ok(ov_builder) = builder.clone().with_execution_providers([
                        ort::execution_providers::OpenVINOExecutionProvider::default().build(),
                    ]) {
                        if let Some(s) = try_provider("OpenVINO (NPU/Auto)", ov_builder) {
                            session = Some(s);
                        }
                    }
                }
            }
            #[cfg(target_os = "macos")]
            {
                if session.is_none() {
                    if let Ok(coreml_builder) = builder.clone().with_execution_providers([
                        ort::execution_providers::CoreMLExecutionProvider::default()
                            .with_ane_only()
                            .build(),
                    ]) {
                        if let Some(s) = try_provider("CoreML (Neural Engine)", coreml_builder) {
                            session = Some(s);
                        }
                    }
                }
            }
        }

        // --- GPU PATHS ---
        #[cfg(target_os = "windows")]
        {
            if session.is_none() {
                if let Ok(dml_builder) = builder.clone().with_execution_providers([
                    ort::execution_providers::DirectMLExecutionProvider::default().build(),
                ]) {
                    if let Some(s) = try_provider("DirectML (GPU)", dml_builder) {
                        session = Some(s);
                    }
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            if session.is_none() {
                if let Ok(rocm_builder) = builder.clone().with_execution_providers([
                    ort::execution_providers::ROCmExecutionProvider::default().build(),
                ]) {
                    if let Some(s) = try_provider("ROCm (AMD)", rocm_builder) {
                        session = Some(s);
                    }
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            if session.is_none() {
                if let Ok(cuda_builder) = builder.clone().with_execution_providers([
                    ort::execution_providers::CUDAExecutionProvider::default().build(),
                ]) {
                    if let Some(s) = try_provider("CUDA (NVIDIA)", cuda_builder) {
                        session = Some(s);
                    }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            if session.is_none() {
                if let Ok(coreml_builder) = builder.clone().with_execution_providers([
                    ort::execution_providers::CoreMLExecutionProvider::default().build(),
                ]) {
                    if let Some(s) = try_provider("CoreML (NPU/GPU)", coreml_builder) {
                        session = Some(s);
                    }
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            if session.is_none() {
                if let Ok(openvino_builder) = builder.clone().with_execution_providers([
                    ort::execution_providers::OpenVINOExecutionProvider::default().build(),
                ]) {
                    if let Some(s) = try_provider("OpenVINO (Intel)", openvino_builder) {
                        session = Some(s);
                    }
                }
            }
        }

        // Fallback to CPU
        let (session, provider) = if let Some((s, p)) = session {
            (s, p)
        } else {
            let s = builder
                .commit_from_file(model_path)
                .map_err(AppError::from)?;
            (s, "CPU".to_string())
        };

        let input_type = session.inputs[0].input_type.clone();
        if let ValueType::Tensor { ty, .. } = input_type {
            tracing::info!("Model loaded: {:?} | Input Type: {:?}", model_path, ty);
            Ok(Self {
                session: Mutex::new(session),
                execution_provider: provider,
                input_type: ty,
            })
        } else {
            Err(AppError::Unknown("Model input is not a tensor".to_string()))
        }
    }

    // Internal Helper
    fn run_inference_inner(
        &self,
        session: &mut Session,
        shape: Vec<i64>,
        input_data: TensorData,
    ) -> AppResult<(Vec<i64>, Vec<f32>)> {
        let shape_usize: Vec<usize> = shape.iter().map(|&x| x as usize).collect();

        let input_tensor_value = match self.input_type {
            TensorElementType::Float32 => {
                let data_f32 = match input_data {
                    TensorData::Float32(d) => d,
                    TensorData::Float16(d) => d.iter().map(|x| x.to_f32()).collect(),
                };
                Tensor::from_array((shape_usize, data_f32))
                    .map_err(AppError::from)?
                    .into_dyn()
            }
            TensorElementType::Float16 => {
                let data_f16 = match input_data {
                    TensorData::Float16(d) => d,
                    TensorData::Float32(d) => d.iter().map(|x| f16::from_f32(*x)).collect(),
                };
                Tensor::from_array((shape_usize, data_f16))
                    .map_err(AppError::from)?
                    .into_dyn()
            }
            _ => {
                return Err(AppError::Unknown(format!(
                    "Unsupported model input type: {:?}",
                    self.input_type
                )))
            }
        };

        let outputs = session
            .run(inputs![input_tensor_value])
            .map_err(AppError::from)?;

        let output_value = outputs
            .iter()
            .next()
            .ok_or_else(|| AppError::OrtError("No output tensor found".to_string()))?
            .1;

        let is_f16 = matches!(
            output_value.dtype(),
            ValueType::Tensor {
                ty: TensorElementType::Float16,
                ..
            }
        );

        let (output_shape, output_data_f32) = if is_f16 {
            let (shape, data_f16) = output_value
                .try_extract_tensor::<f16>()
                .map_err(AppError::from)?;
            (
                shape.to_vec(),
                data_f16.iter().map(|&x| x.to_f32()).collect::<Vec<f32>>(),
            )
        } else {
            let (shape, data_f32) = output_value
                .try_extract_tensor::<f32>()
                .map_err(AppError::from)?;
            (shape.to_vec(), data_f32.to_vec())
        };

        Ok((output_shape, output_data_f32))
    }

    pub fn run_inference(
        &self,
        shape: Vec<i64>,
        input_data: TensorData,
    ) -> AppResult<(Vec<i64>, Vec<f32>)> {
        let mut session = self
            .session
            .lock()
            .map_err(|_| AppError::Unknown("Failed to lock session".to_string()))?;
        self.run_inference_inner(&mut *session, shape, input_data)
    }

    // Smart Copy Execution (Replacing brittle IoBinding)
    pub fn run_with_binding(
        &self,
        shape: Vec<i64>,
        input_buffer: &TensorData,
        output_buffer: &mut TensorData,
    ) -> AppResult<(Vec<i64>, usize)> {
        let mut session = self
            .session
            .lock()
            .map_err(|_| AppError::Unknown("Failed to lock session".to_string()))?;

        // 1. Run Standard Inference
        let (shape, data) = self.run_inference_inner(&mut *session, shape, input_buffer.clone())?;

        // 2. Copy to Recycled Buffer
        let len = data.len();
        match output_buffer {
            TensorData::Float32(buf) => {
                if buf.len() < len {
                    buf.resize(len, 0.0);
                }
                buf[..len].copy_from_slice(&data);
            }
            TensorData::Float16(buf) => {
                if buf.len() < len {
                    buf.resize(len, f16::from_f32(0.0));
                }
                for (i, &val) in data.iter().enumerate() {
                    buf[i] = f16::from_f32(val);
                }
            }
        }
        Ok((shape, len))
    }

    pub fn detect_scale(&self) -> AppResult<u32> {
        let mut session = self
            .session
            .lock()
            .map_err(|_| AppError::Unknown("Failed to lock session".to_string()))?;

        // Adaptive Scale Detection Strategy:
        // 1. Try 64x64 (Fastest, works for dynamic models)
        // 2. Try 256x256 (Standard, works for most fixed models)
        // 3. Try 512x512 (Safe, works for large fixed models like Real-ESRGAN)
        let test_sizes = [64, 256, 512];
        let mut last_error = AppError::Unknown("No sizes tested".to_string());

        for &input_dim in &test_sizes {
            let input_shape = vec![1, 3, input_dim, input_dim];
            let pixel_count = (input_dim * input_dim * 3) as usize;

            let input_data = match self.input_type {
                TensorElementType::Float16 => {
                    TensorData::Float16(vec![f16::from_f32(0.0); pixel_count])
                }
                _ => TensorData::Float32(vec![0.0; pixel_count]),
            };

            // Try inference with this size
            match self.run_inference_inner(&mut *session, input_shape, input_data) {
                Ok((output_shape, _)) => {
                    // Success! Calculate scale.
                    if output_shape.len() < 4 {
                        return Err(AppError::Unknown(
                            "Invalid output shape from model".to_string(),
                        ));
                    }
                    let out_width = output_shape[3] as u32;
                    let scale = out_width / (input_dim as u32);
                    return Ok(scale);
                }
                Err(e) => {
                    // Failed (likely invalid dimensions). Store error and try next size.
                    last_error = e;
                    continue;
                }
            }
        }

        // If we get here, all sizes failed.
        Err(last_error)
    }
}

// --- Buffer Pooling & Helpers ---
pub struct BufferPool {
    f32_buffers: Mutex<Vec<Vec<f32>>>,
    f16_buffers: Mutex<Vec<Vec<f16>>>,
}

impl BufferPool {
    pub fn new(capacity: usize) -> Self {
        Self {
            f32_buffers: Mutex::new(Vec::with_capacity(capacity)),
            f16_buffers: Mutex::new(Vec::with_capacity(capacity)),
        }
    }

    pub fn get_f32(&self, capacity: usize) -> Vec<f32> {
        let mut pool = self.f32_buffers.lock().unwrap();
        if let Some(mut buf) = pool.pop() {
            if buf.capacity() < capacity {
                buf.reserve(capacity - buf.len());
            }
            buf.clear();
            buf
        } else {
            Vec::with_capacity(capacity)
        }
    }

    pub fn return_f32(&self, mut buf: Vec<f32>) {
        let mut pool = self.f32_buffers.lock().unwrap();
        buf.clear();
        pool.push(buf);
    }

    pub fn get_f16(&self, capacity: usize) -> Vec<f16> {
        let mut pool = self.f16_buffers.lock().unwrap();
        if let Some(mut buf) = pool.pop() {
            if buf.capacity() < capacity {
                buf.reserve(capacity - buf.len());
            }
            buf.clear();
            buf
        } else {
            Vec::with_capacity(capacity)
        }
    }

    pub fn return_f16(&self, mut buf: Vec<f16>) {
        let mut pool = self.f16_buffers.lock().unwrap();
        buf.clear();
        pool.push(buf);
    }
}

use std::sync::OnceLock;

static PIXEL_LUT: OnceLock<[f32; 256]> = OnceLock::new();

fn get_pixel_lut() -> &'static [f32; 256] {
    PIXEL_LUT.get_or_init(|| {
        let mut lut = [0.0; 256];
        for (i, val) in lut.iter_mut().enumerate() {
            *val = i as f32 / 255.0;
        }
        lut
    })
}

pub fn images_to_buffer(
    images: &[image::DynamicImage],
    target_type: TensorElementType,
    buffer: &mut TensorData,
) -> AppResult<Vec<i64>> {
    if images.is_empty() {
        return Err(AppError::Unknown(
            "No images provided for batching".to_string(),
        ));
    }

    let (width, height) = images[0].dimensions();
    let batch_size = images.len();

    // Check consistency
    for (i, img) in images.iter().enumerate() {
        if img.dimensions() != (width, height) {
            return Err(AppError::Unknown(format!(
                "Batch dimension mismatch: Image 0 is {}x{}, but Image {} is {}x{}",
                width,
                height,
                i,
                img.width(),
                img.height()
            )));
        }
    }

    let pixel_count = (width * height) as usize;
    let elements_per_image = pixel_count * 3;
    let total_elements = batch_size * elements_per_image;

    match (target_type, buffer) {
        (TensorElementType::Float16, TensorData::Float16(batch_data)) => {
            if batch_data.len() < total_elements {
                batch_data.resize(total_elements, f16::from_f32(0.0));
            }

            batch_data
                .par_chunks_mut(elements_per_image)
                .zip(images.par_iter())
                .for_each(|(dest_slice, img)| {
                    let rgb = img.to_rgb8();
                    let lut = get_pixel_lut();
                    let raw_pixels = rgb.as_raw();

                    let (r_plane, rest) = dest_slice.split_at_mut(pixel_count);
                    let (g_plane, b_plane) = rest.split_at_mut(pixel_count);

                    for (i, chunk) in raw_pixels.chunks(3).enumerate() {
                        r_plane[i] = f16::from_f32(lut[chunk[0] as usize]);
                        g_plane[i] = f16::from_f32(lut[chunk[1] as usize]);
                        b_plane[i] = f16::from_f32(lut[chunk[2] as usize]);
                    }
                });

            let shape = vec![batch_size as i64, 3, height as i64, width as i64];
            Ok(shape)
        }
        (TensorElementType::Float32, TensorData::Float32(batch_data)) => {
            if batch_data.len() < total_elements {
                batch_data.resize(total_elements, 0.0);
            }

            batch_data
                .par_chunks_mut(elements_per_image)
                .zip(images.par_iter())
                .for_each(|(dest_slice, img)| {
                    let rgb = img.to_rgb8();
                    let lut = get_pixel_lut();
                    let raw_pixels = rgb.as_raw();

                    let (r_plane, rest) = dest_slice.split_at_mut(pixel_count);
                    let (g_plane, b_plane) = rest.split_at_mut(pixel_count);

                    for (i, chunk) in raw_pixels.chunks(3).enumerate() {
                        r_plane[i] = lut[chunk[0] as usize];
                        g_plane[i] = lut[chunk[1] as usize];
                        b_plane[i] = lut[chunk[2] as usize];
                    }
                });

            let shape = vec![batch_size as i64, 3, height as i64, width as i64];
            Ok(shape)
        }
        _ => Err(AppError::Unknown(format!(
            "Buffer type mismatch. Target: {:?}, Buffer: ...",
            target_type
        ))),
    }
}

pub fn images_to_batch_tensor(
    images: &[image::DynamicImage],
    target_type: TensorElementType,
) -> AppResult<(Vec<i64>, TensorData)> {
    let mut buffer = match target_type {
        TensorElementType::Float16 => TensorData::Float16(Vec::new()),
        _ => TensorData::Float32(Vec::new()),
    };
    let shape = images_to_buffer(images, target_type, &mut buffer)?;
    Ok((shape, buffer))
}

fn tensor_to_image_from_dims(
    data: &[f32],
    width: u32,
    height: u32,
) -> AppResult<image::DynamicImage> {
    let pixel_count = (width * height) as usize;
    if data.len() != pixel_count * 3 {
        return Err(AppError::Unknown(format!(
            "Tensor data length {} does not match expected size {} for {}x{}",
            data.len(),
            pixel_count * 3,
            width,
            height
        )));
    }

    let r_channel = &data[0..pixel_count];
    let g_channel = &data[pixel_count..2 * pixel_count];
    let b_channel = &data[2 * pixel_count..];

    let mut raw_buffer = vec![0u8; (width * height * 3) as usize];

    raw_buffer
        .par_chunks_mut(3)
        .enumerate()
        .for_each(|(i, pixel)| {
            let r = (r_channel[i].clamp(0.0, 1.0) * 255.0) as u8;
            let g = (g_channel[i].clamp(0.0, 1.0) * 255.0) as u8;
            let b = (b_channel[i].clamp(0.0, 1.0) * 255.0) as u8;
            pixel[0] = r;
            pixel[1] = g;
            pixel[2] = b;
        });

    let img_buffer = image::RgbImage::from_raw(width, height, raw_buffer).ok_or_else(|| {
        AppError::Unknown("Failed to create image buffer from raw data".to_string())
    })?;

    Ok(image::DynamicImage::ImageRgb8(img_buffer))
}

pub fn batch_tensor_to_images(
    inference_output: (Vec<i64>, Vec<f32>),
) -> AppResult<Vec<image::DynamicImage>> {
    let (shape, data) = inference_output;

    if shape.len() != 4 {
        return Err(AppError::Unknown(format!(
            "Unexpected output tensor rank: {}",
            shape.len()
        )));
    }

    let batch_size = shape[0] as usize;
    let height = shape[2] as u32;
    let width = shape[3] as u32;

    if batch_size == 1 {
        let image = tensor_to_image_from_dims(&data, width, height)?;
        return Ok(vec![image]);
    }

    let elements_per_image = (width * height * 3) as usize;

    if data.len() != batch_size * elements_per_image {
        return Err(AppError::Unknown(format!(
            "Batch output size mismatch. Expected {}, got {}",
            batch_size * elements_per_image,
            data.len()
        )));
    }

    let images: Result<Vec<_>, _> = (0..batch_size)
        .into_par_iter()
        .map(|i| {
            let start = i * elements_per_image;
            let end = start + elements_per_image;
            let image_data = &data[start..end];
            tensor_to_image_from_dims(image_data, width, height)
        })
        .collect();

    images
}

pub fn batch_buffer_to_images(
    shape: &[i64],
    buffer: &TensorData,
    valid_len: usize,
) -> AppResult<Vec<image::DynamicImage>> {
    match buffer {
        TensorData::Float32(data) => {
            batch_tensor_to_images((shape.to_vec(), data[..valid_len].to_vec()))
        }
        TensorData::Float16(data) => {
            let data_f32: Vec<f32> = data[..valid_len].iter().map(|x| x.to_f32()).collect();
            batch_tensor_to_images((shape.to_vec(), data_f32))
        }
    }
}
