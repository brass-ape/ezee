use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("Failed to generate keypair")]
    KeyGenerationFailed,

    #[error("Encryption failed {0}")]
    EncryptionFailed(String),

    #[error("Decryption failed - message may have been tampered with")]
    DecryptionFailed,

    #[error("Invalid nonce length: expected 12 bytes, got {0}")]
    InvalidNonce(usize),

    #[error("Failed to serialize key: {0}")]
    SerializationFailed(String),
}