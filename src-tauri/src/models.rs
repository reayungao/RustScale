use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelManifest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub filename: String,
    pub scale: u32,
    pub alignment: u32,
    pub batch_size: Option<u32>,
}

impl ModelManifest {
    pub fn new(
        id: &str,
        name: &str,
        description: &str,
        filename: &str,
        scale: u32,
        alignment: u32,
        batch_size: Option<u32>,
    ) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            filename: filename.to_string(),
            scale,
            alignment,
            batch_size,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelUserConfig {
    pub overrides: HashMap<String, UserModelInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserModelInfo {
    pub name: String,
    pub description: String,
    pub batch_size: Option<u32>,
}

impl ModelUserConfig {
    pub fn load(path: &Path) -> Self {
        let mut attempts = 0;
        while attempts < 5 {
            if let Ok(file) = std::fs::File::open(path) {
                if let Ok(config) = serde_json::from_reader(&file) {
                    return config;
                }
            }
            // If we fail (file locked, empty, partial write), wait and retry
            std::thread::sleep(std::time::Duration::from_millis(100));
            attempts += 1;
        }

        // If it still fails, try one last time or return default.
        // We log this as it indicates a persistent issue.
        if path.exists() {
            eprintln!(
                "Warning: Failed to load model_config.json after retries. Returning default."
            );
        }
        Self::default()
    }

    pub fn save(&self, path: &Path) -> AppResult<()> {
        let file = std::fs::File::create(path).map_err(|e| AppError::Unknown(e.to_string()))?;
        serde_json::to_writer_pretty(file, self).map_err(|e| AppError::Unknown(e.to_string()))?;
        Ok(())
    }
}

pub struct ModelScanner;

impl ModelScanner {
    // NEW: Scan a single file efficiently using provided config
    pub fn scan_file(path: &Path, user_config: &ModelUserConfig) -> AppResult<ModelManifest> {
        // println!("[DIAGNOSTIC] [ModelScanner] Scanning file: {:?}", path);
        if path.extension().and_then(|s| s.to_str()) != Some("onnx") {
            return Err(AppError::Unknown("File is not an ONNX model".to_string()));
        }

        let filename = path.file_name().unwrap().to_string_lossy().to_string();
        let id = filename.clone();

        // FAST SCAN: Infer from filename only. No heavy inspection.
        let scale = if filename.contains("x2") {
            2
        } else if filename.contains("x3") {
            3
        } else {
            4
        };

        let arch_lower = filename.to_lowercase();

        // Default alignment (8 for HAT/Swin, 1 for others)
        let alignment = if arch_lower.contains("hat") {
            8
        } else if arch_lower.contains("dat") {
            16
        } else {
            1
        };

        let (name, description, batch_size) = if let Some(info) = user_config.overrides.get(&id) {
            // println!(
            //     "[DIAGNOSTIC] [ModelScanner] Found override for {}: batch_size={:?}",
            //     id, info.batch_size
            // );
            (info.name.clone(), info.description.clone(), info.batch_size)
        } else {
            // println!("[DIAGNOSTIC] [ModelScanner] No override found for {}", id);
            (
                filename.replace(".onnx", ""),
                format!("{}x Upscaling Model", scale),
                None,
            )
        };

        Ok(ModelManifest::new(
            &id,
            &name,
            &description,
            &filename,
            scale,
            alignment,
            batch_size,
        ))
    }

    pub fn scan_directory(dir: &Path) -> AppResult<Vec<ModelManifest>> {
        let mut manifests = Vec::new();

        if !dir.exists() {
            return Ok(manifests);
        }

        // Load config ONCE for the entire directory scan
        let config_path = dir.join("model_config.json");
        let user_config = ModelUserConfig::load(&config_path);

        for entry in std::fs::read_dir(dir).map_err(|e| AppError::Unknown(e.to_string()))? {
            let entry = entry.map_err(|e| AppError::Unknown(e.to_string()))?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("onnx") {
                if let Ok(manifest) = Self::scan_file(&path, &user_config) {
                    manifests.push(manifest);
                }
            }
        }

        Ok(manifests)
    }
}
