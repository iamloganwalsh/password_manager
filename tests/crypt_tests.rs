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