use crate::error::{Result, SecureEraseError};
use crate::core::WipeResult;
use crate::crypto::{sign_data, load_signing_key, hash_data};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

pub mod enhanced;
pub mod verifier;

/// Digital certificate for wipe operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WipeCertificate {
    pub version: String,
    pub certificate_id: String,
    pub timestamp: u64,
    pub device_info: DeviceInfo,
    pub wipe_details: WipeDetails,
    pub verification: VerificationInfo,
    pub signature: String,
}

/// Device information in certificate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub device_type: String,
    pub model: Option<String>,
    pub serial: Option<String>,
}

/// Wipe operation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WipeDetails {
    pub mode: String,
    pub start_time: u64,
    pub end_time: u64,
    pub duration_seconds: u64,
    pub bytes_written: u64,
    pub verification_passed: bool,
    pub errors: Vec<String>,
}

/// Verification information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationInfo {
    pub hash: String,
    pub algorithm: String,
    pub public_key_fingerprint: String,
}

/// Generate a digital certificate for a wipe operation
pub async fn generate_certificate(
    wipe_result: &WipeResult,
    json_path: &Path,
    pdf_path: &Path,
) -> Result<()> {
    // Load signing key (in production, this should be from secure storage)
    let signing_key_path = Path::new("private_key.pem");
    let signing_key = load_signing_key(signing_key_path).await?;
    
    // Create certificate data
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let certificate_id = format!("WIPE_{:016X}", timestamp);
    
    let device_info = DeviceInfo {
        path: wipe_result.device.path.to_string_lossy().to_string(),
        name: wipe_result.device.name.clone(),
        size: wipe_result.device.size,
        device_type: format!("{:?}", wipe_result.device.device_type),
        model: wipe_result.device.model.clone(),
        serial: wipe_result.device.serial.clone(),
    };
    
    let wipe_details = WipeDetails {
        mode: format!("{:?}", wipe_result.mode),
        start_time: wipe_result.start_time
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        end_time: wipe_result.end_time
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        duration_seconds: wipe_result.duration_seconds,
        bytes_written: wipe_result.bytes_written,
        verification_passed: wipe_result.verification_passed,
        errors: wipe_result.errors.clone(),
    };
    
    // Create verification info
    let certificate_data = serde_json::to_string(&WipeCertificate {
        version: "1.0".to_string(),
        certificate_id: certificate_id.clone(),
        timestamp,
        device_info: device_info.clone(),
        wipe_details: wipe_details.clone(),
        verification: VerificationInfo {
            hash: "".to_string(), // Will be filled after signing
            algorithm: "SHA-256".to_string(),
            public_key_fingerprint: "".to_string(), // Will be filled
        },
        signature: "".to_string(), // Will be filled after signing
    })?;
    
    // Generate hash and signature
    let hash = hash_data(certificate_data.as_bytes()).await?;
    let signature = sign_data(certificate_data.as_bytes(), &signing_key).await?;
    
    // Create final certificate
    let certificate = WipeCertificate {
        version: "1.0".to_string(),
        certificate_id,
        timestamp,
        device_info,
        wipe_details,
        verification: VerificationInfo {
            hash,
            algorithm: "SHA-256".to_string(),
            public_key_fingerprint: "ED25519".to_string(), // Simplified for now
        },
        signature: hex::encode(signature.to_bytes()),
    };
    
    // Save JSON certificate
    let json_data = serde_json::to_string_pretty(&certificate)?;
    fs::write(json_path, json_data)?;
    
    // Generate PDF certificate
    generate_pdf_certificate(&certificate, pdf_path).await?;
    
    Ok(())
}

