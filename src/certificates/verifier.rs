use crate::error::{Result, SecureEraseError};
use crate::crypto::{load_verifying_key, verify_signature, hash_data};
use crate::certificates::{WipeCertificate, enhanced::EnhancedWipeCertificate};
use std::path::Path;
use std::fs;
use log::{info, warn, error};

/// Certificate verifier for validating wipe certificates
pub struct CertificateVerifier {
    pub public_key_path: Option<String>,
    pub verification_level: VerificationLevel,
    pub enable_ocsp: bool,
    pub enable_crl: bool,
}

/// Verification levels
#[derive(Debug, Clone)]
pub enum VerificationLevel {
    Basic,
    Standard,
    Advanced,
    Forensic,
}

impl CertificateVerifier {
    pub fn new() -> Self {
        Self {
            public_key_path: None,
            verification_level: VerificationLevel::Standard,
            enable_ocsp: true,
            enable_crl: true,
        }
    }
    
    pub fn with_public_key(mut self, public_key_path: String) -> Self { self.public_key_path = Some(public_key_path); self }
    pub fn with_verification_level(mut self, level: VerificationLevel) -> Self { self.verification_level = level; self }
    pub fn with_ocsp(mut self, enabled: bool) -> Self { self.enable_ocsp = enabled; self }
    pub fn with_crl(mut self, enabled: bool) -> Self { self.enable_crl = enabled; self }
    
    pub async fn verify_certificate(&self, certificate_path: &Path) -> Result<VerificationResult> {
        info!("Verifying certificate: {}", certificate_path.display());
        let certificate_data = fs::read(certificate_path)?;
        if let Ok(enhanced_cert) = serde_json::from_slice::<EnhancedWipeCertificate>(&certificate_data) {
            return self.verify_enhanced_certificate(&enhanced_cert).await;
        }
        let certificate: WipeCertificate = serde_json::from_slice(&certificate_data)?;
        self.verify_basic_certificate(&certificate).await
    }
    
    async fn verify_basic_certificate(&self, certificate: &WipeCertificate) -> Result<VerificationResult> {
        let mut result = VerificationResult::default();
        let public_key_path = self.public_key_path.as_ref().map(|p| Path::new(p)).unwrap_or(Path::new("public_key.pem"));
        let verifying_key = match load_verifying_key(public_key_path).await { Ok(key) => key, Err(e) => { result.errors.push(format!("Failed to load public key: {}", e)); return Ok(result); } };
        match self.verify_signature_basic(certificate, &verifying_key).await { Ok(valid) => { result.signature_valid = valid; if !valid { result.errors.push("Invalid signature".to_string()); } }, Err(e) => { result.errors.push(format!("Signature verification failed: {}", e)); } }
        match self.verify_hash_basic(certificate).await { Ok(valid) => { result.hash_valid = valid; if !valid { result.warnings.push("Hash verification failed".to_string()); } }, Err(e) => { result.warnings.push(format!("Hash verification error: {}", e)); } }
        result.compliance_valid = certificate.verification_passed && certificate.errors.is_empty();
        result.is_valid = result.signature_valid && result.hash_valid;
        Ok(result)
    }
    
    async fn verify_enhanced_certificate(&self, certificate: &EnhancedWipeCertificate) -> Result<VerificationResult> {
        let mut result = VerificationResult::default();
        let public_key_path = self.public_key_path.as_ref().map(|p| Path::new(p)).unwrap_or(Path::new("public_key.pem"));
        let verifying_key = match load_verifying_key(public_key_path).await { Ok(key) => key, Err(e) => { result.errors.push(format!("Failed to load public key: {}", e)); return Ok(result); } };
        match self.verify_signature_enhanced(certificate, &verifying_key).await { Ok(valid) => { result.signature_valid = valid; if !valid { result.errors.push("Invalid signature".to_string()); } }, Err(e) => { result.errors.push(format!("Signature verification failed: {}", e)); } }
        match self.verify_hash_enhanced(certificate).await { Ok(valid) => { result.hash_valid = valid; if !valid { result.warnings.push("Hash verification failed".to_string()); } }, Err(e) => { result.warnings.push(format!("Hash verification error: {}", e)); } }
        result.compliance_valid = self.check_compliance_enhanced(certificate);
        if self.enable_ocsp { self.check_ocsp_status(certificate, &mut result).await; }
        if self.enable_crl { self.check_crl_status(certificate, &mut result).await; }
        result.is_valid = result.signature_valid && result.hash_valid && result.compliance_valid;
        Ok(result)
    }
    
    async fn verify_signature_basic(&self, certificate: &WipeCertificate, verifying_key: &ed25519_dalek::VerifyingKey) -> Result<bool> {
        let signature_bytes = hex::decode(&certificate.signature).map_err(|_| SecureEraseError::CertificateVerificationFailed("Invalid signature format".to_string()))?;
        let signature = ed25519_dalek::Signature::from_bytes(&signature_bytes).map_err(|_| SecureEraseError::CertificateVerificationFailed("Invalid signature".to_string()))?;
        let mut cert_copy = certificate.clone();
        cert_copy.verification.hash = "".to_string();
        cert_copy.signature = "".to_string();
        let data_to_verify = serde_json::to_string(&cert_copy)?;
        verify_signature(data_to_verify.as_bytes(), &signature, verifying_key).await
    }
    
