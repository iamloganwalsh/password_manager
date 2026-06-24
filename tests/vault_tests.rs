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