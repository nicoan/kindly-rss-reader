use crate::services::templates::{TemplateService, TEMPLATE_NAME_FEED_ADD};
use crate::state::AppState;
use axum::{extract::State, response::Html};
use minijinja::context;

pub async fn add_new_feed_form<S>(State(state): State<S>) -> Html<String>
where
    S: AppState,
{
    Html(
        state
            .template_service()
            .render_template(TEMPLATE_NAME_FEED_ADD, context! {})
            .unwrap_or(
                "<h1> There was an error rendering the add feed form. Please check the logs. </h1>"
                    .to_owned(),
            ),
    )
}
