use std::collections::HashSet;
use crate::shared::{DataSource, Nip};

#[derive(Debug, Default)]
pub struct IntegrationContext {
    pub nip: Option<Nip>,
    pub attempt_number: u32,
    pub triggered_sources: HashSet<DataSource>,
}
