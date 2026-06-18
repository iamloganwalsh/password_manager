use crate::session::Session;
use crate::cli::get_choice;

pub fn run(mut session: Session) {

    loop {
        println!();
        println!("==== Password Manager ====");
        println!("1. List entries");
        println!("2. Add entry");
        println!("3. Remove entry");
        println!("4. Save");
        println!("5. Exit");

        let choice = get_choice();

        match choice.as_str() {
            "1" => {
                println!("Listing entries...");
            }

            "2" => {
                println!("Adding entry...");
            }

            "3" => {
                println!("Removing entry...");
            }

            "4" => {
                match session.save() {
                    Ok(_) => println!("Vault saved."),
                    Err(e) => println!("Save failed: {e}"),
                }
            }

            "5" => {
                println!("Saving before exit...");

                match session.save() {
                    Ok(_) => println!("Saved."),
                    Err(e) => println!("Save failed: {e}"),
                }

                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Invalid option");
            }
        }
    }
}