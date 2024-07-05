use crate::services::{
    rss::{RssService, RssServiceImpl},
    templates::{
        TemplateService, TemplateServiceImpl, TEMPLATE_NAME_ARTICLE, TEMPLATE_NAME_TOOLBAR,
        TEMPLATE_PATH_ARTICLE, TEMPLATE_PATH_TOOLBAR,
    },
};

#[derive(Clone)]
pub struct State {
    pub template_service: TemplateServiceImpl<'static>,
    pub rss_service: RssServiceImpl,
}

pub trait AppState: Sync + Send + Clone + 'static {
    type TS: TemplateService<'static>;
    type RS: RssService;

    fn template_service(&self) -> &Self::TS;

    fn rss_service(&self) -> &Self::RS;
}

impl State {
    pub fn new() -> Self {
        // Initialize template service
        let mut template_service = TemplateServiceImpl::new();
        template_service.register_template(TEMPLATE_NAME_ARTICLE, TEMPLATE_PATH_ARTICLE);
        template_service.register_template(TEMPLATE_NAME_TOOLBAR, TEMPLATE_PATH_TOOLBAR);

        Self {
            template_service,
            rss_service: RssServiceImpl::new(),
        }
    }
}

impl AppState for State {
    type TS = TemplateServiceImpl<'static>;
    type RS = RssServiceImpl;

    fn template_service(&self) -> &Self::TS {
        &self.template_service
    }

    fn rss_service(&self) -> &Self::RS {
        &self.rss_service
    }
}
