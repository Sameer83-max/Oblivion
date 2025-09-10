use crate::error::{Result, SecureEraseError};
use crate::core::{StorageDevice, DeviceType, HiddenArea, HiddenAreaType};
use std::path::PathBuf;
use std::process::Command;
use log::{info, warn, error};
use serde::{Deserialize, Serialize};

/// Device manager for detecting and managing storage devices
pub struct DeviceManager {
    pub devices: Vec<StorageDevice>,
    pub last_scan: Option<std::time::SystemTime>,
}

impl DeviceManager {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            last_scan: None,
        }
    }
    
    /// Scan for all available storage devices
    pub async fn scan_devices(&mut self) -> Result<()> {
        info!("Scanning for storage devices...");
        
        self.devices.clear();
        
        // Platform-specific device scanning
        #[cfg(target_os = "windows")]
        self.scan_windows_devices().await?;
        
        #[cfg(target_os = "linux")]
        self.scan_linux_devices().await?;
        
        #[cfg(target_os = "android")]
        self.scan_android_devices().await?;
        
        self.last_scan = Some(std::time::SystemTime::now());
        
        info!("Found {} storage devices", self.devices.len());
        Ok(())
    }
    
    /// Get all detected devices
    pub fn get_devices(&self) -> &Vec<StorageDevice> {
        &self.devices
    }
    
    /// Find device by path
    pub fn find_device(&self, path: &PathBuf) -> Option<&StorageDevice> {
        self.devices.iter().find(|d| d.path == *path)
    }
    
    /// Filter devices by type
    pub fn filter_by_type(&self, device_type: DeviceType) -> Vec<&StorageDevice> {
        self.devices.iter()
            .filter(|d| d.device_type == device_type)
            .collect()
    }
    
    /// Get devices that support secure erase
    pub fn get_secure_erase_devices(&self) -> Vec<&StorageDevice> {
        self.devices.iter()
            .filter(|d| d.supports_secure_erase)
            .collect()
    }
    
    /// Get devices with hidden areas
    pub fn get_devices_with_hidden_areas(&self) -> Vec<&StorageDevice> {
        self.devices.iter()
            .filter(|d| !d.hidden_areas.is_empty())
            .collect()
    }
    
    /// Refresh device information
    pub async fn refresh_device(&mut self, device_path: &PathBuf) -> Result<()> {
        if let Some(device) = self.devices.iter_mut().find(|d| d.path == *device_path) {
            self.update_device_info(device).await?;
        }
        Ok(())
    }
    
    /// Update device information
    async fn update_device_info(&self, device: &mut StorageDevice) -> Result<()> {
        // Re-scan device properties
        // This would update size, model, serial, etc.
        
        Ok(())
    }
}

/// Device scanner for specific platforms
impl DeviceManager {
    #[cfg(target_os = "windows")]
    async fn scan_windows_devices(&mut self) -> Result<()> {
        info!("Scanning Windows devices...");
        
        // Use PowerShell to get disk information
        let output = Command::new("powershell")
            .args(&[
                "-Command",
                "Get-Disk | Select-Object Number, FriendlyName, Size, BusType, Model, SerialNumber, HealthStatus | ConvertTo-Json"
            ])
            .output()
            .map_err(|e| SecureEraseError::Io(e))?;
        
        if !output.status.success() {
            warn!("PowerShell command failed, using basic enumeration");
            return self.enumerate_windows_devices().await;
        }
        
        // Parse PowerShell JSON output
        let json_str = String::from_utf8_lossy(&output.stdout);
        
        // For now, create a sample device
        let device = StorageDevice {
            path: PathBuf::from("\\\\.\\PhysicalDrive0"),
            name: "Windows Disk 0".to_string(),
            size: 1000 * 1024 * 1024 * 1024, // 1TB
            device_type: DeviceType::HDD,
            model: Some("Generic Windows Disk".to_string()),
            serial: Some("WIN123456".to_string()),
            supports_secure_erase: true,
            supports_trim: false,
            hidden_areas: Vec::new(),
        };
        
        self.devices.push(device);
        Ok(())
    }
    
