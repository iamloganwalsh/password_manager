use dialoguer::{Input, Password};

pub fn get_input(prompt: &str) -> String {
    Input::new()
        .with_prompt(prompt)
        .allow_empty(true)
        .interact_text()
        .expect("Failed to read input")
}

pub fn get_password(prompt: &str) -> String {
    Password::new()
        .with_prompt(prompt)
        .interact()
        .expect("Failed to read password")
}