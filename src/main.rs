use std::sync::Arc;

use crate::repositories::init_database;
use crate::tracing::init_tracing;
use config::Config;
use state::State;

mod config;
mod controllers;
mod models;
pub mod providers;
mod repositories;
mod router;
pub mod services;
mod state;
mod tracing;

#[tokio::main]
async fn main() {
    // Init tracing
    init_tracing();

    // Configuration
    let config = Arc::new(Config::load());

    // Init database
    let connection = init_database(&config);

    // Create state
    let state = State::new(connection, config.clone());

    // Initialize App
    let app = router::build(state, &config);
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.ip, config.port))
        .await
        .expect("unable to bind tcp listener");

    axum::serve(listener, app).await.unwrap();

    config.print_information();
}
