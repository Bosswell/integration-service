pub mod bpo;

use crate::shared::IntegrationError;

pub trait DataConnector {
    fn name(&self) -> &str;
    fn supply(&self) -> Result<String, IntegrationError>;
    fn trigger(&self);
}

pub trait BpoCompatible {}