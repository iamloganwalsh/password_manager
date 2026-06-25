use argon2::password_hash::SaltString;
use std::path::PathBuf;
use crate::vault::{Vault, Entry};
use std::io::Write;
use crate::crypt::{generate_salt, derive_key, encrypt_vault, decrypt_vault};
use crate::error::PasswordManagerError;

pub struct Session {
    vault: Vault,
    key: [u8; 32],
    salt: String,
    path: PathBuf,
}

impl Session {
    pub fn create(path: &str, password: &str) -> Result<Self, PasswordManagerError> {
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

    pub fn unlock(path: &str, password: &str) -> Result<Self, PasswordManagerError> {
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

    // Rewritten to prevent corrupted files and losing data by using a temporary file
    pub fn save(&self) -> Result<(), PasswordManagerError> {
        let encrypted = encrypt_vault(&self.vault, &self.key, &self.salt)?;
        let json = serde_json::to_string_pretty(&encrypted)?;
        

        let main_path = &self.path;
        let backup_path = self.path.with_extension("enc.bak");
        let tmp_path = self.path.with_extension("enc.tmp");

        // Rotate main to backup
        if main_path.exists() {
            let _ = std::fs::remove_file(&backup_path);
            std::fs::rename(main_path, backup_path);
        }

        // Create temp file to store new data
        {
            let mut file = std::fs::File::create(&tmp_path)?;
            file.write_all(json.as_bytes())?;
            file.sync_all()?;
        }

        // Promote temp file to main
        std::fs::rename(tmp_path, main_path)?;

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