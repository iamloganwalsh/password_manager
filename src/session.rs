use argon2::password_hash::SaltString;
use std::path::PathBuf;
use crate::vault::{Vault, Entry};
use crate::storage;
use crate::crypt::{generate_salt, derive_key, encrypt_vault, decrypt_vault};

pub struct Session {
    pub vault: Vault,
    pub key: [u8; 32],
    pub salt: String,
    pub path: PathBuf,
}

impl Session {
    pub fn create(path: &str, password: &str) -> Result<Self, Box<dyn std::error::Error>> {
        println!("Creating new vault");

        let salt = generate_salt();
        let key = derive_key(&password, &salt)?;
        
        let session = Session {
            vault: Vault::new(), 
            key: key, 
            salt: salt.to_string(),
            path: path.into()
        };

        session.save()?;

        Ok(session)
    }

    pub fn unlock(path: &str, password: &str) -> Result<Self, Box<dyn std::error::Error>> {
        println!("Unlocking vault");

        let data = std::fs::read_to_string(path)?;
        let encrypted: crate::crypt::EncryptedVault = serde_json::from_str(&data)?;

        let salt = SaltString::new(&encrypted.salt)?;
        let key = derive_key(&password, &salt)?;
        let vault = decrypt_vault(encrypted, &key)?;

        println!("Vault unlocked & session created");

        Ok(Self { 
            vault, 
            key, 
            salt: salt.to_string(),
            path: path.into() })
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let encrypted = encrypt_vault(&self.vault, &self.key, &self.salt)?;
        let json = serde_json::to_string_pretty(&encrypted)?;
        std::fs::write(&self.path, json)?;
        Ok(())
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.vault.add_entry(entry);
    }

    pub fn delete_entry(&mut self, entry_name: &str) -> bool {
        self.vault.delete_entry(entry_name)
    }

    pub fn get_entries(&mut self) -> &Vec<Entry> {
        self.vault.entries()
    }
}