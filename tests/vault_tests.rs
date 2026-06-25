use password_manager::vault::{Entry, Vault};


#[test]
fn new_vault_is_empty() {
    let vault = Vault::new();

    assert_eq!(vault.entries.len(), 0);
}


#[test]
fn adding_entry_increases_vault_size() {
    let mut vault = Vault::new();

    let entry = Entry::new(
        "Github".to_string(),
        "logan".to_string(),
        "password123".to_string(),
        None,
        None,
    );

    vault.add_entry(entry);

    assert_eq!(vault.entries.len(), 1);
}


#[test]
fn added_entry_contains_correct_data() {
    let mut vault = Vault::new();

    let entry = Entry::new(
        "Github".to_string(),
        "logan".to_string(),
        "password123".to_string(),
        Some("https://github.com".to_string()),
        Some("Personal account".to_string()),
    );

    vault.add_entry(entry);

    let stored_entry = &vault.entries[0];

    assert_eq!(stored_entry.name, "Github");
    assert_eq!(stored_entry.username, "logan");
    assert_eq!(stored_entry.url.as_ref().unwrap(), "https://github.com");
}


#[test]
fn deleting_entry_removes_entry() {
    let mut vault = Vault::new();

    let entry = Entry::new(
        "Github".to_string(),
        "logan".to_string(),
        "password123".to_string(),
        None,
        None,
    );

    vault.add_entry(entry);

    vault.delete_entry("Github");

    assert_eq!(vault.entries.len(), 0);
}

#[test]
fn vault_can_be_serialized() {

    let vault = Vault::new();

    let json = serde_json::to_string(&vault).unwrap();

    assert!(json.contains("entries"));
}

#[test]
fn entry_creation_with_all_fields() {
    let entry = Entry::new(
        "GitHub".to_string(),
        "user@example.com".to_string(),
        "securepass123".to_string(),
        Some("https://github.com".to_string()),
        Some("Personal account".to_string()),
    );

    assert_eq!(entry.name, "GitHub");
    assert_eq!(entry.username, "user@example.com");
    assert_eq!(entry.password, "securepass123");
    assert_eq!(entry.url, Some("https://github.com".to_string()));
    assert_eq!(entry.notes, Some("Personal account".to_string()));
}

#[test]
fn entry_creation_without_optional_fields() {
    let entry = Entry::new(
        "Gmail".to_string(),
        "myemail@gmail.com".to_string(),
        "pass456".to_string(),
        None,
        None,
    );

    assert_eq!(entry.name, "Gmail");
    assert_eq!(entry.username, "myemail@gmail.com");
    assert_eq!(entry.password, "pass456");
    assert_eq!(entry.url, None);
    assert_eq!(entry.notes, None);
}

#[test]
fn entry_serialization_and_deserialization() {
    let entry = Entry::new(
        "AWS".to_string(),
        "admin".to_string(),
        "awskey123".to_string(),
        Some("https://aws.amazon.com".to_string()),
        Some("Production account - use with caution".to_string()),
    );

    let json = serde_json::to_string(&entry).expect("Failed to serialize");
    let deserialized: Entry = serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(deserialized.name, entry.name);
    assert_eq!(deserialized.username, entry.username);
    assert_eq!(deserialized.password, entry.password);
    assert_eq!(deserialized.url, entry.url);
    assert_eq!(deserialized.notes, entry.notes);
}

#[test]
fn vault_add_multiple_entries() {
    let mut vault = Vault::new();

    for i in 0..5 {
        let entry = Entry::new(
            format!("Service{}", i),
            format!("user{}", i),
            format!("pass{}", i),
            None,
            None,
        );
        vault.add_entry(entry);
    }

    assert_eq!(vault.entries().len(), 5);
}

#[test]
fn vault_delete_entry_returns_true_on_success() {
    let mut vault = Vault::new();

    let entry = Entry::new(
        "TestService".to_string(),
        "user".to_string(),
        "pass".to_string(),
        None,
        None,
    );

    vault.add_entry(entry);
    let result = vault.delete_entry("TestService");

    assert!(result);
    assert_eq!(vault.entries().len(), 0);
}

#[test]
fn vault_delete_entry_returns_false_on_not_found() {
    let mut vault = Vault::new();

    let entry = Entry::new(
        "ExistingService".to_string(),
        "user".to_string(),
        "pass".to_string(),
        None,
        None,
    );

    vault.add_entry(entry);
    let result = vault.delete_entry("NonexistentService");

    assert!(!result);
    assert_eq!(vault.entries().len(), 1); // Entry should still be there
}

#[test]
fn vault_delete_specific_entry_from_multiple() {
    let mut vault = Vault::new();

    vault.add_entry(Entry::new(
        "GitHub".to_string(),
        "user1".to_string(),
        "pass1".to_string(),
        None,
        None,
    ));

    vault.add_entry(Entry::new(
        "GitLab".to_string(),
        "user2".to_string(),
        "pass2".to_string(),
        None,
        None,
    ));

    vault.add_entry(Entry::new(
        "Bitbucket".to_string(),
        "user3".to_string(),
        "pass3".to_string(),
        None,
        None,
    ));

    vault.delete_entry("GitLab");

    assert_eq!(vault.entries().len(), 2);
    assert_eq!(vault.entries()[0].name, "GitHub");
    assert_eq!(vault.entries()[1].name, "Bitbucket");
}

#[test]
fn vault_with_empty_optional_fields_serializes() {
    let mut vault = Vault::new();

    vault.add_entry(Entry::new(
        "Service1".to_string(),
        "user".to_string(),
        "pass".to_string(),
        None,
        None,
    ));

    vault.add_entry(Entry::new(
        "Service2".to_string(),
        "user".to_string(),
        "pass".to_string(),
        Some("https://example.com".to_string()),
        None,
    ));

    let json = serde_json::to_string(&vault).expect("Failed to serialize");
    let _deserialized: Vault = serde_json::from_str(&json).expect("Failed to deserialize");

    // If we got here without panicking, serialization/deserialization worked
    assert!(true);
}