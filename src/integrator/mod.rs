use crate::data_connector::DataConnector;
use crate::integrator::integrator_error::IntegratorError;

pub mod bpo_integrator;
mod integrator_error;

pub use crate::integrator::bpo_integrator::BpoIntegrator;

trait Integrator {
    fn new() -> Self;

    fn run(&self) -> Vec<Result<String, IntegratorError>>;

    fn add_connector(&mut self, service: Box<dyn DataConnector>);
}