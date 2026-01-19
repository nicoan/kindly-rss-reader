use std::{net::IpAddr, str::FromStr};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    /// Ip to serve the app
    #[serde(default = "Config::default_ip")]
    pub ip: IpAddr,

    /// Port to serve the app
    #[serde(default = "Config::default_port")]
    pub port: u16,

    /// Path where we save the data that changes (articles, images, database)
    #[serde(default = "Config::default_data_path")]
    pub data_path: String,

    /// Path where we save static apps data (migrations, templates, etc)
    #[serde(default = "Config::default_static_data_path")]
    pub static_data_path: String,

    /// Maximum HTML articles quantity to pre-download when adding a new feed
    /// If it is None, we download all, otherwise we download the specified quantity
    #[serde(default = "Config::max_articles_qty_to_download")]
    pub max_articles_qty_to_download: Option<u8>,

    /// How many minutes we should wait before checking the feed for new articles
    #[serde(default = "Config::minutes_to_check_for_updates")]
    pub minutes_to_check_for_updates: u16,
}

impl Config {
    pub fn load() -> Self {
        match envy::from_env::<Config>() {
            Ok(config) => config,
            Err(error) => panic!("{:#?}", error),
        }
    }

    pub fn print_information(&self) {
        tracing::info!("Running on: {}:{}", self.ip, self.port);
        tracing::info!("Data path: {}", self.data_path);
        tracing::info!("Static data path: {}", self.static_data_path);
    }

    fn default_data_path() -> String {
        ".".to_owned()
    }

    fn default_static_data_path() -> String {
        ".".to_owned()
    }

    fn default_port() -> u16 {
        3000
    }

    fn default_ip() -> IpAddr {
        IpAddr::from_str("0.0.0.0").expect("there was an error creating the default ip")
    }

    fn max_articles_qty_to_download() -> Option<u8> {
        Some(0)
    }

    fn minutes_to_check_for_updates() -> u16 {
        120
    }
}
