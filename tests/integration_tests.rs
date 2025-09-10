use secure_disk_erasure::core::{StorageDevice, DeviceType, EraseMode, device_manager::DeviceManager, advanced::AdvancedWipeEngine};
use secure_disk_erasure::error::Result;
use std::path::PathBuf;
use tempfile::TempDir;

#[tokio::test]
async fn test_device_manager_creation() {
    let device_manager = DeviceManager::new();
    assert!(device_manager.get_devices().is_empty());
    assert!(device_manager.last_scan.is_none());
}

#[tokio::test]
async fn test_device_filtering() {
    let mut device_manager = DeviceManager::new();
    
    // Create test devices
    let hdd_device = StorageDevice {
        path: PathBuf::from("/dev/sda"),
        name: "Test HDD".to_string(),
        size: 1000 * 1024 * 1024 * 1024,
        device_type: DeviceType::HDD,
        model: Some("Test HDD Model".to_string()),
        serial: Some("HDD123".to_string()),
        supports_secure_erase: true,
        supports_trim: false,
        hidden_areas: Vec::new(),
    };
    
    let ssd_device = StorageDevice {
        path: PathBuf::from("/dev/sdb"),
        name: "Test SSD".to_string(),
        size: 500 * 1024 * 1024 * 1024,
        device_type: DeviceType::SSD,
        model: Some("Test SSD Model".to_string()),
        serial: Some("SSD456".to_string()),
        supports_secure_erase: true,
        supports_trim: true,
        hidden_areas: Vec::new(),
    };
    
    device_manager.devices.push(hdd_device);
    device_manager.devices.push(ssd_device);
    
    // Test filtering
    let hdd_devices = device_manager.filter_by_type(DeviceType::HDD);
    assert_eq!(hdd_devices.len(), 1);
    assert_eq!(hdd_devices[0].device_type, DeviceType::HDD);
    
    let ssd_devices = device_manager.filter_by_type(DeviceType::SSD);
    assert_eq!(ssd_devices.len(), 1);
    assert_eq!(ssd_devices[0].device_type, DeviceType::SSD);
    
    let secure_erase_devices = device_manager.get_secure_erase_devices();
    assert_eq!(secure_erase_devices.len(), 2);
    
    let trim_devices: Vec<_> = device_manager.get_devices()
        .iter()
        .filter(|d| d.supports_trim)
        .collect();
    assert_eq!(trim_devices.len(), 1);
}

#[tokio::test]
async fn test_advanced_wipe_engine_creation() {
    let wipe_engine = AdvancedWipeEngine::new();
    assert!(wipe_engine.verify_after_wipe);
    assert!(wipe_engine.generate_hash);
    assert_eq!(wipe_engine.max_retries, 3);
}

#[tokio::test]
async fn test_wipe_result_creation() {
    let device = StorageDevice {
        path: PathBuf::from("/dev/sda"),
        name: "Test Device".to_string(),
        size: 1000 * 1024 * 1024 * 1024,
        device_type: DeviceType::HDD,
        model: None,
        serial: None,
        supports_secure_erase: true,
        supports_trim: false,
        hidden_areas: Vec::new(),
    };
    
    let start_time = std::time::SystemTime::now();
    let end_time = std::time::SystemTime::now();
    
    let result = secure_disk_erasure::core::WipeResult {
        device: device.clone(),
        mode: EraseMode::Full,
        start_time,
        end_time,
        duration_seconds: 60,
        bytes_written: device.size,
        verification_passed: true,
        errors: Vec::new(),
    };
    
    assert_eq!(result.mode, EraseMode::Full);
    assert!(result.verification_passed);
    assert_eq!(result.errors.len(), 0);
    assert_eq!(result.bytes_written, device.size);
}

#[tokio::test]
async fn test_erase_mode_serialization() {
    use serde_json;
    
    let quick_mode = EraseMode::Quick;
    let full_mode = EraseMode::Full;
    let advanced_mode = EraseMode::Advanced;
    
    let quick_json = serde_json::to_string(&quick_mode).unwrap();
    let full_json = serde_json::to_string(&full_mode).unwrap();
    let advanced_json = serde_json::to_string(&advanced_mode).unwrap();
    
    assert!(quick_json.contains("Quick"));
    assert!(full_json.contains("Full"));
    assert!(advanced_json.contains("Advanced"));
}

#[tokio::test]
async fn test_device_serialization() {
    use serde_json;
    
    let device = StorageDevice {
        path: PathBuf::from("/dev/sda"),
        name: "Test Device".to_string(),
        size: 1000 * 1024 * 1024 * 1024,
        device_type: DeviceType::HDD,
        model: Some("Test Model".to_string()),
        serial: Some("TEST123".to_string()),
        supports_secure_erase: true,
        supports_trim: false,
        hidden_areas: Vec::new(),
    };
    
    let json = serde_json::to_string(&device).unwrap();
    let deserialized: StorageDevice = serde_json::from_str(&json).unwrap();
    
    assert_eq!(device.name, deserialized.name);
    assert_eq!(device.device_type, deserialized.device_type);
    assert_eq!(device.supports_secure_erase, deserialized.supports_secure_erase);
}

#[tokio::test]
async fn test_utils_format_bytes() {
    use secure_disk_erasure::utils::Utils;
    
    assert_eq!(Utils::format_bytes(0), "0 B");
    assert_eq!(Utils::format_bytes(1024), "1.00 KB");
    assert_eq!(Utils::format_bytes(1024 * 1024), "1.00 MB");
    assert_eq!(Utils::format_bytes(1024 * 1024 * 1024), "1.00 GB");
    assert_eq!(Utils::format_bytes(1024 * 1024 * 1024 * 1024), "1.00 TB");
}

#[tokio::test]
async fn test_utils_estimate_wipe_time() {
    use secure_disk_erasure::utils::Utils;
    
    let size_1gb = 1024 * 1024 * 1024;
    
    let quick_time = Utils::estimate_wipe_time(size_1gb, &EraseMode::Quick);
    let full_time = Utils::estimate_wipe_time(size_1gb, &EraseMode::Full);
    let advanced_time = Utils::estimate_wipe_time(size_1gb, &EraseMode::Advanced);
    
    assert!(quick_time < full_time);
    assert!(full_time < advanced_time);
    assert!(quick_time > 0);
    assert!(full_time > 0);
    assert!(advanced_time > 0);
}

#[tokio::test]
async fn test_utils_generate_filename() {
    use secure_disk_erasure::utils::Utils;
    
    let filename1 = Utils::generate_filename("test", "txt");
    let filename2 = Utils::generate_filename("test", "txt");
    
    assert!(filename1.starts_with("test_"));
    assert!(filename1.ends_with(".txt"));
    assert!(filename2.starts_with("test_"));
    assert!(filename2.ends_with(".txt"));
    
    // Filenames should be different due to timestamps
    assert_ne!(filename1, filename2);
}

#[tokio::test]
async fn test_progress_bar() {
    use secure_disk_erasure::utils::ProgressBar;
    
    let mut progress_bar = ProgressBar::new(100);
    
    assert_eq!(progress_bar.total, 100);
    assert_eq!(progress_bar.current, 0);
    
    progress_bar.update(50);
    assert_eq!(progress_bar.current, 50);
    
    progress_bar.increment();
    assert_eq!(progress_bar.current, 51);
    
    progress_bar.finish();
}
