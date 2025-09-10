use crate::core::{StorageDevice, DeviceType, EraseMode, WipeResult};
use crate::error::Result;
use std::path::PathBuf;
use std::time::SystemTime;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_device_creation() {
        let device = StorageDevice {
            path: PathBuf::from("/dev/sda"),
            name: "Test Device".to_string(),
            size: 1000 * 1024 * 1024 * 1024, // 1TB
            device_type: DeviceType::HDD,
            model: Some("Test Model".to_string()),
            serial: Some("TEST123".to_string()),
            supports_secure_erase: true,
            supports_trim: false,
            hidden_areas: Vec::new(),
        };

        assert_eq!(device.name, "Test Device");
        assert_eq!(device.device_type, DeviceType::HDD);
        assert!(device.supports_secure_erase);
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

        let start_time = SystemTime::now();
        let end_time = SystemTime::now();

        let result = WipeResult {
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
    }

    #[tokio::test]
    async fn test_erase_modes() {
        assert!(matches!(EraseMode::Quick, EraseMode::Quick));
        assert!(matches!(EraseMode::Full, EraseMode::Full));
        assert!(matches!(EraseMode::Advanced, EraseMode::Advanced));
    }
}
