use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum IntegrationError {
    Running,
    Error,
    Expired,
    Missing,
    Other(String),
}

impl fmt::Display for IntegrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntegrationError::Running => write!(f, "Integration is running"),
            IntegrationError::Error => write!(f, "Error occurred during integration"),
            IntegrationError::Expired => write!(f, "Integration is expired"),
            IntegrationError::Missing => write!(f, "Integration does not exist"),
            IntegrationError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for IntegrationError {}
