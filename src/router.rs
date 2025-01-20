use axum::{routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};

use crate::{
    config::Config,
    controllers::{add_new_feed, add_new_feed_form, get_article, get_article_list, get_feed_list},
    state::AppState,
};

pub fn build<S: AppState>(state: S, config: &Config) -> Router {
    let static_data_path = std::path::absolute(format!("{}/static/", config.static_data_path))
        .expect("invalid static data path")
        .into_os_string()
        .into_string()
        .expect("invalid static data path");
    let articles_path = std::path::absolute(format!("{}/articles/", config.static_data_path))
        .expect("invalid articles path")
        .into_os_string()
        .into_string()
        .expect("invalid articles path");

    Router::new()
        .nest_service(
            "/static",
            ServeDir::new(static_data_path)
                .not_found_service(ServeFile::new("static/not_found.html")),
        )
        .nest_service(
            "/articles",
            ServeDir::new(articles_path).not_found_service(ServeFile::new("static/not_found.html")),
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
