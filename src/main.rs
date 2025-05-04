mod data_connector;
mod shared;
mod integrator;

use std::sync::Arc;
use sqlx::{PgPool, postgres::PgPoolOptions};
use crate::data_connector::bpo::company_connector;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<PgPool>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:pass@localhost/db")
        .await?;

    company_connector::do_something(&pool).await?;
    
    Ok(())
}