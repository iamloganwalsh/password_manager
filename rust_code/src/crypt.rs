use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{SaltString};
use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce};
use serde::{Serialize, Deserialize};
use rand::RngCore;
use base64::{Engine as _, engine::general_purpose};

#[derive(Serialize, Deserialize)]
pub struct EncryptedVault {
    pub version: u32,           // Future-proofing
    pub salt: String,           // Used to derive key
    pub nonce: String,          // Randomises encryption
    pub ciphertext: String,     // Encrypted data
}

// Derive 32-byte encryption key from password + salt
pub fn derive_key(password: &str, salt: &SaltString) -> Result<[u8; 32], argon2::password_hash::Error> {
    let argon2 = Argon2::default();

    let hash = argon2.hash_password(password.as_bytes(), salt)?.hash.ok_or(argon2::password_hash::Error::Password)?;

    let mut key = [0u8; 32];

    let bytes = hash.as_bytes();

    key.copy_from_slice(&bytes[..32.min(bytes.len())]);

    Ok(key)
}

pub fn generate_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}

fn generate_nonce() -> [u8; 12] {
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

pub fn encrypt_vault(
    vault: &crate::vault::Vault,
    key: &[u8; 32],
    salt: &str,
) -> Result<EncryptedVault, Box<dyn std::error::Error>> {
    let cipher = ChaCha20Poly1305::new(key.into());

    let nonce_bytes = generate_nonce();
    let nonce = Nonce::from_slice(&nonce_bytes);

    let plaintext = serde_json::to_vec(vault)?;

    let ciphertext = cipher.encrypt(nonce, plaintext.as_ref()).map_err(|e| format!("encryption failed: {e}"))?;

    Ok(EncryptedVault {
        version: 1,
        salt: salt.to_string(),
        nonce: general_purpose::STANDARD.encode(nonce_bytes),
        ciphertext: general_purpose::STANDARD.encode(ciphertext)
    })
}

pub fn decrypt_vault(
    encrypted: EncryptedVault,
    key: &[u8; 32],
) -> Result<crate::vault::Vault, Box<dyn std::error::Error>> {
    let cipher = ChaCha20Poly1305::new(key.into());

    let nonce_bytes = general_purpose::STANDARD.decode(encrypted.nonce)?;
    let ciphertext = general_purpose::STANDARD.decode(encrypted.ciphertext)?;

    let nonce = Nonce::from_slice(&nonce_bytes);

    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).map_err(|e| format!("decryption failed: {e}"))?;

    let vault: crate::vault::Vault = serde_json::from_slice(&plaintext)?;

    Ok(vault)
}
