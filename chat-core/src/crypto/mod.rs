// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2026 Brass-ape

pub mod error;
pub mod keys;
pub mod encrypt;

pub use error::CryptoError;
pub use keys::KeyPair;
pub use encrypt::{encrypt, decrypt, SessionKey, EncryptedMessage};