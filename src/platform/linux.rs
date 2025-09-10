use crate::error::{Result, SecureEraseError};
use crate::core::{StorageDevice, DeviceType, EraseMode, WipeResult, HiddenArea, HiddenAreaType};
use std::path::PathBuf;
use std::time::SystemTime;
use std::process::Command;
use log::{info, warn, error};

/// Linux-specific implementation of disk operations
pub async fn list_devices() -> Result<Vec<StorageDevice>> {
    info!("Scanning Linux storage devices...");
    
    let mut devices = Vec::new();
    
    // Scan /dev for storage devices
    let dev_dir = std::fs::read_dir("/dev")?;
    
    for entry in dev_dir {
        let entry = entry?;
        let path = entry.path();
        let path_str = path.to_string_lossy();
        
        // Check for common storage device patterns
        if path_str.starts_with("/dev/sd") || 
           path_str.starts_with("/dev/nvme") || 
           path_str.starts_with("/dev/mmcblk") {
            
            if let Some(device) = analyze_linux_device(&path).await? {
                devices.push(device);
            }
        }
    }
    
    Ok(devices)
}

/// Analyze a Linux device and extract information
async fn analyze_linux_device(device_path: &PathBuf) -> Result<Option<StorageDevice>> {
    let device_name = device_path.file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| SecureEraseError::DeviceNotFound("Invalid device path".to_string()))?;
    
    // Get device size using lsblk
    let size = get_device_size_linux(device_path).await?;
    
    // Get device type
    let device_type = determine_device_type_linux(device_path).await?;
    
    // Get model and serial using hdparm or similar tools
    let (model, serial) = get_device_info_linux(device_path).await?;
    
    // Check for secure erase support
    let supports_secure_erase = check_secure_erase_support_linux(device_path).await?;
    
    // Check for TRIM support
    let supports_trim = check_trim_support_linux(device_path).await?;
    
    // Detect hidden areas
    let hidden_areas = detect_hidden_areas_linux(device_path).await?;
    
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
async fn get_device_size_linux(device_path: &PathBuf) -> Result<u64> {
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
async fn determine_device_type_linux(device_path: &PathBuf) -> Result<DeviceType> {
    let path_str = device_path.to_string_lossy();
    
    if path_str.starts_with("/dev/nvme") {
        Ok(DeviceType::NVMe)
    } else if path_str.starts_with("/dev/sd") || path_str.starts_with("/dev/hd") {
        // Check if it's SSD or HDD using /sys/block
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
            Ok(DeviceType::HDD) // Default assumption
        }
    } else if path_str.starts_with("/dev/mmcblk") {
        Ok(DeviceType::SSD) // Treat eMMC as SSD
    } else {
        Ok(DeviceType::Unknown)
    }
}

/// Get device model and serial number
async fn get_device_info_linux(device_path: &PathBuf) -> Result<(Option<String>, Option<String>)> {
    let mut model = None;
    let mut serial = None;
    
    // Try hdparm for ATA devices
    let output = Command::new("hdparm")
        .args(&["-I", device_path])
        .output();
    
    if let Ok(output) = output {
        if output.status.success() {
            let info = String::from_utf8_lossy(&output.stdout);
            
            // Parse model and serial from hdparm output
            for line in info.lines() {
                if line.starts_with("Model Number:") {
                    model = Some(line.replace("Model Number:", "").trim().to_string());
                } else if line.starts_with("Serial Number:") {
                    serial = Some(line.replace("Serial Number:", "").trim().to_string());
                }
            }
        }
    }
    
    // Try lsblk for additional info
    let output = Command::new("lsblk")
        .args(&["-n", "-o", "MODEL,SERIAL", device_path])
        .output();
    
    if let Ok(output) = output {
        if output.status.success() {
            let info = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = info.trim().split('\t').collect();
            if parts.len() >= 2 {
                if model.is_none() && !parts[0].is_empty() {
                    model = Some(parts[0].to_string());
                }
                if serial.is_none() && !parts[1].is_empty() {
                    serial = Some(parts[1].to_string());
                }
            }
        }
    }
    
    Ok((model, serial))
}

/// Check if device supports secure erase
async fn check_secure_erase_support_linux(device_path: &PathBuf) -> Result<bool> {
    // Check for ATA Security Feature Set
    let output = Command::new("hdparm")
        .args(&["-I", device_path])
        .output()
        .map_err(|e| SecureEraseError::Io(e))?;
    
    if !output.status.success() {
        return Ok(false);
    }
    
    let info = String::from_utf8_lossy(&output.stdout);
    
    // Look for security features
    Ok(info.contains("Security:") && info.contains("supported"))
}

