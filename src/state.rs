use crate::{
    providers::html_processor::HtmlProcessorImpl,
    services::templates::{TEMPLATE_NAME_FEED_ADD, TEMPLATE_PATH_FEED_ADD},
};
use std::sync::Arc;

use sqlite::ConnectionThreadSafe;

use crate::{
    repositories::feed::FeedRepositoryImpl,
    services::{
        feed::{FeedService, FeedServiceImpl},
        templates::{
            TemplateService, TemplateServiceImpl, TEMPLATE_NAME_ARTICLE,
            TEMPLATE_NAME_ARTICLE_LIST, TEMPLATE_NAME_COMMON_HEAD, TEMPLATE_NAME_FEED_LIST,
            TEMPLATE_NAME_TOOLBAR, TEMPLATE_PATH_ARTICLE, TEMPLATE_PATH_ARTICLE_LIST,
            TEMPLATE_PATH_COMMON_HEAD, TEMPLATE_PATH_FEED_LIST, TEMPLATE_PATH_TOOLBAR,
        },
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
    pub fn new(connection: ConnectionThreadSafe) -> Self {
        // Database connection
        let connection = Arc::new(connection);

        // Initialize repositories
        let feed_repository = Arc::new(FeedRepositoryImpl::new(connection));

        // Initialize providers
        let html_processor_provider =
            HtmlProcessorImpl::new().expect("unable to initialize html processor");

        // Initialize template service
        let mut template_service = TemplateServiceImpl::new();
        template_service.register_template(TEMPLATE_NAME_ARTICLE, TEMPLATE_PATH_ARTICLE);
        template_service.register_template(TEMPLATE_NAME_ARTICLE_LIST, TEMPLATE_PATH_ARTICLE_LIST);
        template_service.register_template(TEMPLATE_NAME_COMMON_HEAD, TEMPLATE_PATH_COMMON_HEAD);
        template_service.register_template(TEMPLATE_NAME_FEED_ADD, TEMPLATE_PATH_FEED_ADD);
        template_service.register_template(TEMPLATE_NAME_FEED_LIST, TEMPLATE_PATH_FEED_LIST);
        template_service.register_template(TEMPLATE_NAME_TOOLBAR, TEMPLATE_PATH_TOOLBAR);

        Self {
            template_service,
            feed_service: FeedServiceImpl::new(feed_repository, Arc::new(html_processor_provider)),
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
