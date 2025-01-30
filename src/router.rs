use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::{ServeDir, ServeFile};

use crate::{
    config::Config,
    controllers::{
        config::{set_dark_theme, set_zoom},
        feed::{add_new_feed, add_new_feed_form, get_article, get_article_list, get_feed_list},
    },
    state::AppState,
};

pub const STATIC_DIR: &str = "/static";
pub const ARTICLES_DIR: &str = "/articles";

pub fn build<S: AppState>(state: S, config: &Config) -> Router {
    let static_data_path = std::path::absolute(format!("{}/static/", config.static_data_path))
        .expect("invalid static data path")
        .into_os_string()
        .into_string()
        .expect("invalid static data path");
    let articles_path = std::path::absolute(format!("{}/articles/", config.data_path))
        .expect("invalid articles path")
        .into_os_string()
        .into_string()
        .expect("invalid articles path");

    let not_found = format!("{static_data_path}/not_found.html");

    Router::new()
        .nest_service(
            STATIC_DIR,
            ServeDir::new(&static_data_path).not_found_service(ServeFile::new(&not_found)),
        )
        .nest_service(
            ARTICLES_DIR,
            ServeDir::new(articles_path).not_found_service(ServeFile::new(&not_found)),
        )
        .route(
            "/feed/add",
            get(add_new_feed_form::<S>).post(add_new_feed::<S>),
        )
        .route("/feed/:feed_id", get(get_article_list::<S>))
        .route("/feed/:feed_id/article/:article_id", get(get_article::<S>))
        .route("/config/dark_theme", post(set_dark_theme::<S>))
        .route("/config/zoom", post(set_zoom::<S>))
        .route("/", get(get_feed_list::<S>))
        .with_state(state)
}
