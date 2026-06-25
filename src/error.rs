use std::fmt;
use std::io;

#[derive(Debug)]
pub enum PasswordManagerError {
    VaultNotFound,
    InvalidPassword,
    CorruptedVault(String),
    IoError(io::Error),
    SerializationError(String),
    CryptoError(String),
    InvalidInput(String),
}

impl fmt::Display for PasswordManagerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::VaultNotFound => write!(f, "Vault file not found"),
            Self::InvalidPassword => write!(f, "Invalid master password or corrupted vault"),
            Self::CorruptedVault(msg) => write!(f, "Vault is corrupted: {}", msg),
            Self::IoError(err) => write!(f, "File I/O error: {}", err),
            Self::SerializationError(msg) => write!(f, "Data serialization error: {}", msg),
            Self::CryptoError(msg) => write!(f, "Encryption/decryption error: {}", msg),
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for PasswordManagerError {}

// Allow automatic conversion from io::Error
impl From<io::Error> for PasswordManagerError {
    fn from(err: io::Error) -> Self {
        PasswordManagerError::IoError(err)
    }
}

// Allow automatic conversion from serde_json::Error
impl From<serde_json::Error> for PasswordManagerError {
    fn from(err: serde_json::Error) -> Self {
        PasswordManagerError::SerializationError(err.to_string())
    }
}

// Allow automatic conversion from base64 DecodeError
impl From<base64::DecodeError> for PasswordManagerError {
    fn from(err: base64::DecodeError) -> Self {
        PasswordManagerError::CryptoError(err.to_string())
    }
}

// Allow automatic conversion from argon2 errors
impl From<argon2::password_hash::Error> for PasswordManagerError {
    fn from(err: argon2::password_hash::Error) -> Self {
        PasswordManagerError::CryptoError(err.to_string())
    }
}

// Allow automatic conversion from chacha20poly1305 error
impl From<chacha20poly1305::aead::Error> for PasswordManagerError {
    fn from(_err: chacha20poly1305::aead::Error) -> Self {
        PasswordManagerError::CryptoError("Encryption/decryption failed".to_string())
    }
}
