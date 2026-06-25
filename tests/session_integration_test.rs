use password_manager::session::Session;
use password_manager::vault::Entry;
use std::fs;

fn cleanup_vault_files(path: &str) {
    let _ = fs::remove_file(path);
    let _ = fs::remove_file(format!("{}.bak", path));
    let _ = fs::remove_file(format!("{}.tmp", path));
}

#[test]
fn full_vault_lifecycle_works() {
    let path = "test_vault.enc";
    let test_password = "password123";

    // Clean up before test (important for repeatability)
    cleanup_vault_files(path);

    // ----------------------------
    // 1. Create vault
    // ----------------------------
    let mut session = Session::create(path, test_password)
        .expect("Failed to create vault");

    // Add entry
    let entry = Entry::new(
        "GitHub".to_string(),
        "logan".to_string(),
        "password123".to_string(),
        Some("https://github.com".to_string()),
        Some("test account".to_string()),
    );

    session.add_entry(entry);

    // Save vault
    session.save().expect("Failed to save vault");

    // Drop session (simulate closing app)
    drop(session);

    // ----------------------------
    // 2. Reopen vault
    // ----------------------------
    let mut session = Session::unlock(path, test_password)
        .expect("Failed to unlock vault");

    // ----------------------------
    // 3. Verify data persisted
    // ----------------------------
    let entries = session.get_entries();

    assert_eq!(entries.len(), 1);

    let entry = &entries[0];

    assert_eq!(entry.name, "GitHub");
    assert_eq!(entry.username, "logan");
    assert_eq!(entry.password, "password123");

    assert_eq!(
        entry.url.as_ref().unwrap(),
        "https://github.com"
    );

    assert_eq!(
        entry.notes.as_ref().unwrap(),
        "test account"
    );

    // Clean up after test
    cleanup_vault_files(path);
}

#[test]
fn session_create_and_unlock() {
    use std::path::Path;
    
    let path = "test_session_create_unlock.enc";
    let password = "test_password_123";

    // Clean up before test
    cleanup_vault_files(path);

    // Create session
    let session = Session::create(path, password).expect("Failed to create session");
    drop(session);

    // Verify file exists
    assert!(Path::new(path).exists());

    // Unlock session
    let _session = Session::unlock(path, password).expect("Failed to unlock session");

    // Clean up
    cleanup_vault_files(path);
}

#[test]
fn session_unlock_with_wrong_password_fails() {
    let path = "test_session_wrong_password.enc";
    let correct_password = "correct_password";
    let wrong_password = "wrong_password";

    // Clean up before test
    cleanup_vault_files(path);

    // Create session with correct password
    Session::create(path, correct_password).expect("Failed to create session");

    // Try to unlock with wrong password
    let result = Session::unlock(path, wrong_password);

    assert!(result.is_err());

    // Clean up
    cleanup_vault_files(path);
}

#[test]
fn session_add_and_retrieve_entries() {
    let path = "test_session_add_retrieve.enc";
    let password = "session_test";

    cleanup_vault_files(path);

    let mut session = Session::create(path, password).expect("Failed to create session");

    let entry1 = Entry::new(
        "GitHub".to_string(),
        "user1".to_string(),
        "pass1".to_string(),
        None,
        None,
    );

    let entry2 = Entry::new(
        "GitLab".to_string(),
        "user2".to_string(),
        "pass2".to_string(),
        Some("https://gitlab.com".to_string()),
        None,
    );

    session.add_entry(entry1);
    session.add_entry(entry2);

    session.save().expect("Failed to save session");
    drop(session);

    // Reopen and verify
    let mut reopened = Session::unlock(path, password).expect("Failed to unlock");
    let entries = reopened.get_entries();

    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].name, "GitHub");
    assert_eq!(entries[1].name, "GitLab");

    cleanup_vault_files(path);
}

