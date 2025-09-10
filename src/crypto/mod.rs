use crate::error::{Result, SecureEraseError};
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use std::path::Path;
use std::fs;

/// Generate a new Ed25519 key pair
pub async fn generate_key_pair(output_dir: &Path) -> Result<(std::path::PathBuf, std::path::PathBuf)> {
    let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying_key = signing_key.verifying_key();
    
    let private_key_path = output_dir.join("private_key.pem");
    let public_key_path = output_dir.join("public_key.pem");
    
    // Save private key (PEM format)
    let private_pem = format!(
        "-----BEGIN PRIVATE KEY-----\n{}\n-----END PRIVATE KEY-----\n",
        base64::encode(signing_key.to_bytes())
    );
    fs::write(&private_key_path, private_pem)?;
    
    // Save public key (PEM format)
    let public_pem = format!(
        "-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----\n",
        base64::encode(verifying_key.to_bytes())
    );
    fs::write(&public_key_path, public_pem)?;
    
    Ok((private_key_path, public_key_path))
}

/// Load a signing key from file
pub async fn load_signing_key(key_path: &Path) -> Result<SigningKey> {
    let key_data = fs::read(key_path)?;
    let key_str = String::from_utf8(key_data)
        .map_err(|_| SecureEraseError::Crypto("Invalid key file format".to_string()))?;
    
    // Parse PEM format
    let key_str = key_str
        .replace("-----BEGIN PRIVATE KEY-----", "")
        .replace("-----END PRIVATE KEY-----", "")
        .replace('\n', "");
    
    let key_bytes = base64::decode(&key_str)
        .map_err(|_| SecureEraseError::Crypto("Invalid base64 encoding".to_string()))?;
    
    SigningKey::from_bytes(&key_bytes)
        .map_err(|_| SecureEraseError::Crypto("Invalid key format".to_string()))
}

/// Load a verifying key from file
pub async fn load_verifying_key(key_path: &Path) -> Result<VerifyingKey> {
    let key_data = fs::read(key_path)?;
    let key_str = String::from_utf8(key_data)
        .map_err(|_| SecureEraseError::Crypto("Invalid key file format".to_string()))?;
    
    // Parse PEM format
    let key_str = key_str
        .replace("-----BEGIN PUBLIC KEY-----", "")
        .replace("-----END PUBLIC KEY-----", "")
        .replace('\n', "");
    
    let key_bytes = base64::decode(&key_str)
        .map_err(|_| SecureEraseError::Crypto("Invalid base64 encoding".to_string()))?;
    
    VerifyingKey::from_bytes(&key_bytes)
        .map_err(|_| SecureEraseError::Crypto("Invalid key format".to_string()))
}

/// Sign data with a signing key
pub async fn sign_data(data: &[u8], signing_key: &SigningKey) -> Result<Signature> {
    Ok(signing_key.sign(data))
}

/// Verify a signature
pub async fn verify_signature(data: &[u8], signature: &Signature, verifying_key: &VerifyingKey) -> Result<bool> {
    Ok(verifying_key.verify(data, signature).is_ok())
}

/// Generate a cryptographic hash of data
pub async fn hash_data(data: &[u8]) -> Result<String> {
    use sha2::{Sha256, Digest};
    
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    
    Ok(hex::encode(result))
}
