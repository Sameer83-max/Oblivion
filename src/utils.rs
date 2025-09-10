use crate::error::{Result, SecureEraseError};
use std::path::PathBuf;
use std::fs;
use log::{info, warn, error};

/// Utility functions for the secure disk erasure tool
pub struct Utils;

impl Utils {
    /// Format bytes into human-readable format
    pub fn format_bytes(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
        const THRESHOLD: u64 = 1024;
        
        if bytes == 0 {
            return "0 B".to_string();
        }
        
        let mut size = bytes as f64;
        let mut unit_index = 0;
        
        while size >= THRESHOLD as f64 && unit_index < UNITS.len() - 1 {
            size /= THRESHOLD as f64;
            unit_index += 1;
        }
        
        if unit_index == 0 {
            format!("{} {}", bytes, UNITS[unit_index])
        } else {
            format!("{:.2} {}", size, UNITS[unit_index])
        }
    }
    
    /// Calculate estimated wipe time based on device size and mode
    pub fn estimate_wipe_time(size_bytes: u64, mode: &crate::core::EraseMode) -> u64 {
        let size_gb = size_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
        
        match mode {
            crate::core::EraseMode::Quick => {
                // Quick wipe: ~100 MB/s
                (size_gb * 10.0) as u64
            }
            crate::core::EraseMode::Full => {
                // Full wipe: ~50 MB/s (multiple passes)
                (size_gb * 20.0) as u64
            }
            crate::core::EraseMode::Advanced => {
                // Advanced wipe: ~25 MB/s (hardware secure erase + verification)
                (size_gb * 40.0) as u64
            }
        }
    }
    
    /// Validate device path
    pub fn validate_device_path(path: &PathBuf) -> Result<()> {
        if !path.exists() {
            return Err(SecureEraseError::DeviceNotFound(
                format!("Device path does not exist: {}", path.display())
            ));
        }
        
        // Check if path looks like a device path
        let path_str = path.to_string_lossy();
        
        #[cfg(target_os = "windows")]
        {
            if !path_str.starts_with("\\\\.\\PhysicalDrive") {
                return Err(SecureEraseError::DeviceNotFound(
                    "Invalid Windows device path format".to_string()
                ));
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            if !path_str.starts_with("/dev/") {
                return Err(SecureEraseError::DeviceNotFound(
                    "Invalid Linux device path format".to_string()
                ));
            }
        }
        
        Ok(())
    }
    
    /// Create output directory if it doesn't exist
    pub fn ensure_output_directory(path: &PathBuf) -> Result<()> {
        if !path.exists() {
            info!("Creating output directory: {}", path.display());
            fs::create_dir_all(path)?;
        }
        Ok(())
    }
    
    /// Generate unique filename with timestamp
    pub fn generate_filename(prefix: &str, extension: &str) -> String {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        format!("{}_{:016X}.{}", prefix, timestamp, extension)
    }
    
    /// Check if running with sufficient privileges
    pub fn check_privileges() -> Result<()> {
        #[cfg(target_os = "windows")]
        {
            // Check if running as administrator
            use std::process::Command;
            
            let output = Command::new("net")
                .args(&["session"])
                .output()
                .map_err(|e| SecureEraseError::Io(e))?;
            
            if !output.status.success() {
                return Err(SecureEraseError::PermissionDenied(
                    "Administrator privileges required on Windows".to_string()
                ));
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            // Check if running as root or in disk group
            use std::process::Command;
            
            let output = Command::new("groups")
                .output()
                .map_err(|e| SecureEraseError::Io(e))?;
            
            if output.status.success() {
                let groups = String::from_utf8_lossy(&output.stdout);
                if !groups.contains("disk") && !groups.contains("root") {
                    warn!("User may not have sufficient privileges for disk operations");
                }
            }
        }
        
        Ok(())
    }
    
    /// Log system information
    pub fn log_system_info() {
        info!("System Information:");
        info!("  OS: {}", std::env::consts::OS);
        info!("  Architecture: {}", std::env::consts::ARCH);
        info!("  Family: {}", std::env::consts::FAMILY);
        
        #[cfg(target_os = "windows")]
        {
            if let Ok(version) = std::env::var("OS") {
                info!("  Windows Version: {}", version);
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            if let Ok(release) = fs::read_to_string("/etc/os-release") {
                for line in release.lines() {
                    if line.starts_with("PRETTY_NAME=") {
                        info!("  Linux Distribution: {}", line);
                        break;
                    }
                }
            }
        }
    }
    
    /// Create progress bar for long operations
    pub fn create_progress_bar(total: u64) -> ProgressBar {
        ProgressBar::new(total)
    }
}

/// Simple progress bar implementation
pub struct ProgressBar {
    total: u64,
    current: u64,
    width: usize,
}

impl ProgressBar {
    pub fn new(total: u64) -> Self {
        Self {
            total,
            current: 0,
            width: 50,
        }
    }
    
    pub fn update(&mut self, current: u64) {
        self.current = current;
        self.display();
    }
    
    pub fn increment(&mut self) {
        self.current += 1;
        self.display();
    }
    
    fn display(&self) {
        let percentage = (self.current as f64 / self.total as f64) * 100.0;
        let filled = (percentage / 100.0 * self.width as f64) as usize;
        
        let bar = "█".repeat(filled) + &"░".repeat(self.width - filled);
        
        print!("\r[{}] {:.1}% ({}/{})", bar, percentage, self.current, self.total);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }
    
    pub fn finish(&self) {
        println!(); // New line after progress bar
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_bytes() {
        assert_eq!(Utils::format_bytes(0), "0 B");
        assert_eq!(Utils::format_bytes(1024), "1.00 KB");
        assert_eq!(Utils::format_bytes(1024 * 1024), "1.00 MB");
        assert_eq!(Utils::format_bytes(1024 * 1024 * 1024), "1.00 GB");
    }
    
    #[test]
    fn test_estimate_wipe_time() {
        let size_1gb = 1024 * 1024 * 1024;
        
        let quick_time = Utils::estimate_wipe_time(size_1gb, &crate::core::EraseMode::Quick);
        let full_time = Utils::estimate_wipe_time(size_1gb, &crate::core::EraseMode::Full);
        let advanced_time = Utils::estimate_wipe_time(size_1gb, &crate::core::EraseMode::Advanced);
        
        assert!(quick_time < full_time);
        assert!(full_time < advanced_time);
    }
    
    #[test]
    fn test_generate_filename() {
        let filename = Utils::generate_filename("test", "txt");
        assert!(filename.starts_with("test_"));
        assert!(filename.ends_with(".txt"));
        assert!(filename.len() > 10); // Should include timestamp
    }
}
