use password_manager::session::Session;
use password_manager::vault::Entry;
use std::fs;

#[test]
fn full_vault_lifecycle_works() {
    let path = "test_vault.enc";
    let test_password = "password123";

    // Clean up before test (important for repeatability)
    let _ = fs::remove_file(path);

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
    let _ = fs::remove_file(path);
}