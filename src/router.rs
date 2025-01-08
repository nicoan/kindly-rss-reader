use crate::controllers::rss::{add_new_feed, add_new_feed_form, get_article};
use axum::{routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};

use crate::{
    controllers::rss::{get_article_list, get_feed_list},
    state::AppState,
};

pub fn build<S: AppState>(state: S) -> Router {
    Router::new()
        .nest_service(
            "/static",
            ServeDir::new("static").not_found_service(ServeFile::new("static/not_found.html")),
        )
        .nest_service(
            "/articles",
            ServeDir::new("articles").not_found_service(ServeFile::new("static/not_found.html")),
        )
        .route(
            "/feed/add",
            get(add_new_feed_form::<S>).post(add_new_feed::<S>),
        )
        .route("/feed/:feed_id", get(get_article_list::<S>))
        .route("/feed/:feed_id/article/:article_id", get(get_article::<S>))
        .route("/", get(get_feed_list::<S>))
        .with_state(state)
}
