use password_manager::session::Session;
use std::fs;

fn cleanup_vault_files(path: &str) {
    let _ = fs::remove_file(path);
    let _ = fs::remove_file(format!("{}.bak", path));
    let _ = fs::remove_file(format!("{}.tmp", path));
}

#[test]
fn session_with_very_short_password() {
    let path = "test_short_password.enc";
    let password = "short";

    cleanup_vault_files(path);

    // Should still create vault even with short password (validation is in UI)
    let result = Session::create(path, password);

    assert!(result.is_ok());

    cleanup_vault_files(path);
}

#[test]
fn session_with_empty_password() {
    let path = "test_empty_password.enc";
    let password = "";

    cleanup_vault_files(path);

    // Should still create vault even with empty password (validation is in UI)
    let result = Session::create(path, password);

    assert!(result.is_ok());

    cleanup_vault_files(path);
}
