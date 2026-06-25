use password_manager::crypt::{
    generate_salt,
    derive_key,
    encrypt_vault,
    decrypt_vault,
};

use password_manager::vault::Vault;


#[test]
fn encrypted_vault_can_be_decrypted() {

    let vault = Vault::new();

    let salt = generate_salt();

    let key = derive_key(
        "test_password",
        &salt
    ).unwrap();


    let encrypted = encrypt_vault(
        &vault,
        &key,
        &salt.to_string()
    ).unwrap();


    let decrypted = decrypt_vault(
        encrypted,
        &key
    ).unwrap();


    assert_eq!(
        decrypted.entries.len(),
        0
    );
}

#[test]
fn wrong_key_cannot_decrypt() {

    let vault = Vault::new();

    let salt = generate_salt();

    let correct_key = derive_key(
        "correct_password",
        &salt
    ).unwrap();


    let wrong_key = derive_key(
        "wrong_password",
        &salt
    ).unwrap();


    let encrypted = encrypt_vault(
        &vault,
        &correct_key,
        &salt.to_string()
    ).unwrap();


    let result = decrypt_vault(
        encrypted,
        &wrong_key
    );


    assert!(result.is_err());
}

#[test]
fn different_encryptions_produce_different_ciphertexts() {
    let vault = Vault::new();
    let salt = generate_salt();
    let key = derive_key("password", &salt).expect("Failed to derive key");

    let encrypted1 = encrypt_vault(&vault, &key, &salt.to_string())
        .expect("Failed to encrypt vault 1");
    let encrypted2 = encrypt_vault(&vault, &key, &salt.to_string())
        .expect("Failed to encrypt vault 2");

    // Due to random nonce, ciphertexts should be different
    assert_ne!(encrypted1.ciphertext, encrypted2.ciphertext);
    assert_ne!(encrypted1.nonce, encrypted2.nonce);
}

#[test]
fn decrypt_encrypted_vault_with_entries() {
    use password_manager::vault::Entry;

    let mut vault = Vault::new();

    vault.add_entry(Entry::new(
        "Service1".to_string(),
        "user1".to_string(),
        "password1".to_string(),
        Some("https://service1.com".to_string()),
        Some("Notes for service 1".to_string()),
    ));

    vault.add_entry(Entry::new(
        "Service2".to_string(),
        "user2".to_string(),
        "password2".to_string(),
        None,
        Some("Only notes, no URL".to_string()),
    ));

    let salt = generate_salt();
    let key = derive_key("testpassword", &salt).expect("Failed to derive key");

    let encrypted = encrypt_vault(&vault, &key, &salt.to_string())
        .expect("Failed to encrypt");

    let decrypted = decrypt_vault(encrypted, &key).expect("Failed to decrypt");

    assert_eq!(decrypted.entries().len(), 2);
    assert_eq!(decrypted.entries()[0].name, "Service1");
    assert_eq!(decrypted.entries()[0].username, "user1");
    assert_eq!(decrypted.entries()[1].name, "Service2");
}

#[test]
fn key_derivation_is_deterministic() {
    let salt = generate_salt();
    let password = "consistent_password";

    let key1 = derive_key(password, &salt).expect("Failed to derive key 1");
    let key2 = derive_key(password, &salt).expect("Failed to derive key 2");

    assert_eq!(key1, key2);
}

#[test]
fn different_passwords_produce_different_keys() {
    let salt = generate_salt();

    let key1 = derive_key("password1", &salt).expect("Failed to derive key 1");
    let key2 = derive_key("password2", &salt).expect("Failed to derive key 2");

    assert_ne!(key1, key2);
}

#[test]
fn wrong_password_fails_to_decrypt() {
    use password_manager::vault::Entry;

    let mut vault = Vault::new();
    vault.add_entry(Entry::new(
        "Test".to_string(),
        "user".to_string(),
        "pass".to_string(),
        None,
        None,
    ));

    let salt = generate_salt();
    let correct_key = derive_key("correct", &salt).expect("Failed to derive correct key");
    let wrong_key = derive_key("incorrect", &salt).expect("Failed to derive wrong key");

    let encrypted = encrypt_vault(&vault, &correct_key, &salt.to_string())
        .expect("Failed to encrypt");

    let result = decrypt_vault(encrypted, &wrong_key);

    assert!(result.is_err());
}

#[test]
fn encrypted_vault_has_required_fields() {
    let vault = Vault::new();
    let salt = generate_salt();
    let key = derive_key("password", &salt).expect("Failed to derive key");

    let encrypted = encrypt_vault(&vault, &key, &salt.to_string())
        .expect("Failed to encrypt");

    assert_eq!(encrypted.version, 1);
    assert!(!encrypted.salt.is_empty());
    assert!(!encrypted.nonce.is_empty());
    assert!(!encrypted.ciphertext.is_empty());
}