use crate::tracing::init_tracing;
use ::tracing::info;
use state::State;

mod controllers;
mod router;
pub mod services;
mod state;
mod tracing;

#[tokio::main]
async fn main() {
    // Init tracing
    init_tracing();

    // Create state
    let state = State::new();

    // Initialize App
    let app = router::build(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