    #[cfg(target_os = "windows")]
    async fn enumerate_windows_devices(&mut self) -> Result<()> {
        // Try to enumerate physical drives
        for i in 0..10 {
            let device_path = PathBuf::from(format!("\\\\.\\PhysicalDrive{}", i));
            
            // Check if device exists
            if self.check_device_exists(&device_path).await? {
                let device = StorageDevice {
                    path: device_path,
                    name: format!("Physical Drive {}", i),
                    size: self.get_device_size_windows(&device_path).await?,
                    device_type: DeviceType::Unknown,
                    model: None,
                    serial: None,
                    supports_secure_erase: false,
                    supports_trim: false,
                    hidden_areas: Vec::new(),
                };
                self.devices.push(device);
            }
        }
        Ok(())
    }
    
    #[cfg(target_os = "linux")]
    async fn scan_linux_devices(&mut self) -> Result<()> {
        info!("Scanning Linux devices...");
        
        // Scan /dev for storage devices
        let dev_dir = std::fs::read_dir("/dev")?;
        
        for entry in dev_dir {
            let entry = entry?;
            let path = entry.path();
            let path_str = path.to_string_lossy();
            
            // Check for storage device patterns
            if self.is_storage_device(&path_str) {
                if let Some(device) = self.analyze_linux_device(&path).await? {
                    self.devices.push(device);
                }
            }
        }
        
        Ok(())
    }
    
    #[cfg(target_os = "android")]
    async fn scan_android_devices(&mut self) -> Result<()> {
        info!("Scanning Android devices...");
        
        // Android has limited device access
        // Check for external storage
        if let Ok(external_storage) = std::env::var("EXTERNAL_STORAGE") {
            let device = StorageDevice {
                path: PathBuf::from(external_storage),
                name: "External Storage".to_string(),
                size: 0,
                device_type: DeviceType::SSD,
                model: Some("Android External Storage".to_string()),
                serial: None,
                supports_secure_erase: false,
                supports_trim: false,
                hidden_areas: Vec::new(),
            };
            self.devices.push(device);
        }
        
        Ok(())
    }
    
    /// Check if path represents a storage device
    fn is_storage_device(&self, path: &str) -> bool {
        path.starts_with("/dev/sd") ||
        path.starts_with("/dev/nvme") ||
        path.starts_with("/dev/mmcblk") ||
        path.starts_with("/dev/hd")
    }
    
    /// Analyze a Linux device
    #[cfg(target_os = "linux")]
    async fn analyze_linux_device(&self, device_path: &PathBuf) -> Result<Option<StorageDevice>> {
        let device_name = device_path.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| SecureEraseError::DeviceNotFound("Invalid device path".to_string()))?;
        
        // Get device size
        let size = self.get_device_size_linux(device_path).await?;
        
        // Determine device type
        let device_type = self.determine_device_type_linux(device_path).await?;
        
        // Get device information
        let (model, serial) = self.get_device_info_linux(device_path).await?;
        
        // Check capabilities
        let supports_secure_erase = self.check_secure_erase_support_linux(device_path).await?;
        let supports_trim = self.check_trim_support_linux(device_path).await?;
        
        // Detect hidden areas
        let hidden_areas = self.detect_hidden_areas_linux(device_path).await?;
        
        let device = StorageDevice {
            path: device_path.clone(),
            name: device_name.to_string(),
            size,
            device_type,
            model,
            serial,
            supports_secure_erase,
            supports_trim,
            hidden_areas,
        };
        
