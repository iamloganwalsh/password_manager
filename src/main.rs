#![allow(unused)]

mod app;

use password_manager::session::Session;
use password_manager::cli::{get_input, get_password};
use std::path::Path;

fn main() {

    let path = "vault.enc";

    if !Path::new(path).exists() {
        println!("No vault found.");
        println!("1. Create new vault");
        println!("2. Exit");

        match get_input("Choice").as_str() {
            "1" => {
                let password = loop {
                    let pwd = get_password("Master Password");
                    if pwd.len() < 8 {
                        println!("Password must be at least 8 characters.");
                        continue;
                    }
                    let confirm = get_password("Confirm Password");
                    if pwd == confirm {
                        break pwd;
                    }
                    println!("Passwords don't match.");
                };

                match Session::create(path, &password) {
                    Ok(session) => app::run(session),
                    Err(e) => println!("Error: {}", e),
                }
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
        match Session::unlock(path, &password) {
            Ok(session) => app::run(session),
            Err(e) => println!("Error unlocking vault: {}", e),
        }
    }

    return
}