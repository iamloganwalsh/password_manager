mod vault;
mod storage;
mod crypt;

use vault::{Vault, Entry};
use storage::{read_vault, write_vault};

fn main() {
    let path = "vault.enc";
    let password = "test_password";

    let mut vault = read_vault(path, password).unwrap_or_else(|_| {
        println!("Creating new vault...");
        Vault::new()
    });

    let entry = Entry::new(
        "GitHub".to_string(),
        "logan".to_string(),
        "password123".to_string(),
        Some("https://github.com".to_string()),
        None,
    );

    vault.add_entry(entry);

    vault.list_entries();

    write_vault(path, &vault, password)
        .expect("Failed to save vault");
}