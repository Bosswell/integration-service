use crate::data_connector::{BpoCompatible, DataConnector};
use crate::integrator::Integrator;
use crate::integrator::integrator_error::IntegratorError;

pub struct BpoIntegrator<T: DataConnector + BpoCompatible> {
    connectors: Vec<T>
}

// TODO problem z brakiem integracji Integratora
impl<T: DataConnector + BpoCompatible> Integrator BpoIntegrator<T> {
    pub fn new() -> Self {
       BpoIntegrator {
            connectors: Vec::new()
        }
    }

    fn run(&self) -> Vec<Result<String, IntegratorError>> {
        todo!()
    }

    fn add_connector(&mut self, connector: T) {
        self.connectors.push(connector);
    }
}