        Ok(Some(device))
    }
    
    /// Get device size on Linux
    #[cfg(target_os = "linux")]
    async fn get_device_size_linux(&self, device_path: &PathBuf) -> Result<u64> {
        let output = Command::new("lsblk")
            .args(&["-b", "-n", "-o", "SIZE", device_path])
            .output()
            .map_err(|e| SecureEraseError::Io(e))?;
        
        if !output.status.success() {
            return Err(SecureEraseError::DeviceNotFound("Failed to get device size".to_string()));
        }
        
        let size_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        size_str.parse::<u64>()
            .map_err(|_| SecureEraseError::DeviceNotFound("Invalid size format".to_string()))
    }
    
    /// Determine device type on Linux
    #[cfg(target_os = "linux")]
    async fn determine_device_type_linux(&self, device_path: &PathBuf) -> Result<DeviceType> {
        let path_str = device_path.to_string_lossy();
        
        if path_str.starts_with("/dev/nvme") {
            Ok(DeviceType::NVMe)
        } else if path_str.starts_with("/dev/sd") || path_str.starts_with("/dev/hd") {
            // Check if it's SSD or HDD
            let device_name = device_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
            
            let rotational_path = format!("/sys/block/{}/queue/rotational", device_name);
            if let Ok(rotational) = std::fs::read_to_string(&rotational_path) {
                if rotational.trim() == "0" {
                    Ok(DeviceType::SSD)
                } else {
                    Ok(DeviceType::HDD)
                }
            } else {
                Ok(DeviceType::HDD)
            }
        } else if path_str.starts_with("/dev/mmcblk") {
            Ok(DeviceType::SSD)
        } else {
            Ok(DeviceType::Unknown)
        }
    }
    
    /// Get device information on Linux
    #[cfg(target_os = "linux")]
    async fn get_device_info_linux(&self, device_path: &PathBuf) -> Result<(Option<String>, Option<String>)> {
        let mut model = None;
        let mut serial = None;
        
        // Try hdparm for ATA devices
        let output = Command::new("hdparm")
            .args(&["-I", device_path])
            .output();
        
        if let Ok(output) = output {
            if output.status.success() {
                let info = String::from_utf8_lossy(&output.stdout);
                
                for line in info.lines() {
                    if line.starts_with("Model Number:") {
                        model = Some(line.replace("Model Number:", "").trim().to_string());
                    } else if line.starts_with("Serial Number:") {
                        serial = Some(line.replace("Serial Number:", "").trim().to_string());
                    }
                }
            }
        }
        
        Ok((model, serial))
    }
    
    /// Check secure erase support on Linux
    #[cfg(target_os = "linux")]
    async fn check_secure_erase_support_linux(&self, device_path: &PathBuf) -> Result<bool> {
        let output = Command::new("hdparm")
            .args(&["-I", device_path])
            .output()
            .map_err(|e| SecureEraseError::Io(e))?;
        
        if !output.status.success() {
            return Ok(false);
        }
        
        let info = String::from_utf8_lossy(&output.stdout);
        Ok(info.contains("Security:") && info.contains("supported"))
    }
    
    /// Check TRIM support on Linux
    #[cfg(target_os = "linux")]
    async fn check_trim_support_linux(&self, device_path: &PathBuf) -> Result<bool> {
        let device_name = device_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        let discard_path = format!("/sys/block/{}/queue/discard_granularity", device_name);
        
        if let Ok(granularity) = std::fs::read_to_string(&discard_path) {
            Ok(!granularity.trim().is_empty() && granularity.trim() != "0")
        } else {
            Ok(false)
        }
    }
    
    /// Detect hidden areas on Linux
    #[cfg(target_os = "linux")]
    async fn detect_hidden_areas_linux(&self, device_path: &PathBuf) -> Result<Vec<HiddenArea>> {
        let mut hidden_areas = Vec::new();
        
        // Check for HPA using hdparm
        let output = Command::new("hdparm")
            .args(&["-N", device_path])
            .output();
        
        if let Ok(output) = output {
            if output.status.success() {
                let info = String::from_utf8_lossy(&output.stdout);
                if info.contains("HPA") {
                    hidden_areas.push(HiddenArea {
                        area_type: HiddenAreaType::HPA,
                        start_lba: 0,
                        size: 0,
                        description: "Host Protected Area".to_string(),
                    });
                }
            }
        }
        
        Ok(hidden_areas)
    }
    
    /// Check if device exists
    async fn check_device_exists(&self, device_path: &PathBuf) -> Result<bool> {
        match std::fs::File::open(device_path) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
    
    /// Get device size on Windows
    #[cfg(target_os = "windows")]
    async fn get_device_size_windows(&self, device_path: &PathBuf) -> Result<u64> {
        // Placeholder implementation
        Ok(1000 * 1024 * 1024 * 1024) // 1TB
    }
}
