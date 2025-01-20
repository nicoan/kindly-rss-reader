use crate::{
    config::Config, providers::html_processor::HtmlProcessorImpl, services::templates::TEMPLATES,
};
use std::sync::Arc;

use sqlite::ConnectionThreadSafe;

use crate::{
    repositories::feed::FeedRepositoryImpl,
    services::{
        feed::{FeedService, FeedServiceImpl},
        templates::{TemplateService, TemplateServiceImpl},
    },
};

#[derive(Clone)]
pub struct State {
    pub template_service: TemplateServiceImpl<'static>,
    pub feed_service: FeedServiceImpl<FeedRepositoryImpl, HtmlProcessorImpl>,
}

pub trait AppState: Sync + Send + Clone + 'static {
    // Services
    type TS: TemplateService<'static>;
    type FS: FeedService;

    // Services
    fn template_service(&self) -> &Self::TS;

    fn feed_service(&self) -> &Self::FS;
}

impl State {
    pub fn new(connection: ConnectionThreadSafe, config: Arc<Config>) -> Self {
        // Database connection
        let connection = Arc::new(connection);

        // Initialize repositories
        let feed_repository = Arc::new(FeedRepositoryImpl::new(connection));

        // Initialize providers
        let html_processor_provider =
            Arc::new(HtmlProcessorImpl::new().expect("unable to initialize html processor"));

        // Initialize template service
        let mut template_service = TemplateServiceImpl::new();
        for (name, path) in TEMPLATES {
            template_service
                .register_template(name, format!("{}/{path}", config.static_data_path))
                .expect("there was an error registering a template");
        }

        Self {
            template_service,
            feed_service: FeedServiceImpl::new(feed_repository, html_processor_provider, config),
        }
    }
}

impl AppState for State {
    type TS = TemplateServiceImpl<'static>;
    type FS = FeedServiceImpl<FeedRepositoryImpl, HtmlProcessorImpl>;

    fn template_service(&self) -> &Self::TS {
        &self.template_service
    }

    fn feed_service(&self) -> &Self::FS {
        &self.feed_service
    }
}
