use crate::{
    config::Config,
    providers::{
        favicon::FaviconProviderImpl,
        feed_parser::{AtomParserImpl, RssParserImpl},
        html_processor::HtmlProcessorImpl,
        persisted_config::PersistedConfigProviderImpl,
    },
    repositories::{
        feed_content::FeedContentFsRepositoryImpl,
        persisted_config::{
            persisted_config_repository_impl::PersistedConfigFsRepositoryImpl,
            PersistedConfigRepository,
        },
    },
    router::{ARTICLES_DIR, FAVICONS_DIR},
    services::templates::TEMPLATES,
};
use std::sync::Arc;

use sqlite::ConnectionThreadSafe;

use crate::{
    repositories::feed::FeedRepositoryImpl,
    services::{
        feed::{FeedService, FeedServiceImpl},
        persisted_config::{PersistedConfigService, PersistedConfigServiceImpl},
        templates::{TemplateService, TemplateServiceImpl},
    },
};

#[derive(Clone)]
pub struct State {
    pub template_service: Arc<TemplateServiceImpl<'static, PersistedConfigProviderImpl>>,

    pub feed_service: Arc<
        FeedServiceImpl<
            FeedRepositoryImpl,
            FeedContentFsRepositoryImpl,
            HtmlProcessorImpl,
            RssParserImpl,
            AtomParserImpl,
            FaviconProviderImpl,
        >,
    >,

    pub persisted_config_service: Arc<
        PersistedConfigServiceImpl<PersistedConfigFsRepositoryImpl, PersistedConfigProviderImpl>,
    >,
}

pub trait AppState: Sync + Send + Clone + 'static {
    // Services
    type TS: TemplateService<'static>;
    type FS: FeedService;
    type PCS: PersistedConfigService;

    fn template_service(&self) -> &Self::TS;

    fn feed_service(&self) -> &Self::FS;

    fn persisted_config_service(&self) -> &Self::PCS;
}

impl State {
    pub async fn new(connection: ConnectionThreadSafe, config: Arc<Config>) -> Self {
        // Database connection
        let connection = Arc::new(connection);

        // Initialize repositories
        let feed_repository = Arc::new(FeedRepositoryImpl::new(connection.clone()));
        let feed_content_repository =
            Arc::new(FeedContentFsRepositoryImpl::new(connection, config.clone()));
        let persisted_config_repository = Arc::new(PersistedConfigFsRepositoryImpl::new(
            config.data_path.clone(),
        ));

        // Initialize providers
        let html_processor_provider =
            Arc::new(HtmlProcessorImpl::new().expect("unable to initialize html processor"));
        let rss_parser_provider = Arc::new(RssParserImpl);
        let atom_parser_provider = Arc::new(AtomParserImpl);

        let persisted_config = persisted_config_repository.load_configuration().await;
        let persisted_config_provider =
            Arc::new(PersistedConfigProviderImpl::new(persisted_config));

        // Initialize template service
        let mut template_service = TemplateServiceImpl::new(persisted_config_provider.clone());
        for (name, path) in TEMPLATES {
            template_service
                .register_template(name, format!("{}/{path}", config.static_data_path))
                .expect("there was an error registering a template");
        }
        let template_service = Arc::new(template_service);

        let feed_service = Arc::new(FeedServiceImpl::new(
            feed_repository,
            feed_content_repository,
            html_processor_provider,
            rss_parser_provider,
            atom_parser_provider,
            Arc::new(FaviconProviderImpl::new(config.clone(), FAVICONS_DIR)),
            config.clone(),
            ARTICLES_DIR,
        ));

        let persisted_config_service = Arc::new(PersistedConfigServiceImpl::new(
            persisted_config_repository,
            persisted_config_provider,
        ));

        Self {
            template_service,
            feed_service,
            persisted_config_service,
        }
    }
}

impl AppState for State {
    type TS = TemplateServiceImpl<'static, PersistedConfigProviderImpl>;
    type FS = FeedServiceImpl<
        FeedRepositoryImpl,
        FeedContentFsRepositoryImpl,
        HtmlProcessorImpl,
        RssParserImpl,
        AtomParserImpl,
        FaviconProviderImpl,
    >;
    type PCS =
        PersistedConfigServiceImpl<PersistedConfigFsRepositoryImpl, PersistedConfigProviderImpl>;

    fn template_service(&self) -> &Self::TS {
        &self.template_service
    }

    fn feed_service(&self) -> &Self::FS {
        &self.feed_service
    }

    fn persisted_config_service(&self) -> &Self::PCS {
        &self.persisted_config_service
    }
}
