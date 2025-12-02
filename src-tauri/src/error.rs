use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum AppError {
    #[error("IO Error: {0}")]
    IoError(String),

    #[error("ONNX Runtime Error: {0}")]
    OrtError(String),

    #[error("Image Processing Error: {0}")]
    ImageError(String),

    #[error("VRAM Error: {0}")]
    VramError(String),

    #[error("Model Load Error: {0}")]
    ModelLoadError(String),

    #[error("Image Load Error: {0}")]
    ImageLoadError(String),

    #[error("Inference Error: {0}")]
    InferenceError(String),

    #[error("Unknown Error: {0}")]
    Unknown(String),
}

// Implement From<std::io::Error> for AppError
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err.to_string())
    }
}

// Implement From<image::ImageError> for AppError
impl From<image::ImageError> for AppError {
    fn from(err: image::ImageError) -> Self {
        AppError::ImageError(err.to_string())
    }
}

// Implement From<ort::Error> for AppError
impl From<ort::Error> for AppError {
    fn from(err: ort::Error) -> Self {
        AppError::OrtError(err.to_string())
    }
}

// Implement From<serde_json::Error> for AppError
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::IoError(format!("Serialization Error: {}", err))
    }
}

// Helper for results
pub type AppResult<T> = Result<T, AppError>;
