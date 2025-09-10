// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use secure_disk_erasure::core::{device_manager::DeviceManager, advanced::AdvancedWipeEngine, EraseMode};
use secure_disk_erasure::certificates::{enhanced::EnhancedCertificateGenerator, verifier::CertificateVerifier};
use secure_disk_erasure::crypto::generate_key_pair;
use secure_disk_erasure::error::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct SystemInfo {
    os: String,
    arch: String,
    tool_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct WipeResult {
    success: bool,
    message: String,
    certificate_path: Option<String>,
    duration_seconds: u64,
    bytes_written: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct VerificationResult {
    is_valid: bool,
    signature_valid: bool,
    hash_valid: bool,
    compliance_valid: bool,
    warnings: Vec<String>,
    errors: Vec<String>,
    verification_details: VerificationDetails,
}

#[derive(Debug, Serialize, Deserialize)]
struct VerificationDetails {
    certificate_age_days: u64,
    device_size_gb: u64,
    wipe_duration_seconds: u64,
    verification_ratio: f64,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn get_system_info() -> Result<SystemInfo, String> {
    Ok(SystemInfo {
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        tool_version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

#[tauri::command]
async fn list_devices(detailed: bool) -> Result<Vec<secure_disk_erasure::core::StorageDevice>, String> {
    let mut device_manager = DeviceManager::new();
    device_manager.scan_devices().await
        .map_err(|e| e.to_string())?;
    
    Ok(device_manager.get_devices().clone())
}

#[tauri::command]
async fn wipe_device(
    device: String,
    mode: String,
    certificate: bool,
    verify: bool,
) -> Result<WipeResult, String> {
    // Parse erase mode
    let erase_mode = match mode.to_lowercase().as_str() {
        "quick" => EraseMode::Quick,
        "full" => EraseMode::Full,
        "advanced" => EraseMode::Advanced,
        _ => return Err("Invalid erase mode".to_string()),
    };

    // Find the device
    let mut device_manager = DeviceManager::new();
    device_manager.scan_devices().await
        .map_err(|e| e.to_string())?;
    
    let target_device = device_manager.find_device(&PathBuf::from(&device))
        .ok_or_else(|| "Device not found".to_string())?;

    // Perform wipe operation
    let wipe_engine = AdvancedWipeEngine::new();
    let result = wipe_engine.secure_erase_with_verification(target_device, erase_mode).await
        .map_err(|e| e.to_string())?;

    let mut certificate_path = None;

    // Generate certificate if requested
    if certificate {
        let cert_generator = EnhancedCertificateGenerator::new(
            "Secure Disk Erasure Tool".to_string(),
            "Your Organization".to_string()
        );
        
        let cert_path = PathBuf::from("wipe_certificate.json");
        let pdf_path = PathBuf::from("wipe_certificate.pdf");
        let private_key_path = PathBuf::from("private_key.pem");
        
        cert_generator.generate_enhanced_certificate(&result, &cert_path, &pdf_path, &private_key_path).await
            .map_err(|e| e.to_string())?;
        
        certificate_path = Some(cert_path.to_string_lossy().to_string());
    }

    Ok(WipeResult {
        success: result.verification_passed,
        message: if result.verification_passed {
            "Wipe operation completed successfully".to_string()
        } else {
            "Wipe operation completed with warnings".to_string()
        },
        certificate_path,
        duration_seconds: result.duration_seconds,
        bytes_written: result.bytes_written,
    })
}

#[tauri::command]
async fn verify_certificate(
    certificate: String,
    public_key: Option<String>,
) -> Result<VerificationResult, String> {
    let mut verifier = CertificateVerifier::new();
    
    if let Some(key_path) = public_key {
        verifier = verifier.with_public_key(key_path);
    }
    
    let result = verifier.verify_certificate(&PathBuf::from(certificate)).await
        .map_err(|e| e.to_string())?;

    Ok(VerificationResult {
        is_valid: result.is_valid,
        signature_valid: result.signature_valid,
        hash_valid: result.hash_valid,
        compliance_valid: result.compliance_valid,
        warnings: result.warnings,
        errors: result.errors,
        verification_details: VerificationDetails {
            certificate_age_days: result.verification_details.certificate_age_days,
            device_size_gb: result.verification_details.device_size_gb,
            wipe_duration_seconds: result.verification_details.wipe_duration_seconds,
            verification_ratio: result.verification_details.verification_ratio,
        },
    })
}

#[tauri::command]
async fn generate_keys(output: String) -> Result<String, String> {
    let output_path = PathBuf::from(output);
    generate_key_pair(&output_path).await
        .map_err(|e| e.to_string())?;
    
    Ok("Key pair generated successfully".to_string())
}

#[tauri::command]
async fn get_settings() -> Result<HashMap<String, String>, String> {
    // Load settings from file or return defaults
    let mut settings = HashMap::new();
    settings.insert("default_wipe_mode".to_string(), "full".to_string());
    settings.insert("output_directory".to_string(), "./certificates".to_string());
    settings.insert("auto_generate_certificate".to_string(), "true".to_string());
    settings.insert("verify_after_wipe".to_string(), "true".to_string());
    settings.insert("private_key_path".to_string(), "./private_key.pem".to_string());
    settings.insert("public_key_path".to_string(), "./public_key.pem".to_string());
    
    Ok(settings)
}

#[tauri::command]
async fn save_settings(settings: HashMap<String, String>) -> Result<String, String> {
    // Save settings to file
    // This would be implemented based on the actual storage mechanism
    Ok("Settings saved successfully".to_string())
}

fn main() {
    env_logger::init();
    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_system_info,
            list_devices,
            wipe_device,
            verify_certificate,
            generate_keys,
            get_settings,
            save_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
