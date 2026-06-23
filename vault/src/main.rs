mod app;
mod cli;
mod crypt;
mod session;
mod storage;
mod vault;

use session::Session;
use storage::vault_exists;
use cli::get_input;

fn main() {
    let path = "vault.enc";

    if !vault_exists(path) {
        println!("No vault found.");
        println!("1. Create new vault");
        println!("2. Exit");

        let choice = get_input("Choice: ");

        match choice.as_str() {
            "1" => {
                let session = Session::create(path).unwrap();
                app::run(session);
            }

            "2" => {
                println!("Goodbye!");
            }

            _ => {
                println!("Invalid option");
            }
        }
        
    } else {
        let session = Session::unlock(path).unwrap();
        app::run(session);
    }

    return
}