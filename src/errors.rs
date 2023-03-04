use std::{fmt::Display, error::Error};

#[derive(Debug)]
pub struct BankAccountError(String);

impl Display for BankAccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


impl Error for BankAccountError {}

impl From<&str> for BankAccountError {
    fn from(value: &str) -> Self {
        BankAccountError(value.to_string())
    }
}

pub struct AtmError;
pub struct CheckingError;