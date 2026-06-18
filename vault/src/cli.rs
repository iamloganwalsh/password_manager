use std::io::{self, Write};

pub fn get_choice() -> String {
    print!("Choice: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

pub fn get_password() -> String {
    print!("Master password: ");
    io::stdout().flush().unwrap();

    rpassword::read_password()
        .expect("Failed to read password")
}