/// Generate PDF certificate
async fn generate_pdf_certificate(certificate: &WipeCertificate, pdf_path: &Path) -> Result<()> {
    use printpdf::*;
    use std::io::BufWriter;
    
    let (doc, page1, layer1) = PdfDocument::new("Secure Disk Erasure Certificate", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    
    // Title
    let font = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;
    current_layer.set_font(&font, 24.0);
    current_layer.begin_text_section();
    current_layer.write_text("Secure Disk Erasure Certificate", &font, 24.0, Mm(20.0), Mm(270.0), &[]);
    current_layer.end_text_section();
    
    // Certificate ID
    current_layer.set_font(&font, 12.0);
    current_layer.begin_text_section();
    current_layer.write_text(&format!("Certificate ID: {}", certificate.certificate_id), &font, 12.0, Mm(20.0), Mm(250.0), &[]);
    current_layer.end_text_section();
    
    // Timestamp
    current_layer.write_text(&format!("Generated: {}", chrono::DateTime::from_timestamp(certificate.timestamp as i64, 0).unwrap().format("%Y-%m-%d %H:%M:%S UTC")), &font, 12.0, Mm(20.0), Mm(240.0), &[]);
    
    // Device Information
    current_layer.write_text("Device Information:", &font, 14.0, Mm(20.0), Mm(220.0), &[]);
    current_layer.write_text(&format!("Device: {}", certificate.device_info.name), &font, 12.0, Mm(30.0), Mm(210.0), &[]);
    current_layer.write_text(&format!("Path: {}", certificate.device_info.path), &font, 12.0, Mm(30.0), Mm(200.0), &[]);
    current_layer.write_text(&format!("Size: {} GB", certificate.device_info.size / (1024 * 1024 * 1024)), &font, 12.0, Mm(30.0), Mm(190.0), &[]);
    current_layer.write_text(&format!("Type: {}", certificate.device_info.device_type), &font, 12.0, Mm(30.0), Mm(180.0), &[]);
    
    if let Some(model) = &certificate.device_info.model {
        current_layer.write_text(&format!("Model: {}", model), &font, 12.0, Mm(30.0), Mm(170.0), &[]);
    }
    
    // Wipe Details
    current_layer.write_text("Wipe Details:", &font, 14.0, Mm(20.0), Mm(150.0), &[]);
    current_layer.write_text(&format!("Mode: {}", certificate.wipe_details.mode), &font, 12.0, Mm(30.0), Mm(140.0), &[]);
    current_layer.write_text(&format!("Duration: {} seconds", certificate.wipe_details.duration_seconds), &font, 12.0, Mm(30.0), Mm(130.0), &[]);
    current_layer.write_text(&format!("Bytes Written: {} GB", certificate.wipe_details.bytes_written / (1024 * 1024 * 1024)), &font, 12.0, Mm(30.0), Mm(120.0), &[]);
    current_layer.write_text(&format!("Verification: {}", if certificate.wipe_details.verification_passed { "PASSED" } else { "FAILED" }), &font, 12.0, Mm(30.0), Mm(110.0), &[]);
    
    // Verification Info
    current_layer.write_text("Verification:", &font, 14.0, Mm(20.0), Mm(90.0), &[]);
    current_layer.write_text(&format!("Hash: {}", certificate.verification.hash), &font, 10.0, Mm(30.0), Mm(80.0), &[]);
    current_layer.write_text(&format!("Algorithm: {}", certificate.verification.algorithm), &font, 12.0, Mm(30.0), Mm(70.0), &[]);
    current_layer.write_text(&format!("Signature: {}", &certificate.signature[..32]), &font, 10.0, Mm(30.0), Mm(60.0), &[]);
    
    // Footer
    current_layer.write_text("This certificate provides cryptographic proof of secure data erasure.", &font, 10.0, Mm(20.0), Mm(30.0), &[]);
    current_layer.write_text("The signature can be verified using the corresponding public key.", &font, 10.0, Mm(20.0), Mm(20.0), &[]);
    
    current_layer.end_text_section();
    
    // Save PDF
    doc.save(&mut BufWriter::new(fs::File::create(pdf_path)?))?;
    
    Ok(())
}

/// Verify a wipe certificate
pub async fn verify_certificate(
    certificate_path: &Path,
    public_key_path: Option<&Path>,
) -> Result<bool> {
    let certificate_data = fs::read(certificate_path)?;
    let certificate: WipeCertificate = serde_json::from_slice(&certificate_data)?;
    
    // Load public key
    let public_key_path = public_key_path.unwrap_or(Path::new("public_key.pem"));
    let verifying_key = crate::crypto::load_verifying_key(public_key_path).await?;
    
    // Verify signature
    let signature_bytes = hex::decode(&certificate.signature)
        .map_err(|_| SecureEraseError::CertificateVerificationFailed("Invalid signature format".to_string()))?;
    
    let signature = ed25519_dalek::Signature::from_bytes(&signature_bytes)
        .map_err(|_| SecureEraseError::CertificateVerificationFailed("Invalid signature".to_string()))?;
    
    // Create data to verify (certificate without signature)
    let mut cert_copy = certificate.clone();
    cert_copy.signature = "".to_string();
    let data_to_verify = serde_json::to_string(&cert_copy)?;
    
    // Verify signature
    let is_valid = crate::crypto::verify_signature(data_to_verify.as_bytes(), &signature, &verifying_key).await?;
    
    Ok(is_valid)
}
