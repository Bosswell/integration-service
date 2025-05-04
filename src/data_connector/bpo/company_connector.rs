use crate::data_connector::{BpoCompatible, DataConnector, IntegrationUnit};
use crate::shared::{IntegrationContext, IntegrationError};
use crate::shared::DataSource::CEIDG;
use crate::shared::Profile::FREE;

pub struct CompanyDataConnector;

impl BpoCompatible for CompanyDataConnector {}

impl DataConnector for CompanyDataConnector {
    type Output = CompanyData;
    
    fn supply(&self, _: &IntegrationContext) -> Result<Self::Output, IntegrationError> {
        Ok(CompanyData {
            name: String::from("Example Company"),
            is_active: true,
            self_employed: false,
            started_at: Some(String::from("2024-01-01")),
            closed_at: None,
            suspended_at: None,
        })
    }

    fn get_integration_units(&self, context: &IntegrationContext) -> Vec<IntegrationUnit> {
        if (context.attempt_number > 5) {
            return vec![IntegrationUnit::DataSource(CEIDG)];
        }
        
        vec![
            IntegrationUnit::Profile(FREE),
        ]
    }
}

pub struct CompanyData {
    name: String,
    is_active: bool,
    self_employed: bool,
    started_at: Option<String>,
    closed_at: Option<String>,
    suspended_at: Option<String>,
}