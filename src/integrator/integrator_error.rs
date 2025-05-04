use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum IntegratorError {
    SomeError,
    Other(String),
}

impl fmt::Display for IntegratorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntegratorError::SomeError => write!(f, "Some error"),
            IntegratorError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for IntegratorError {}
