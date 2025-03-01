use super::error::FeedServiceError;
use super::FeedService;
use super::Result;
use crate::config::Config;
use crate::models::article::Article;
use crate::providers::feed_parser::FeedParser;
use crate::providers::feed_parser::FeedParserImpl;
use crate::providers::html_processor::HtmlProcessor;
use crate::providers::image_processor::ImageProcessor;
use crate::providers::image_processor::ImageProcessorFsImpl;
use crate::repositories::feed_content::FeedContentRepository;
use crate::{models::feed::Feed, repositories::feed::FeedRepository};
use axum::async_trait;
use axum::body::Bytes;
use chrono::DateTime;
use chrono::TimeDelta;
use chrono::Utc;
use reqwest::Url;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::task::JoinSet;
use uuid::Uuid;

type ArticleContent = String;

pub struct FeedServiceImpl<FR, FCR, HP, FP>
where
    FR: FeedRepository,
    FCR: FeedContentRepository,
    HP: HtmlProcessor + 'static,
    FP: FeedParser + 'static,
{
    feed_repository: Arc<FR>,
    feed_content_repository: Arc<FCR>,
    html_processor: Arc<HP>,
    feed_parser: Arc<FP>,
    config: Arc<Config>,
    articles_router_path: &'static str,
}

impl<FR, FCR, HP, FP> FeedServiceImpl<FR, FCR, HP, FP>
where
    FR: FeedRepository,
    FCR: FeedContentRepository,
    HP: HtmlProcessor + 'static,
    FP: FeedParser + 'static,
{
    pub fn new(
        feed_repository: Arc<FR>,
        feed_content_repository: Arc<FCR>,
        html_processor: Arc<HP>,
        feed_parser: Arc<FP>,
        config: Arc<Config>,
        articles_router_path: &'static str,
    ) -> Self {
        Self {
            feed_repository,
            feed_content_repository,
            html_processor,
            feed_parser,
            config,
            articles_router_path,
        }
    }

    async fn download_feed_content(feed_url: &str) -> Result<Bytes> {
        reqwest::get(feed_url)
            .await
            .map_err(FeedServiceError::GettingFeed)?
            .bytes()
            .await
            .map_err(|e| FeedServiceError::Unexpected(e.into()))
    }

    async fn download_html_article(article_url: &str) -> Result<String> {
        reqwest::get(article_url)
            .await
            .map_err(FeedServiceError::GettingArticle)?
            .text()
            .await
            .map_err(|e| FeedServiceError::Unexpected(e.into()))
    }

    async fn process_html_content(
        content: &str,
        html_processor: Arc<HP>,
        image_processor: Arc<impl ImageProcessor>,
        feed_link: Arc<String>,
    ) -> Result<String> {
        let content = html_processor
            .process_html_article(content)
            .map_err(|e| FeedServiceError::Unexpected(anyhow::anyhow!(e)))?
            .to_owned();

        // Fix img src in contents
        let content = html_processor
            .fix_img_src(&content, &feed_link, &*image_processor)
            .await
            .map_err(|e| FeedServiceError::Unexpected(e.into()))?;

        html_processor
            .sanitize(&content)
            .map_err(|e| FeedServiceError::Unexpected(e.into()))
    }

    async fn process_rss_content(
        content: &str,
        html_processor: Arc<HP>,
        image_processor: Arc<impl ImageProcessor>,
        feed_link: Arc<String>,
    ) -> Result<String> {
        // Fix img src in contents
        let content = html_processor
            .fix_img_src(content, &feed_link, &*image_processor)
            .await
            .map_err(|e| FeedServiceError::Unexpected(e.into()))?;

        html_processor
            .sanitize(&content)
            .map_err(|e| FeedServiceError::Unexpected(e.into()))
    }


    // TODO: File paths are not responsibility of this service, they should be managed by
    // ImageProcessorFsImpl and FeedContentFsRepositoryImpl
    fn get_article_router_path(&self, feed_id: Uuid) -> String {
        format!("{}/{feed_id}", self.articles_router_path)
    }

    // TODO: File paths are not responsibility of this service, they should be managed by
    // ImageProcessorFsImpl and FeedContentFsRepositoryImpl
    fn get_article_file_path(&self, feed_id: Uuid) -> String {
        format!("{}/articles/{feed_id}", self.config.data_path)
    }
}

