use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub name: String,
    pub vram_used_mb: u64,
    pub vram_total_mb: u64,
    pub vendor: GpuVendor,
    pub detection_method: String,
    pub is_npu: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GpuVendor {
    Nvidia,
    Amd,
    Intel,
    Apple,
    Unknown,
}

impl GpuVendor {
    fn from_name(name: &str) -> Self {
        let name_lower = name.to_lowercase();
        if name_lower.contains("nvidia") {
            GpuVendor::Nvidia
        } else if name_lower.contains("amd") || name_lower.contains("radeon") {
            GpuVendor::Amd
        } else if name_lower.contains("intel") {
            GpuVendor::Intel
        } else if name_lower.contains("apple")
            || name_lower.contains("m1")
            || name_lower.contains("m2")
            || name_lower.contains("m3")
        {
            GpuVendor::Apple
        } else {
            GpuVendor::Unknown
        }
    }
}

impl GpuInfo {
    /// Detect GPU information using platform-specific methods
    pub fn detect() -> AppResult<Self> {
        // Try platform-specific detection in order of reliability

        #[cfg(target_os = "windows")]
        {
            // Try NVIDIA first (most common)
            if let Ok(info) = Self::detect_nvidia() {
                return Ok(info);
            }

            // Try NPU (Intel AI Boost / AMD IPU / Qualcomm Hexagon)
            if let Ok(info) = Self::detect_npu() {
                return Ok(info);
            }

            // Try AMD
            if let Ok(info) = Self::detect_amd() {
                return Ok(info);
            }

            // Try Intel
            if let Ok(info) = Self::detect_intel() {
                return Ok(info);
            }
        }

        #[cfg(target_os = "macos")]
        {
            // Try Apple Silicon first
            if let Ok(info) = Self::detect_apple() {
                return Ok(info);
            }

            // Try AMD (for Intel Macs)
            if let Ok(info) = Self::detect_amd() {
                return Ok(info);
            }
        }

        #[cfg(target_os = "linux")]
        {
            // Try NVIDIA first
            if let Ok(info) = Self::detect_nvidia() {
                return Ok(info);
            }

            // Try AMD
            if let Ok(info) = Self::detect_amd() {
                return Ok(info);
            }

            // Try Intel
            if let Ok(info) = Self::detect_intel() {
                return Ok(info);
            }
        }

        // Fallback to system memory detection
        Self::fallback_system_memory()
    }

    /// NVIDIA GPU detection using NVML (Native) with CLI fallback
    fn detect_nvidia() -> AppResult<Self> {
        // 1. Try Native NVML
        #[cfg(any(target_os = "windows", target_os = "linux"))]
        {
            use nvml_wrapper::Nvml;
            if let Ok(nvml) = Nvml::init() {
                if let Ok(device) = nvml.device_by_index(0) {
                    if let Ok(memory) = device.memory_info() {
                        let name = device.name().unwrap_or_else(|_| "NVIDIA GPU".to_string());
                        return Ok(GpuInfo {
                            name,
                            vram_used_mb: memory.used / 1024 / 1024,
                            vram_total_mb: memory.total / 1024 / 1024,
                            vendor: GpuVendor::Nvidia,
                            detection_method: "NVML (Native)".to_string(),
                            is_npu: false,
                        });
                    }
                }
            }
        }

        // 2. Fallback to CLI
        Self::detect_nvidia_cli()
    }

