use crate::error::{Result, SecureEraseError};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::SystemTime;

pub mod advanced;
pub mod device_manager;

/// Represents a storage device that can be securely erased
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDevice {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub device_type: DeviceType,
    pub model: Option<String>,
    pub serial: Option<String>,
    pub supports_secure_erase: bool,
    pub supports_trim: bool,
    pub hidden_areas: Vec<HiddenArea>,
}

/// Types of storage devices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    HDD,
    SSD,
    NVMe,
    USB,
    Unknown,
}

/// Hidden storage areas that need special handling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HiddenArea {
    pub area_type: HiddenAreaType,
    pub start_lba: u64,
    pub size: u64,
    pub description: String,
}

/// Types of hidden areas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HiddenAreaType {
    HPA,    // Host Protected Area
    DCO,    // Device Configuration Overlay
    SSDReserved,
    VendorSpecific,
}

/// Erase modes supported by the tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EraseMode {
    Quick,      // Single pass with zeros
    Full,       // Multiple passes with random data
    Advanced,   // Hardware secure erase + verification
}

/// Wipe operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WipeResult {
    pub device: StorageDevice,
    pub mode: EraseMode,
    pub start_time: SystemTime,
    pub end_time: SystemTime,
    pub duration_seconds: u64,
    pub bytes_written: u64,
    pub verification_passed: bool,
    pub errors: Vec<String>,
}

/// Core wipe engine trait
pub trait WipeEngine {
    /// List all available storage devices
    async fn list_devices(&self) -> Result<Vec<StorageDevice>>;
    
    /// Securely erase a device
    async fn wipe_device(&self, device: &StorageDevice, mode: EraseMode) -> Result<WipeResult>;
    
    /// Verify that a device has been properly erased
    async fn verify_wipe(&self, device: &StorageDevice) -> Result<bool>;
    
    /// Check if device supports hardware secure erase
    async fn supports_secure_erase(&self, device: &StorageDevice) -> Result<bool>;
}

/// Platform-specific wipe engine implementation
pub struct PlatformWipeEngine;

impl WipeEngine for PlatformWipeEngine {
    async fn list_devices(&self) -> Result<Vec<StorageDevice>> {
        #[cfg(target_os = "windows")]
        return platform::windows::list_devices().await;
        
        #[cfg(target_os = "linux")]
        return platform::linux::list_devices().await;
        
        #[cfg(target_os = "android")]
        return platform::android::list_devices().await;
        
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "android")))]
        Err(SecureEraseError::UnsupportedPlatform)
    }
    
    async fn wipe_device(&self, device: &StorageDevice, mode: EraseMode) -> Result<WipeResult> {
        #[cfg(target_os = "windows")]
        return platform::windows::wipe_device(device, mode).await;
        
        #[cfg(target_os = "linux")]
        return platform::linux::wipe_device(device, mode).await;
        
        #[cfg(target_os = "android")]
        return platform::android::wipe_device(device, mode).await;
        
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "android")))]
        Err(SecureEraseError::UnsupportedPlatform)
    }
    
    async fn verify_wipe(&self, device: &StorageDevice) -> Result<bool> {
        #[cfg(target_os = "windows")]
        return platform::windows::verify_wipe(device).await;
        
        #[cfg(target_os = "linux")]
        return platform::linux::verify_wipe(device).await;
        
        #[cfg(target_os = "android")]
        return platform::android::verify_wipe(device).await;
        
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "android")))]
        Err(SecureEraseError::UnsupportedPlatform)
    }
    
    async fn supports_secure_erase(&self, device: &StorageDevice) -> Result<bool> {
        #[cfg(target_os = "windows")]
        return platform::windows::supports_secure_erase(device).await;
        
        #[cfg(target_os = "linux")]
        return platform::linux::supports_secure_erase(device).await;
        
        #[cfg(target_os = "android")]
        return platform::android::supports_secure_erase(device).await;
        
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "android")))]
        Err(SecureEraseError::UnsupportedPlatform)
    }
}
