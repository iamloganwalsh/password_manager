use password_manager::session::Session;
use password_manager::vault::Entry;
use std::fs;

fn cleanup_vault_files(path: &str) {
    let _ = fs::remove_file(path);
    let _ = fs::remove_file(format!("{}.bak", path));
    let _ = fs::remove_file(format!("{}.tmp", path));
}

#[test]
fn corrupted_vault_file_fails_to_unlock() {
    let path = "test_corrupted.enc";

    cleanup_vault_files(path);

    // Create a file with invalid JSON
    fs::write(path, "{ invalid json }").expect("Failed to write test file");

    let result = Session::unlock(path, "password");

    assert!(result.is_err());

    cleanup_vault_files(path);
}

#[test]
fn vault_entries_maintain_order() {
    let path = "test_entry_order.enc";
    let password = "order_test";

    cleanup_vault_files(path);

    let mut session = Session::create(path, password).expect("Failed to create session");

    for i in 0..10 {
        session.add_entry(Entry::new(
            format!("Service{}", i),
            format!("user{}", i),
            format!("pass{}", i),
            None,
            None,
        ));
    }

    session.save().expect("Failed to save");
    drop(session);

    let mut reopened = Session::unlock(path, password).expect("Failed to unlock");
    let entries = reopened.get_entries();

    for i in 0..10 {
        assert_eq!(entries[i].name, format!("Service{}", i));
    }

    cleanup_vault_files(path);
}

#[test]
fn entry_with_unicode_characters_persists() {
    let path = "test_unicode.enc";
    let password = "unicode_test";

    cleanup_vault_files(path);

    let mut session = Session::create(path, password).expect("Failed to create session");

    session.add_entry(Entry::new(
        "Café".to_string(),
        "用户".to_string(),
        "🔐secure".to_string(),
        Some("https://例え.jp".to_string()),
        Some("Ελληνικά σημειώσεις".to_string()),
    ));

    session.save().expect("Failed to save");
    drop(session);

    let mut reopened = Session::unlock(path, password).expect("Failed to unlock");
    let entries = reopened.get_entries();

    assert_eq!(entries[0].name, "Café");
    assert_eq!(entries[0].username, "用户");
    assert_eq!(entries[0].password, "🔐secure");
    assert_eq!(entries[0].notes.as_ref().unwrap(), "Ελληνικά σημειώσεις");

    cleanup_vault_files(path);
}
