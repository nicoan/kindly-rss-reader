use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use axum::{
    body::Body,
    http::{Request, Response},
    response::IntoResponse,
};
use minijinja::context;
use reqwest::StatusCode;
use tower::{Layer, Service};

use crate::{
    controllers::HtmlResponse,
    services::templates::{TemplateService, TEMPLATE_NAME_ERROR},
    state::AppState,
    view_models::error::Error,
};

// Middleware Layer
#[derive(Clone)]
pub struct ErrorHandlingLayer<AS: AppState> {
    state: AS,
}

impl<AS: AppState> ErrorHandlingLayer<AS> {
    pub fn new(state: AS) -> Self {
        Self { state }
    }
}

impl<S, AS: AppState> Layer<S> for ErrorHandlingLayer<AS> {
    type Service = ErrorHandlingMiddleware<S, AS>;

    fn layer(&self, inner: S) -> Self::Service {
        ErrorHandlingMiddleware {
            inner,
            state: self.state.clone(),
        }
    }
}

#[derive(Clone)]
pub struct ErrorHandlingMiddleware<S, AS: AppState> {
    inner: S,
    state: AS,
}

async fn get_error(error: Error, state: impl AppState) -> HtmlResponse {
    let rendered_html = state
        .template_service()
        .render_template(TEMPLATE_NAME_ERROR, context! { error => error})
        .await;

    match rendered_html {
        Ok(rendered_html) => {
            HtmlResponse::new(rendered_html).with_status_code(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Err(e) => {
            tracing::error!("an unexpected error ocurred rendering an error: {e:?}");
            HtmlResponse::new("Something went horribly wrong. There was an error trying to render the error page. Please check the logs".to_owned()).with_status_code(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

impl<S, ReqBody, AS: AppState> Service<Request<ReqBody>> for ErrorHandlingMiddleware<S, AS>
where
    S: Service<Request<ReqBody>, Response = Response<Body>, Error = Infallible>
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
    ReqBody: std::marker::Send + 'static,
{
    type Response = Response<Body>;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let mut inner = self.inner.clone();
        let state = self.state.clone();

        Box::pin(async move {
            match inner.call(req).await {
                Ok(response) => match response.status().as_u16() {
                    s if (500..=599).contains(&s) => {
                        tracing::error!("an unexpected error ocurred: {response:?}");
                        let error = Error::internal_error();
                        Ok(get_error(error, state).await.into_response())
                    }
                    404 => {
                        let error = Error::not_found();
                        Ok(get_error(error, state).await.into_response())
                    }
                    400 => {
                        let error = Error::bad_request();
                        Ok(get_error(error, state).await.into_response())
                    }
                    _ => Ok(response),
                },
                Err(err) => {
                    tracing::error!("an unexpected error ocurred: {err:?}");
                    let error = Error::internal_error();
                    Ok(get_error(error, state).await.into_response())
                }
            }
        })
    }
}
