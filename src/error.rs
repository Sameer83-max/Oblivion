use thiserror::Error;

/// Custom error types for the secure disk erasure tool
#[derive(Error, Debug)]
pub enum SecureEraseError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Cryptographic error: {0}")]
    Crypto(String),
    
    #[error("Device not found: {0}")]
    DeviceNotFound(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Unsupported platform")]
    UnsupportedPlatform,
    
    #[error("Unsupported device type: {0}")]
    UnsupportedDeviceType(String),
    
    #[error("Secure erase not supported on this device")]
    SecureEraseNotSupported,
    
    #[error("Wipe operation failed: {0}")]
    WipeFailed(String),
    
    #[error("Verification failed: {0}")]
    VerificationFailed(String),
    
    #[error("Certificate generation failed: {0}")]
    CertificateGenerationFailed(String),
    
    #[error("Certificate verification failed: {0}")]
    CertificateVerificationFailed(String),
    
    #[error("Invalid erase mode: {0}")]
    InvalidEraseMode(String),
    
    #[error("Hidden area access failed: {0}")]
    HiddenAreaAccessFailed(String),
}

/// Result type alias for the secure erase tool
pub type Result<T> = std::result::Result<T, SecureEraseError>;