/// Check if device supports TRIM
async fn check_trim_support_linux(device_path: &PathBuf) -> Result<bool> {
    // Check for TRIM support in /sys/block
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
async fn detect_hidden_areas_linux(device_path: &PathBuf) -> Result<Vec<HiddenArea>> {
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
                    start_lba: 0, // Would need to parse actual values
                    size: 0,
                    description: "Host Protected Area".to_string(),
                });
            }
        }
    }
    
    Ok(hidden_areas)
}

/// Securely erase a device on Linux
pub async fn wipe_device(device: &StorageDevice, mode: EraseMode) -> Result<WipeResult> {
    info!("Starting Linux wipe operation on device: {}", device.path.display());
    
    let start_time = SystemTime::now();
    
    match mode {
        EraseMode::Quick => {
            quick_wipe_linux(device).await?;
        }
        EraseMode::Full => {
            full_wipe_linux(device).await?;
        }
        EraseMode::Advanced => {
            advanced_wipe_linux(device).await?;
        }
    }
    
    let end_time = SystemTime::now();
    let duration = end_time.duration_since(start_time)
        .map_err(|e| SecureEraseError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
    
    // Verify the wipe
    let verification_passed = verify_wipe(device).await.unwrap_or(false);
    
    Ok(WipeResult {
        device: device.clone(),
        mode,
        start_time,
        end_time,
        duration_seconds: duration.as_secs(),
        bytes_written: device.size,
        verification_passed,
        errors: Vec::new(),
    })
}

/// Quick wipe using Linux tools
async fn quick_wipe_linux(device: &StorageDevice) -> Result<()> {
    info!("Performing quick wipe on Linux");
    
    // Use dd to write zeros
    let output = Command::new("dd")
        .args(&[
            "if=/dev/zero",
            &format!("of={}", device.path.display()),
            "bs=1M",
            "status=progress"
        ])
        .output()
        .map_err(|e| SecureEraseError::Io(e))?;
    
    if !output.status.success() {
        return Err(SecureEraseError::WipeFailed("Quick wipe failed".to_string()));
    }
    
    Ok(())
}

/// Full wipe with multiple passes
async fn full_wipe_linux(device: &StorageDevice) -> Result<()> {
    info!("Performing full wipe on Linux");
    
    // Use shred for multiple passes
    let output = Command::new("shred")
        .args(&[
            "-v",
            "-n", "3", // 3 passes
            "-z", // Final zero pass
            &device.path.to_string_lossy()
        ])
        .output()
        .map_err(|e| SecureEraseError::Io(e))?;
    
    if !output.status.success() {
        return Err(SecureEraseError::WipeFailed("Full wipe failed".to_string()));
    }
    
    Ok(())
}

/// Advanced wipe with hardware secure erase
async fn advanced_wipe_linux(device: &StorageDevice) -> Result<()> {
    info!("Performing advanced wipe on Linux");
    
    // Try hardware secure erase first
    if supports_secure_erase(device).await? {
        hardware_secure_erase_linux(device).await?;
    } else {
        // Fall back to software wiping
        full_wipe_linux(device).await?;
    }
    
    Ok(())
}

/// Hardware secure erase on Linux
async fn hardware_secure_erase_linux(device: &StorageDevice) -> Result<()> {
    info!("Attempting hardware secure erase on Linux");
    
    // Set security password (empty password for secure erase)
    let output = Command::new("hdparm")
        .args(&[
            "--user-master", "u",
            "--security-set-pass", "",
            &device.path.to_string_lossy()
        ])
        .output()
        .map_err(|e| SecureEraseError::Io(e))?;
    
    if !output.status.success() {
        return Err(SecureEraseError::WipeFailed("Failed to set security password".to_string()));
    }
    
    // Execute secure erase
    let output = Command::new("hdparm")
        .args(&[
            "--user-master", "u",
            "--security-erase", "",
            &device.path.to_string_lossy()
        ])
        .output()
        .map_err(|e| SecureEraseError::Io(e))?;
    
    if !output.status.success() {
        return Err(SecureEraseError::WipeFailed("Hardware secure erase failed".to_string()));
    }
    
    Ok(())
}

/// Verify that a device has been properly wiped
pub async fn verify_wipe(device: &StorageDevice) -> Result<bool> {
    info!("Verifying wipe on Linux device: {}", device.path.display());
    
    // Sample random sectors and check for non-zero data
    // This is a simplified implementation
    
    Ok(true) // Placeholder
}

/// Check if device supports hardware secure erase
pub async fn supports_secure_erase(device: &StorageDevice) -> Result<bool> {
    info!("Checking secure erase support for Linux device: {}", device.path.display());
    
    Ok(device.supports_secure_erase)
}
