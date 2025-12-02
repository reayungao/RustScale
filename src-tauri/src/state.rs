use crate::error::AppResult;
use crate::inference::OrtSession;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use sysinfo::System;
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub path: String,
    pub status: String, // "pending", "processing", "done", "failed"
    pub model: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JobStore {
    pub jobs: Vec<Job>,
}

use crate::gpu::GpuInfo;

#[derive(Clone)] // Added Clone
pub struct AppState {
    pub store: Arc<Mutex<JobStore>>,
    pub running_jobs: Arc<Mutex<HashMap<String, Arc<AtomicBool>>>>,
    // Single-Slot Cache: Only holds the currently selected model.
    // We store (Path, Session) to check if the requested model is already loaded.
    pub model_cache: Arc<Mutex<Option<(PathBuf, Arc<OrtSession>)>>>,
    pub scale_cache: Arc<Mutex<HashMap<PathBuf, u32>>>,
    pub gpu_info: Arc<Mutex<Option<GpuInfo>>>,
    pub app_data_dir: PathBuf,
}

impl AppState {
    pub fn new(app_handle: &tauri::AppHandle) -> Self {
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .expect("failed to get app data dir");

        let store = Self::load_jobs(&app_data_dir).unwrap_or_default();

        Self {
            store: Arc::new(Mutex::new(store)),
            running_jobs: Arc::new(Mutex::new(HashMap::new())),
            model_cache: Arc::new(Mutex::new(None)),
            scale_cache: Arc::new(Mutex::new(HashMap::new())),
            gpu_info: Arc::new(Mutex::new(None)),
            app_data_dir,
        }
    }

    fn get_store_path(app_data_dir: &PathBuf) -> PathBuf {
        app_data_dir.join("jobs.json")
    }

    pub fn load_jobs(app_data_dir: &PathBuf) -> AppResult<JobStore> {
        let path = Self::get_store_path(app_data_dir);
        if !path.exists() {
            return Ok(JobStore::default());
        }

        let content = fs::read_to_string(path)?;
        let store = serde_json::from_str(&content)?;
        Ok(store)
    }

    pub fn save_jobs(&self) -> AppResult<()> {
        let store = self.store.lock().unwrap();
        let path = Self::get_store_path(&self.app_data_dir);
        let content = serde_json::to_string_pretty(&*store)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn add_job(&self, job: Job) -> AppResult<()> {
        {
            let mut store = self.store.lock().unwrap();
            store.jobs.push(job);
        }
        self.save_jobs()
    }

    pub fn update_job_status(&self, id: &str, status: &str) -> AppResult<()> {
        {
            let mut store = self.store.lock().unwrap();
            if let Some(job) = store.jobs.iter_mut().find(|j| j.id == id) {
                job.status = status.to_string();
            }
        }
        self.save_jobs()
    }

    pub fn get_or_load_model(
        &self,
        model_path: &Path,
        prefer_npu: bool,
    ) -> AppResult<Arc<OrtSession>> {
        // 1. Memory Safety Check
        // We use a simple refresh to be safe and compatible
        let mut sys = System::new();
        sys.refresh_memory();
        let available_ram_gb = sys.available_memory() as f32 / 1024.0 / 1024.0 / 1024.0;

        // If less than 2GB available, SKIP CACHING entirely to prevent system instability.
        // We just load, use, and drop.
        if available_ram_gb < 2.0 {
            tracing::warn!(
                "Low Memory ({:.2} GB free). Skipping model cache for safety.",
                available_ram_gb
            );
            let session = OrtSession::new(model_path, prefer_npu)?;
            return Ok(Arc::new(session));
        }

        // 2. Check Cache
        {
            let cache = self.model_cache.lock().unwrap();
            if let Some((cached_path, session)) = &*cache {
                if cached_path == model_path {
                    tracing::info!("Cache Hit: {:?}", model_path);
                    return Ok(session.clone());
                }
            }
        }

        // 3. Load New Model (Heavy Operation)
        tracing::info!("Loading model from: {:?}", model_path);
        let session = OrtSession::new(model_path, prefer_npu)?;
        let session_arc = Arc::new(session);

        // 4. Update Cache (Single Slot)
        {
            let mut cache = self.model_cache.lock().unwrap();
            // This drops the previous model (if any), freeing its memory
            // unless it's currently in use by a running job (Arc count > 1).
            *cache = Some((model_path.to_path_buf(), session_arc.clone()));
        }

        Ok(session_arc)
    }

    pub fn get_model_scale(&self, model_path: &Path, session: &OrtSession) -> AppResult<u32> {
        // 1. Check Cache
        {
            let cache = self.scale_cache.lock().unwrap();
            if let Some(&scale) = cache.get(model_path) {
                if scale == 0 {
                    // Negative Cache Hit
                    return Err(crate::error::AppError::Unknown(
                        "Model previously failed scale detection (Negative Cache)".to_string(),
                    ));
                }
                return Ok(scale);
            }
        }

        // 2. Run Detection (Heavy)
        tracing::info!("Detecting scale for: {:?}", model_path);
        let result = session.detect_scale();

        // 3. Update Cache
        let mut cache = self.scale_cache.lock().unwrap();
        match result {
            Ok(scale) => {
                cache.insert(model_path.to_path_buf(), scale);
                Ok(scale)
            }
            Err(e) => {
                // Negative Caching: Remember failure to prevent retry loops
                tracing::warn!(
                    "Scale detection failed for {:?}. Caching failure.",
                    model_path
                );
                cache.insert(model_path.to_path_buf(), 0);
                Err(e)
            }
        }
    }

    pub fn invalidate_cache(&self, path: &Path) {
        let mut cache = self.scale_cache.lock().unwrap();
        if cache.remove(path).is_some() {
            tracing::info!("Invalidated scale cache for: {:?}", path);
        }
    }
}
