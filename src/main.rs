#![allow(unused)]

mod app;

use password_manager::session::Session;
use password_manager::storage::vault_exists;
use password_manager::cli::{get_input, get_password};

fn main() {

    let path = "vault.enc";

    if !vault_exists(path) {
        println!("No vault found.");
        println!("1. Create new vault");
        println!("2. Exit");

        let choice = get_input("Choice");


        match choice.as_str() {
            "1" => {
                let password = get_password("Master Password");
                let session = Session::create(path, &password).unwrap();
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
        let password = get_password("Master Password");
        let session = Session::unlock(path, &password).unwrap();
        app::run(session);
    }

    return
}