    fn detect_nvidia_cli() -> AppResult<Self> {
        let mut cmd = Command::new("nvidia-smi");
        cmd.args(&[
            "--query-gpu=name,memory.used,memory.total",
            "--format=csv,noheader,nounits",
        ]);

        #[cfg(target_os = "windows")]
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

        let output = cmd
            .output()
            .map_err(|_| AppError::Unknown("nvidia-smi not found".to_string()))?;

        if !output.status.success() {
            return Err(AppError::Unknown("nvidia-smi failed".to_string()));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = output_str.trim().split(',').collect();

        if parts.len() < 3 {
            return Err(AppError::Unknown("Invalid nvidia-smi output".to_string()));
        }

        Ok(GpuInfo {
            name: parts[0].trim().to_string(),
            vram_used_mb: parts[1].trim().parse().unwrap_or(0),
            vram_total_mb: parts[2].trim().parse().unwrap_or(0),
            vendor: GpuVendor::Nvidia,
            detection_method: "nvidia-smi (CLI)".to_string(),
            is_npu: false,
        })
    }

    /// AMD GPU detection
    #[cfg(target_os = "windows")]
    fn detect_amd() -> AppResult<Self> {
        // 1. Try Native WMI
        if let Ok(info) = Self::detect_wmi("AMD", "Radeon") {
            return Ok(info);
        }

        // 2. Fallback to CLI
        Self::detect_amd_cli()
    }

    #[cfg(target_os = "windows")]
    fn detect_amd_cli() -> AppResult<Self> {
        let script = r#"
            $gpu = Get-WmiObject Win32_VideoController | Where-Object { $_.Name -like '*AMD*' -or $_.Name -like '*Radeon*' } | Select-Object -First 1
            if ($gpu) {
                $name = $gpu.Name
                $vram = [math]::Round($gpu.AdapterRAM / 1MB)
                Write-Output "$name,$vram"
            }
        "#;

        let mut cmd = Command::new("powershell");
        cmd.args(&["-NoProfile", "-Command", script]);
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

        let output = cmd
            .output()
            .map_err(|_| AppError::Unknown("PowerShell failed".to_string()))?;

        if !output.status.success() {
            return Err(AppError::Unknown("AMD detection failed".to_string()));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = output_str.trim().split(',').collect();

        if parts.len() < 2 {
            return Err(AppError::Unknown("Invalid AMD output".to_string()));
        }

        let total_mb: u64 = parts[1].trim().parse().unwrap_or(0);

        Ok(GpuInfo {
            name: parts[0].trim().to_string(),
            vram_used_mb: 0,
            vram_total_mb: total_mb,
            vendor: GpuVendor::Amd,
            detection_method: "WMI (CLI)".to_string(),
            is_npu: false,
        })
    }

    #[cfg(not(target_os = "windows"))]
    fn detect_amd() -> AppResult<Self> {
        // Try rocm-smi on Linux
        let output = Command::new("rocm-smi")
            .args(&["--showmeminfo", "vram"])
            .output()
            .map_err(|_| AppError::Unknown("rocm-smi not found".to_string()))?;

        if !output.status.success() {
            return Err(AppError::Unknown("rocm-smi failed".to_string()));
        }

        // Parse rocm-smi output (format varies)
        // This is a simplified parser
        let output_str = String::from_utf8_lossy(&output.stdout);

        Ok(GpuInfo {
            name: "AMD GPU".to_string(),
            vram_used_mb: 0,
            vram_total_mb: 8192, // Placeholder
            vendor: GpuVendor::Amd,
            detection_method: "rocm-smi".to_string(),
            is_npu: false,
        })
    }

    /// Intel GPU detection
    #[cfg(target_os = "windows")]
    fn detect_intel() -> AppResult<Self> {
        // 1. Try Native WMI
        if let Ok(info) = Self::detect_wmi("Intel", "Intel") {
            return Ok(info);
        }

        // 2. Fallback to CLI
        Self::detect_intel_cli()
    }

    #[cfg(target_os = "windows")]
    fn detect_intel_cli() -> AppResult<Self> {
        let script = r#"
            $gpu = Get-WmiObject Win32_VideoController | Where-Object { $_.Name -like '*Intel*' } | Select-Object -First 1
            if ($gpu) {
                $name = $gpu.Name
                $vram = [math]::Round($gpu.AdapterRAM / 1MB)
                Write-Output "$name,$vram"
            }
        "#;

        let mut cmd = Command::new("powershell");
        cmd.args(&["-NoProfile", "-Command", script]);
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

        let output = cmd
            .output()
            .map_err(|_| AppError::Unknown("PowerShell failed".to_string()))?;

        if !output.status.success() {
            return Err(AppError::Unknown("Intel detection failed".to_string()));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = output_str.trim().split(',').collect();

        if parts.len() < 2 {
            return Err(AppError::Unknown("Invalid Intel output".to_string()));
        }

        let total_mb: u64 = parts[1].trim().parse().unwrap_or(0);

        Ok(GpuInfo {
            name: parts[0].trim().to_string(),
            vram_used_mb: 0,
            vram_total_mb: total_mb,
            vendor: GpuVendor::Intel,
            detection_method: "WMI (CLI)".to_string(),
            is_npu: false,
        })
    }

    #[cfg(not(target_os = "windows"))]
    fn detect_intel() -> AppResult<Self> {
        Err(AppError::Unknown(
            "Intel GPU detection not implemented for this platform".to_string(),
        ))
    }

    /// Apple Silicon GPU detection
    #[cfg(target_os = "macos")]
    fn detect_apple() -> AppResult<Self> {
        // Use system_profiler to get GPU info
        let output = Command::new("system_profiler")
            .args(&["SPDisplaysDataType", "-json"])
            .output()
            .map_err(|_| AppError::Unknown("system_profiler failed".to_string()))?;

        if !output.status.success() {
            return Err(AppError::Unknown("system_profiler failed".to_string()));
        }

        // For Apple Silicon, unified memory is shared
        // We can get total memory and estimate GPU portion
        let output_str = String::from_utf8_lossy(&output.stdout);

        // Parse JSON (simplified - would use serde_json in production)
        Ok(GpuInfo {
            name: "Apple GPU".to_string(),
            vram_used_mb: 0,
            vram_total_mb: 16384, // Placeholder - would parse actual value
            vendor: GpuVendor::Apple,
            detection_method: "system_profiler".to_string(),
            is_npu: false,
        })
    }

    /// Fallback to system memory detection
    fn fallback_system_memory() -> AppResult<Self> {
        use sysinfo::System;

        let mut sys = System::new_all();
        sys.refresh_all();

        Ok(GpuInfo {
            name: "System Memory (GPU detection unavailable)".to_string(),
            vram_used_mb: sys.used_memory() / 1024 / 1024,
            vram_total_mb: sys.total_memory() / 1024 / 1024,
            vendor: GpuVendor::Unknown,
            detection_method: "sysinfo_fallback".to_string(),
            is_npu: false,
        })
    }

    /// NPU detection (Windows)
    #[cfg(target_os = "windows")]
    fn detect_npu() -> AppResult<Self> {
        // 1. Try Native WMI (PnPEntity)
        // Note: WMI query for PnPEntity can be slow, but native is faster than PowerShell
        // For now, we'll stick to the CLI fallback for NPU as it's less critical,
        // but we MUST use CREATE_NO_WINDOW.

        // Actually, let's try to implement native WMI for NPU too if possible,
        // but Win32_PnPEntity is huge. Let's stick to CLI for NPU for now to avoid complexity,
        // but fix the window.
        Self::detect_npu_cli()
    }

    #[cfg(target_os = "windows")]
    fn detect_npu_cli() -> AppResult<Self> {
        // Look for common NPU device names in PNP Entities
        // Intel: "Intel(R) AI Boost"
        // AMD: "AMD IPU Device"
        // Qualcomm: "Qualcomm(R) Hexagon(TM) Processor"
        let script = r#"
            $npu = Get-WmiObject Win32_PnPEntity | Where-Object { $_.Name -match 'Intel\(R\) AI Boost|AMD IPU|Hexagon' } | Select-Object -First 1
            if ($npu) {
                Write-Output $npu.Name
            }
        "#;

        let mut cmd = Command::new("powershell");
        cmd.args(&["-NoProfile", "-Command", script]);
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

        let output = cmd
            .output()
            .map_err(|_| AppError::Unknown("PowerShell failed".to_string()))?;

        if !output.status.success() {
            return Err(AppError::Unknown("NPU detection failed".to_string()));
        }

        let output_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if output_str.is_empty() {
            return Err(AppError::Unknown("No NPU found".to_string()));
        }

        // NPU usually shares system memory, so we report system RAM as "VRAM"
        // This is similar to how iGPUs work
        use sysinfo::System;
        let mut sys = System::new_all();
        sys.refresh_memory();

        Ok(GpuInfo {
            name: output_str.clone(),
            vram_used_mb: sys.used_memory() / 1024 / 1024,
            vram_total_mb: sys.total_memory() / 1024 / 1024,
            vendor: GpuVendor::from_name(&output_str),
            detection_method: "WMI (NPU CLI)".to_string(),
            is_npu: true,
        })
    }

    #[cfg(not(target_os = "windows"))]
    fn detect_npu() -> AppResult<Self> {
        Err(AppError::Unknown(
            "NPU detection not supported on this OS".to_string(),
        ))
    }

    /// Helper for Native WMI Detection (Windows)
    #[cfg(target_os = "windows")]
    fn detect_wmi(filter1: &str, filter2: &str) -> AppResult<Self> {
        use serde::Deserialize;

        use wmi::{COMLibrary, WMIConnection};

        #[derive(Deserialize, Debug)]
        #[allow(non_snake_case)]
        struct VideoController {
            Name: String,
            AdapterRAM: Option<u64>,
        }

        let com_con =
            COMLibrary::new().map_err(|_| AppError::Unknown("COM Init failed".to_string()))?;
        let wmi_con = WMIConnection::new(com_con)
            .map_err(|_| AppError::Unknown("WMI Init failed".to_string()))?;

        let results: Vec<VideoController> = wmi_con
            .raw_query("SELECT Name, AdapterRAM FROM Win32_VideoController")
            .map_err(|_| AppError::Unknown("WMI Query failed".to_string()))?;

        for gpu in results {
            if gpu.Name.contains(filter1) || gpu.Name.contains(filter2) {
                let vram = gpu.AdapterRAM.unwrap_or(0) / 1024 / 1024;
                let vendor = if gpu.Name.to_lowercase().contains("amd")
                    || gpu.Name.to_lowercase().contains("radeon")
                {
                    GpuVendor::Amd
                } else {
                    GpuVendor::Intel
                };

                return Ok(GpuInfo {
                    name: gpu.Name,
                    vram_used_mb: 0,
                    vram_total_mb: vram,
                    vendor,
                    detection_method: "WMI (Native)".to_string(),
                    is_npu: false,
                });
            }
        }

        Err(AppError::Unknown("GPU not found via WMI".to_string()))
    }

    /// Check if we have enough VRAM for a given operation
    pub fn has_sufficient_vram(&self, required_mb: u64) -> bool {
        let available_mb = self.vram_total_mb.saturating_sub(self.vram_used_mb);
        available_mb >= required_mb
    }

    /// Get recommended tile size based on available VRAM
    pub fn recommended_tile_size(&self) -> u32 {
        // Use Total VRAM for the "Tier" of the card.
        // This prevents the tile size from dropping just because the model is loaded.
        let total_mb = self.vram_total_mb;
        let available_mb = self.vram_total_mb.saturating_sub(self.vram_used_mb);

        // Safety Fallback: If we are critically low on memory (< 512MB), force small tiles.
        if available_mb < 512 {
            return 256;
        }

        match total_mb {
            0..=2048 => 192,      // < 2GB (Integrated/Old): Very small tiles (f32 safe)
            2049..=4096 => 256,   // 2-4GB (Entry): Small tiles
            4097..=6144 => 512,   // 4-6GB (Mid-Range): Medium tiles (Reduced from 768 for f32)
            6145..=8192 => 768,   // 6-8GB (High-End): Large tiles (Reduced from 1024 for f32)
            8193..=16384 => 1024, // 8-16GB (Enthusiast): Very large tiles
            _ => 1536,            // > 16GB (Pro): Massive tiles
        }
    }

    /// Get recommended batch size based on available VRAM
    pub fn recommended_batch_size(&self) -> usize {
        // Use Available VRAM for adaptive optimization.
        // If we can't detect usage (e.g. AMD on Windows), we assume 50% usage for safety.
        let used_mb = if self.vram_used_mb == 0 && self.vendor != GpuVendor::Unknown {
            self.vram_total_mb / 2
        } else {
            self.vram_used_mb
        };

        let available_mb = self.vram_total_mb.saturating_sub(used_mb);

        // Adaptive Logic:
        // > 12GB Free: Batch 4 (High Throughput)
        // 6-12GB Free: Batch 2 (Balanced)
        // < 6GB Free: Batch 1 (Safety)
        // Note: We are more conservative with "Free" memory than "Total" memory.
        match available_mb {
            0..=6144 => 1,     // < 6GB Free: Single tile to prevent OOM
            6145..=12288 => 2, // 6-12GB Free: Batch 2
            _ => 4,            // > 12GB Free: Batch 4
        }
    }
}