#[async_trait]
impl<FR, FCR, HP, FP> FeedService for FeedServiceImpl<FR, FCR, HP, FP>
where
    FR: FeedRepository + 'static,
    FCR: FeedContentRepository + 'static,
    HP: HtmlProcessor,
    FP: FeedParser,
{
    async fn get_feed_list(&self) -> Result<Vec<Feed>> {
        Ok(self.feed_repository.get_feed_list().await?)
    }

    async fn get_feed(&self, feed_id: Uuid) -> Result<Option<Feed>> {
        Ok(self.feed_repository.get_feed(feed_id).await?)
    }

    async fn add_feed(&self, feed_url: Url) -> Result<()> {
        let content = Self::download_feed_content(feed_url.as_str()).await?;
        
        let feed = self.feed_parser.parse_feed(feed_url.as_str(), &content)
            .await
            .map_err(|e| FeedServiceError::Unexpected(e.into()))?;
            
        self.feed_repository.add_feed(feed).await?;

        Ok(())
    }

    async fn get_channel(&self, feed_id: Uuid) -> Result<(Feed, Vec<Article>)> {
        let feed = self.feed_repository.get_feed(feed_id).await?;
        if let Some(feed) = feed {
            let mut articles = if Utc::now() - feed.last_updated
                > TimeDelta::minutes(self.config.minutes_to_check_for_updates.into())
            {
                let content = Self::download_feed_content(feed.url.as_str()).await?;
                
                // First we check with the feed items and the database if any of the articles is new
                let saved_articles = self.feed_repository.get_feed_articles(feed_id).await?;

                let articles_guid: HashSet<&str> = saved_articles
                    .iter()
                    .map(|article| &*article.guid)
                    .collect();
                    
                // Parse the feed items using our feed parser
                let feed_items = self.feed_parser.parse_feed_items(feed_id, feed.link.clone(), &content)
                    .await
                    .map_err(|e| FeedServiceError::Unexpected(e.into()))?;
                
                // Filter out items that already exist in the database
                let new_items: Vec<(String, Option<String>)> = feed_items
                    .into_iter()
                    .filter(|item| {
                        let identifier = self.feed_parser.get_item_identifier(item);
                        !articles_guid.contains(&*identifier)
                    })
                    .collect();

                if new_items.is_empty() {
                    self.feed_repository
                        .update_last_updated(feed_id, Utc::now())
                        .await?;
                    return Ok((feed, saved_articles));
                }

                let mut join_set: JoinSet<Result<(Article, Option<ArticleContent>)>> =
                    JoinSet::new();
                let feed_link = Arc::new(feed.link.clone());
                let router_path = self.get_article_router_path(feed_id);
                let file_path = self.get_article_file_path(feed_id);
                let image_processor = Arc::new(ImageProcessorFsImpl::new(router_path, file_path));

                // TODO: We are assuming the items are sorted by pub date desc

                // Create the articles from the feed items
                let mut processed_html_articles = 0;
                for (item_data, content) in new_items {
                    let feed_link = feed_link.clone();
                    let img_processor = image_processor.clone();
                    let html_processor = self.html_processor.clone();
                    
                    // Parse the item_data JSON to create an Article
                    let item_json: serde_json::Value = serde_json::from_str(&item_data)
                        .map_err(|e| FeedServiceError::Unexpected(e.into()))?;
                    
                    // If content is not present, we need to download it
                    let has_content = content.is_some();
                    if !has_content {
                        processed_html_articles += 1;
                    }

                    let download_content =
                        if let Some(qty) = self.config.max_articles_qty_to_download {
                            processed_html_articles <= qty
                        } else {
                            true
                        };
                        
                    let title = item_json["title"].as_str()
                        .unwrap_or("Unknown title")
                        .to_string();
                        
                    let link = item_json["link"].as_str()
                        .ok_or_else(|| FeedServiceError::Unexpected(anyhow::anyhow!(
                            "Item missing link"
                        )))?
                        .to_string();
                        
                    let guid = item_json["guid"].as_str()
                        .ok_or_else(|| FeedServiceError::Unexpected(anyhow::anyhow!(
                            "Item missing guid"
                        )))?
                        .to_string();
                        
                    let author = item_json["author"].as_str().map(String::from);
                    
                    let date_str = item_json["date"].as_str()
                        .ok_or_else(|| FeedServiceError::Unexpected(anyhow::anyhow!(
                            "Item missing date"
                        )))?;
                        
                    let date = DateTime::parse_from_rfc3339(date_str)
                        .map_err(|e| FeedServiceError::ParsingDate(link.clone(), e))?
                        .with_timezone(&Utc);
                    
                    let html_parsed = item_json["html_parsed"].as_bool().unwrap_or(false);
                    
                    // Create a new Article
                    let article = Article {
                        id: Uuid::new_v4(),
                        feed_id,
                        title,
                        link: link.clone(),
                        guid,
                        author,
                        html_parsed,
                        content: None,
                        read: false,
                        last_updated: date,
                    };
                    
                    // Start a task to process the article content if available
                    if let Some(content) = content {
                        let content_clone = content.clone();
                        join_set.spawn(async move {
                            let processed_content = if html_parsed {
                                Self::process_html_content(
                                    &content_clone,
                                    html_processor,
                                    img_processor,
                                    feed_link,
                                ).await?
                            } else {
                                Self::process_rss_content(
                                    &content_clone,
                                    html_processor,
                                    img_processor,
                                    feed_link,
                                ).await?
                            };
                            
                            Ok((article, Some(processed_content)))
                        });
                    } 
                    // Or download the content if requested
                    else if download_content {
                        let link_clone = link.clone();
                        join_set.spawn(async move {
                            let downloaded_content = Self::download_html_article(&link_clone).await?;
                            let processed_content = Self::process_html_content(
                                &downloaded_content,
                                html_processor,
                                img_processor,
                                feed_link,
                            ).await?;
                            
                            Ok((article, Some(processed_content)))
                        });
                    } 
                    // Or just return the article without content
                    else {
                        join_set.spawn(async move {
                            Ok((article, None))
                        });
                    }
                }

                let mut processed_articles = vec![];
                while let Some(Ok(article)) = join_set.join_next().await {
                    match article {
                        Ok((article, content)) => processed_articles.push((article, content)),
                        Err(e) => {
                            tracing::error!("there was an error processing an article: {e:?}")
                        }
                    }
                }

                let articles: Vec<&Article> = processed_articles.iter().map(|(a, _)| a).collect();
                // Add the articles
                self.feed_repository
                    .add_articles(feed_id, &articles)
                    .await?;

                let articles_contents: Vec<(&Article, &ArticleContent)> = processed_articles
                    .iter()
                    .filter_map(|(a, c)| c.as_ref().map(|c| (a, c)))
                    .collect();

                if let Err(e) = self
                    .feed_content_repository
                    .save_article_content(&articles_contents)
                    .await
                {
                    tracing::error!("there was an error saving articles content: {e:?}")
                }

                let mut articles: Vec<Article> =
                    processed_articles.into_iter().map(|(a, _)| a).collect();

                articles.extend(saved_articles);

                articles
            } else {
                self.feed_repository.get_feed_articles(feed_id).await?
            };

            articles.sort_by(|a1, a2| a2.last_updated.cmp(&a1.last_updated));

            Ok((feed, articles))
        } else {
            Err(FeedServiceError::FeedNotFound(feed_id))
        }
    }

    async fn get_item_content(&self, feed_id: Uuid, article_id: Uuid) -> Result<(Article, String)> {
        let content = self
            .feed_content_repository
            .get_article_content(feed_id, article_id)
            .await?;

        let article_data = self
            .feed_repository
            .get_article_description(feed_id, article_id)
            .await?;

        if let Some(article_data) = article_data {
            // If the content exist we serve it
            if let Some(content) = content {
                Ok((article_data, content))
            }
            // Otherwise, we download it on demand
            else {
                let feed = self.feed_repository.get_feed(feed_id).await?.ok_or(
                    FeedServiceError::Unexpected(anyhow::anyhow!(
                        "feed with id {feed_id} not found"
                    )),
                )?;
                let content = Self::download_html_article(&article_data.link).await?;

                let router_path = self.get_article_router_path(feed_id);
                let file_path = self.get_article_file_path(feed_id);
                let image_processor = Arc::new(ImageProcessorFsImpl::new(router_path, file_path));

                let processed_article = Self::process_html_content(
                    &content,
                    self.html_processor.clone(),
                    image_processor,
                    feed.link.into(),
                )
                .await?;

                self.feed_content_repository
                    .save_article_content(&[(&article_data, &processed_article)])
                    .await?;

                Ok((article_data, processed_article))
            }
        } else {
            Err(FeedServiceError::ArticleContentNotFound(
                article_id, feed_id,
            ))
        }
    }

    async fn mark_article_as_read(&self, feed_id: Uuid, article_id: Uuid) -> Result<()> {
        Ok(self
            .feed_repository
            .mark_article_as_read(feed_id, article_id)
            .await?)
    }
    
    async fn delete_feed(&self, feed_id: Uuid) -> Result<()> {
        // First verify that the feed exists
        let feed = self.feed_repository.get_feed(feed_id).await?;
        if feed.is_none() {
            return Err(FeedServiceError::FeedNotFound(feed_id));
        }
        
        // Delete the feed content files
        self.feed_content_repository.delete_feed_content(feed_id).await?;
        
        // Delete the feed and its articles from the database
        Ok(self.feed_repository.delete_feed(feed_id).await?)
    }
}
