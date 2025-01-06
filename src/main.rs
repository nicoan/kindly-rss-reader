use crate::repositories::init_database;
use crate::tracing::init_tracing;
use ::tracing::info;
use state::State;

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

    // Init database
    let connection = init_database();

    // Create state
    let state = State::new(connection);

    // Initialize App
    let app = router::build(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