#[test]
fn session_delete_entry() {
    let path = "test_session_delete.enc";
    let password = "session_test";

    cleanup_vault_files(path);

    let mut session = Session::create(path, password).expect("Failed to create session");

    session.add_entry(Entry::new(
        "Service1".to_string(),
        "user".to_string(),
        "pass".to_string(),
        None,
        None,
    ));

    session.add_entry(Entry::new(
        "Service2".to_string(),
        "user".to_string(),
        "pass".to_string(),
        None,
        None,
    ));

    let deleted = session.delete_entry("Service1");

    assert!(deleted);
    assert_eq!(session.get_entries().len(), 1);
    assert_eq!(session.get_entries()[0].name, "Service2");

    session.save().expect("Failed to save");

    cleanup_vault_files(path);
}

#[test]
fn session_delete_nonexistent_entry_returns_false() {
    let path = "test_session_delete_nonexistent.enc";
    let password = "session_test";

    cleanup_vault_files(path);

    let mut session = Session::create(path, password).expect("Failed to create session");

    session.add_entry(Entry::new(
        "Service1".to_string(),
        "user".to_string(),
        "pass".to_string(),
        None,
        None,
    ));

    let deleted = session.delete_entry("NonexistentService");

    assert!(!deleted);
    assert_eq!(session.get_entries().len(), 1); // Entry should still be there

    cleanup_vault_files(path);
}

#[test]
fn session_save_creates_vault_file() {
    use password_manager::crypt::EncryptedVault;
    use std::path::Path;

    let path = "test_session_save.enc";
    let password = "session_test";

    cleanup_vault_files(path);

    let mut session = Session::create(path, password).expect("Failed to create session");

    session.add_entry(Entry::new(
        "Service".to_string(),
        "user".to_string(),
        "pass".to_string(),
        None,
        None,
    ));

    session.save().expect("Failed to save");

    assert!(Path::new(path).exists());

    // Verify file contains valid JSON
    let content = fs::read_to_string(path).expect("Failed to read file");
    let _: EncryptedVault = serde_json::from_str(&content).expect("Invalid JSON in vault file");

    cleanup_vault_files(path);
}

#[test]
fn session_multiple_saves_preserve_data() {
    let path = "test_session_multiple_saves.enc";
    let password = "session_test";

    cleanup_vault_files(path);

    let mut session = Session::create(path, password).expect("Failed to create session");

    session.add_entry(Entry::new(
        "Service1".to_string(),
        "user1".to_string(),
        "pass1".to_string(),
        None,
        None,
    ));

    session.save().expect("Failed to save 1");

    session.add_entry(Entry::new(
        "Service2".to_string(),
        "user2".to_string(),
        "pass2".to_string(),
        None,
        None,
    ));

    session.save().expect("Failed to save 2");

    drop(session);

    // Reopen and verify both entries persist
    let mut reopened = Session::unlock(path, password).expect("Failed to unlock");
    let entries = reopened.get_entries();

    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].name, "Service1");
    assert_eq!(entries[1].name, "Service2");

    cleanup_vault_files(path);
}

#[test]
fn session_with_special_characters_in_password() {
    let path = "test_session_special_chars.enc";
    let password = "p@$$w0rd!#%&*()_+-=[]{}|;:,.<>?";

    cleanup_vault_files(path);

    let mut session = Session::create(path, password).expect("Failed to create session");

    session.add_entry(Entry::new(
        "Special".to_string(),
        "user".to_string(),
        "pass".to_string(),
        None,
        None,
    ));

    session.save().expect("Failed to save");
    drop(session);

    let mut reopened = Session::unlock(path, password).expect("Failed to unlock");
    let entries = reopened.get_entries();

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].name, "Special");

    cleanup_vault_files(path);
}

#[test]
fn session_with_long_password() {
    let path = "test_session_long_password.enc";
    let password = &"a".repeat(256); // 256-character password

    cleanup_vault_files(path);

    let mut session = Session::create(path, password).expect("Failed to create session");

    session.add_entry(Entry::new(
        "LongPass".to_string(),
        "user".to_string(),
        "pass".to_string(),
        None,
        None,
    ));

    session.save().expect("Failed to save");
    drop(session);

    let mut reopened = Session::unlock(path, password).expect("Failed to unlock");
    let entries = reopened.get_entries();

    assert_eq!(entries.len(), 1);

    cleanup_vault_files(path);
}