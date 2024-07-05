use axum::{routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};

use crate::{controllers::rss::get_article, state::AppState};

pub fn build<S: AppState>(state: S) -> Router {
    Router::new()
        .nest_service(
            "/static",
            ServeDir::new("static").not_found_service(ServeFile::new("static/not_found.html")),
        )
        .route("/", get(get_article::<S>))
        .with_state(state)
}
