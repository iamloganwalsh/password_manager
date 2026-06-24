use std::fs;
use std::path::Path;
use argon2::password_hash::SaltString;

use crate::vault::Vault;
use crate::crypt::{encrypt_vault, decrypt_vault, EncryptedVault};
use crate::crypt::{derive_key, generate_salt};

pub fn write_vault(
    path: &str,
    vault: &Vault,
    password: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let salt = generate_salt();
    let key = derive_key(password, &salt)?;

    let encrypted = encrypt_vault(vault, &key, salt.as_str())?;

    let json = serde_json::to_string_pretty(&encrypted)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn read_vault(
    path: &str,
    password: &str
) -> Result<Vault, Box<dyn std::error::Error>> {
    if !Path::new(path).exists() {
        return Ok(Vault::new());
    }

    let data = fs::read_to_string(path)?;
    let encrypted: EncryptedVault = serde_json::from_str(&data)?;

    let salt = SaltString::new(&encrypted.salt)?;
    let key = derive_key(password, &salt)?;

    let vault = decrypt_vault(encrypted, &key)?;
    Ok(vault)
}

pub fn vault_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}