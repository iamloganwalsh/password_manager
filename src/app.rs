use password_manager::session::Session;
use password_manager::cli::{get_input, get_password};
use password_manager::vault::Entry;

pub fn run(mut session: Session) {

    loop {
        println!();
        println!("==== Password Manager ====");
        println!("1. List entries");
        println!("2. Add entry");
        println!("3. Save");
        println!("4. Exit");
        println!("9. Remove entry");

        let choice = get_input("Choice");

        match choice.as_str() {
            "1" => {
                let vault_entries = session.get_entries();

                for (i, entry) in vault_entries.iter().enumerate() {
                    println!(
                        "{}: {} ({})",
                        i,
                        entry.name,
                        entry.username
                    );
                }
            }

            "2" => {
                let name = get_input("Name");
                let username = get_input("Username");
                let password = get_password("Password");
                let url = get_input("URL (optional)");
                let notes = get_input("Notes (optional)");

                let entry = Entry::new(
                    name,
                    username,
                    password,
                    optional_string(url),
                    optional_string(notes),
                );

                session.add_entry(entry);

                println!("Entry added.");
            }

            "3" => {
                match session.save() {
                    Ok(_) => println!("Vault saved."),
                    Err(e) => println!("Save failed: {e}"),
                }
            }

            "4" => {
                println!("Saving before exit...");

                match session.save() {
                    Ok(_) => println!("Saved."),
                    Err(e) => println!("Save failed: {e}"),
                }

                println!("Goodbye!");
                break;
            }

            "9" => {
                let name = get_input("Name to delete: ");
                if !session.delete_entry(&name) {
                    println!("Couldn't find entry.");
                } else {
                    println!("Deleted entry {}", name);
                }
               
            }

            _ => {
                println!("Invalid option");
            }
        }
    }
}

fn optional_string(value: String) -> Option<String> {
    if value.trim().is_empty() {
        None
    } else {
        Some(value)
    }
}