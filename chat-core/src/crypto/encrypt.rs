// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2026 Brass-ape
use chacha20poly1305::{
    ChaCha20Poly1305, Key, Nonce, aead::{Aead, AeadCore, KeyInit, OsRng}
};
use zeroize::Zeroize;
use crate::crypto::CryptoError;

pub struct EncryptedMessage {
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; 12],
}

#[derive(Zeroize)]
#[zeroize(drop)]
pub struct SessionKey([u8; 32]);

impl SessionKey {
    pub fn from_shared_secret(bytes: [u8; 32]) -> Self {
        SessionKey(bytes)
    }

    fn as_key(&self) -> &Key {
        Key::from_slice(&self.0)
    }
}

pub fn encrypt(
    plaintext: &[u8],
    session_key: &SessionKey,
) -> Result<EncryptedMessage, CryptoError> {
    let cipher = ChaCha20Poly1305::new(session_key.as_key());
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;

    Ok(EncryptedMessage { ciphertext, nonce: nonce.into(),
    })

}

pub fn decrypt(
    message: &EncryptedMessage,
    session_key: &SessionKey,
) -> Result<Vec<u8>,  CryptoError> {
    let cipher = ChaCha20Poly1305::new(session_key.as_key());
    let nonce = Nonce::from_slice(&message.nonce);

    let plaintext = cipher
    .decrypt(nonce, message.ciphertext.as_ref())
    .map_err(|_| CryptoError::DecryptionFailed)?;

    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let key_bytes = [42u8; 32];  // test key — never do this outside tests
        let session_key = SessionKey::from_shared_secret(key_bytes);
        let plaintext = b"hello from the test suite";

        let encrypted = encrypt(plaintext, &session_key)
            .expect("encryption should not fail");

        let decrypted = decrypt(&encrypted, &session_key)
            .expect("decryption should not fail");

        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[test]
    fn wrong_key_fails() {
        let key_bytes = [42u8; 32];
        let wrong_key_bytes = [99u8; 32];

        let session_key = SessionKey::from_shared_secret(key_bytes);
        let wrong_key = SessionKey::from_shared_secret(wrong_key_bytes);

        let encrypted = encrypt(b"secret", &session_key)
            .expect("encryption should not fail");

        let result = decrypt(&encrypted, &wrong_key);

        assert!(result.is_err(), "decryption with wrong key should fail");
    }
}