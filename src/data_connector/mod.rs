pub mod bpo;

use std::any::{type_name, TypeId};
use crate::shared::{DataSource, IntegrationContext, IntegrationError, Profile};

pub trait DataConnector {
    type Output;

    fn name(&self) -> &str {
        type_name::<Self>()
    }

    fn dependencies(&self) -> Vec<TypeId> {
        vec![]
    }

    fn supply(&self, integration: &IntegrationContext) -> Result<Self::Output, IntegrationError>;
    fn get_integration_units(&self, integration: &IntegrationContext) -> Vec<IntegrationUnit>;
}

#[derive(Debug, Clone)]
pub enum IntegrationUnit {
    DataSource(DataSource),
    Profile(Profile),
}

pub trait BpoCompatible {}