use crate::error::{Result, SecureEraseError};
use crate::core::{StorageDevice, DeviceType, EraseMode, WipeResult, device_manager::DeviceManager, advanced::AdvancedWipeEngine};
use crate::certificates::{enhanced::EnhancedCertificateGenerator, verifier::CertificateVerifier};
use std::path::PathBuf;
use log::{info, warn, error};

/// List available storage devices
pub async fn list_devices(detailed: bool) -> Result<()> {
    info!("Scanning for available storage devices...");
    
    let mut device_manager = DeviceManager::new();
    device_manager.scan_devices().await?;
    let devices = device_manager.get_devices();
    
    if devices.is_empty() {
        println!("No storage devices found.");
        return Ok(());
    }
    
    println!("Found {} storage device(s):", devices.len());
    println!();
    
    for (i, device) in devices.iter().enumerate() {
        println!("{}. {}", i + 1, device.name);
        println!("   Path: {}", device.path.display());
        println!("   Size: {} GB", device.size / (1024 * 1024 * 1024));
        println!("   Type: {:?}", device.device_type);
        
        if detailed {
            if let Some(model) = &device.model {
                println!("   Model: {}", model);
            }
            if let Some(serial) = &device.serial {
                println!("   Serial: {}", serial);
            }
            println!("   Secure Erase: {}", device.supports_secure_erase);
            println!("   TRIM Support: {}", device.supports_trim);
            
            if !device.hidden_areas.is_empty() {
                println!("   Hidden Areas: {}", device.hidden_areas.len());
                for area in &device.hidden_areas {
                    println!("     - {}: {} sectors", area.description, area.size);
                }
            }
        }
        println!();
    }
    
    Ok(())
}

/// Securely erase a storage device
pub async fn wipe_device(
    device_path: PathBuf,
    mode_str: String,
    generate_certificate: bool,
    output_dir: PathBuf,
) -> Result<()> {
    info!("Starting secure erase operation...");
    
    // Parse erase mode
    let mode = match mode_str.to_lowercase().as_str() {
        "quick" => EraseMode::Quick,
        "full" => EraseMode::Full,
        "advanced" => EraseMode::Advanced,
        _ => return Err(SecureEraseError::InvalidEraseMode(mode_str)),
    };
    
    // Find the device
    let mut device_manager = DeviceManager::new();
    device_manager.scan_devices().await?;
    
    let device = device_manager.find_device(&device_path)
        .ok_or_else(|| SecureEraseError::DeviceNotFound(device_path.display().to_string()))?;
    
    // Confirm the operation
    println!("WARNING: This operation will permanently destroy all data on the device!");
    println!("Device: {} ({})", device.name, device_path.display());
    println!("Size: {} GB", device.size / (1024 * 1024 * 1024));
    println!("Mode: {:?}", mode);
    println!();
    
    // Perform the wipe
    info!("Starting wipe operation on device: {}", device_path.display());
    let wipe_engine = AdvancedWipeEngine::new();
    let result = wipe_engine.secure_erase_with_verification(device, mode).await?;
    
    // Display results
    println!("Wipe operation completed!");
    println!("Duration: {} seconds", result.duration_seconds);
    println!("Bytes written: {} GB", result.bytes_written / (1024 * 1024 * 1024));
    println!("Verification: {}", if result.verification_passed { "PASSED" } else { "FAILED" });
    
    if !result.errors.is_empty() {
        warn!("Errors encountered during wipe:");
        for error in &result.errors {
            error!("  - {}", error);
        }
    }
    
    // Generate certificate if requested
    if generate_certificate {
        info!("Generating enhanced wipe certificate...");
        let cert_path = output_dir.join("wipe_certificate.json");
        let pdf_path = output_dir.join("wipe_certificate.pdf");
        let private_key_path = PathBuf::from("private_key.pem");
        
        // Create enhanced certificate generator
        let cert_generator = EnhancedCertificateGenerator::new(
            "Secure Disk Erasure Tool".to_string(),
            "Your Organization".to_string()
        );
        
        cert_generator.generate_enhanced_certificate(&result, &cert_path, &pdf_path, &private_key_path).await?;
        
        println!("Enhanced certificate generated:");
        println!("  JSON: {}", cert_path.display());
        println!("  PDF: {}", pdf_path.display());
    }
    
    Ok(())
}

/// Verify a wipe certificate
pub async fn verify_certificate(
    certificate_path: PathBuf,
    public_key_path: Option<PathBuf>,
) -> Result<()> {
    info!("Verifying certificate: {}", certificate_path.display());
    
    // Create certificate verifier
    let mut verifier = CertificateVerifier::new();
    
    if let Some(key_path) = public_key_path {
        verifier = verifier.with_public_key(key_path.to_string_lossy().to_string());
    }
    
    // Perform verification
    let verification_result = verifier.verify_certificate(&certificate_path).await?;
    
    // Print results
    verification_result.print_result();
    
    if !verification_result.is_valid {
        return Err(SecureEraseError::CertificateVerificationFailed(
            "Certificate verification failed".to_string()
        ));
    }
    
    Ok(())
}

/// Generate signing key pair
pub async fn generate_keys(output_dir: PathBuf) -> Result<()> {
    info!("Generating Ed25519 key pair...");
    
    let (private_key_path, public_key_path) = crate::crypto::generate_key_pair(&output_dir).await?;
    
    println!("Key pair generated successfully:");
    println!("  Private key: {}", private_key_path.display());
    println!("  Public key: {}", public_key_path.display());
    println!();
    println!("IMPORTANT: Keep your private key secure and never share it!");
    println!("The public key can be shared for certificate verification.");
    
    Ok(())
}
