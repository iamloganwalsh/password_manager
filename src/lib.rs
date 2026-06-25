// src/lib.rs

pub mod error;
pub mod vault;
pub mod session;
pub mod crypt;
pub mod cli;

pub use error::PasswordManagerError;