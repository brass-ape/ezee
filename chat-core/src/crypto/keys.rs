use x25519_dalek::{EphemeralSecret, PublicKey, StaticSecret};
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};
use zeroize::Zeroize;
use crate::crypto::CryptoError;

pub struct KeyPair {
    pub public: PublicKey,
    secret: StaticSecret,
}

impl KeyPair {
    pub fn generate() -> Result<Self, CryptoError> {
        let secret = StaticSecret::random_from_rng(OsRng);
        let public = PublicKey::from(&secret);

        Ok(KeyPair { secret, public  })
    }


    pub fn diffie_hellman(&self, their_public: &PublicKey) -> [u8; 32]{
        let shared = self.secret.diffie_hellman(their_public);
        shared.to_bytes()
    }

    pub fn public_bytes(&self) -> [u8; 32] {
        self.public.to_bytes()
    }
}

#[derive(Serialize, Deserialize, Zeroize)]
pub struct KeyPairBytes {
    pub public: [u8; 32],
    pub secret: [u8; 32],
}

impl KeyPair {
    pub fn to_byes(&self) -> KeyPairBytes {
        KeyPairBytes {
            public: self.public.to_bytes(),
            secret: self.secret.to_bytes(),
        }
    }

    pub fn from_bytes(bytes: KeyPairBytes) -> Result<Self, CryptoError> {
        let secret = StaticSecret::from(bytes.secret);
        let public = PublicKey::from(&secret);

        Ok(KeyPair { public, secret })
    }
}