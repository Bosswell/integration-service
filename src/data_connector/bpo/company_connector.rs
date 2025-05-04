use std::any::type_name;

use crate::data_connector::{BpoCompatible, DataConnector};
use crate::shared::IntegrationError;
pub struct CompanyDataConnector;

impl BpoCompatible for CompanyDataConnector {}

impl DataConnector for CompanyDataConnector {
    fn name(&self) -> &str {
        type_name::<Self>()
    }

    fn supply(&self) -> Result<String, IntegrationError> {
        Ok(format!("Data from {}", self.name()))
    }

    fn trigger(&self) {
        println!("Integration triggered {}", self.name());
    }
}