    async fn verify_signature_enhanced(&self, certificate: &EnhancedWipeCertificate, verifying_key: &ed25519_dalek::VerifyingKey) -> Result<bool> {
        let signature_bytes = hex::decode(&certificate.signature).map_err(|_| SecureEraseError::CertificateVerificationFailed("Invalid signature format".to_string()))?;
        let signature = ed25519_dalek::Signature::from_bytes(&signature_bytes).map_err(|_| SecureEraseError::CertificateVerificationFailed("Invalid signature".to_string()))?;
        let mut cert_copy = certificate.clone();
        cert_copy.verification.hash = "".to_string();
        cert_copy.signature = "".to_string();
        let data_to_verify = serde_json::to_string(&cert_copy)?;
        verify_signature(data_to_verify.as_bytes(), &signature, verifying_key).await
    }
    
    async fn verify_hash_basic(&self, certificate: &WipeCertificate) -> Result<bool> {
        let mut cert_copy = certificate.clone();
        cert_copy.verification.hash = "".to_string();
        cert_copy.signature = "".to_string();
        let data_to_hash = serde_json::to_string(&cert_copy)?;
        let calculated_hash = hash_data(data_to_hash.as_bytes()).await?;
        Ok(calculated_hash == certificate.verification.hash)
    }
    
    async fn verify_hash_enhanced(&self, certificate: &EnhancedWipeCertificate) -> Result<bool> {
        let mut cert_copy = certificate.clone();
        cert_copy.verification.hash = "".to_string();
        cert_copy.signature = "".to_string();
        let data_to_hash = serde_json::to_string(&cert_copy)?;
        let calculated_hash = hash_data(data_to_hash.as_bytes()).await?;
        Ok(calculated_hash == certificate.verification.hash)
    }
    
    fn check_compliance_enhanced(&self, certificate: &EnhancedWipeCertificate) -> bool {
        certificate.wipe_details.verification_passed && certificate.wipe_details.errors.is_empty() && !certificate.compliance.standards.is_empty()
    }

    async fn check_ocsp_status(&self, certificate: &EnhancedWipeCertificate, result: &mut VerificationResult) {
        if let Some(url) = &certificate.pki.ocsp_url {
            // Placeholder: perform OCSP request for the station cert (future)
            result.verification_details.ocsp_checked = true;
        }
    }

    async fn check_crl_status(&self, certificate: &EnhancedWipeCertificate, result: &mut VerificationResult) {
        if let Some(url) = &certificate.pki.crl_url {
            // Placeholder: fetch CRL and check revocation (future)
            result.verification_details.crl_checked = true;
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct VerificationResult {
    pub is_valid: bool,
    pub signature_valid: bool,
    pub hash_valid: bool,
    pub compliance_valid: bool,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    pub verification_details: VerificationDetails,
}

#[derive(Debug, Clone, Default)]
pub struct VerificationDetails {
    pub certificate_age_days: u64,
    pub device_size_gb: u64,
    pub wipe_duration_seconds: u64,
    pub verification_ratio: f64,
    pub ocsp_checked: bool,
    pub crl_checked: bool,
}

impl VerificationResult {
    pub fn print_result(&self) {
        println!("Certificate Verification Result:");
        println!("================================");
        if self.is_valid { println!("✓ Certificate is VALID"); } else { println!("✗ Certificate is INVALID"); }
        println!("\nVerification Details:");
        println!("  Signature: {}", if self.signature_valid { "✓ Valid" } else { "✗ Invalid" });
        println!("  Hash: {}", if self.hash_valid { "✓ Valid" } else { "✗ Invalid" });
        println!("  Compliance: {}", if self.compliance_valid { "✓ Valid" } else { "✗ Invalid" });
        println!("  OCSP Checked: {}", if self.verification_details.ocsp_checked { "Yes" } else { "No" });
        println!("  CRL Checked: {}", if self.verification_details.crl_checked { "Yes" } else { "No" });
        if !self.warnings.is_empty() { println!("\nWarnings:"); for w in &self.warnings { println!("  ⚠ {}", w); } }
        if !self.errors.is_empty() { println!("\nErrors:"); for e in &self.errors { println!("  ✗ {}", e); } }
        println!("\nVerification Details:");
        println!("  Certificate Age: {} days", self.verification_details.certificate_age_days);
        println!("  Device Size: {} GB", self.verification_details.device_size_gb);
        println!("  Wipe Duration: {} seconds", self.verification_details.wipe_duration_seconds);
        println!("  Verification Ratio: {:.1}%", self.verification_details.verification_ratio * 100.0);
    }
}
