use std::{net::IpAddr, str::FromStr};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_ip")]
    pub ip: IpAddr,

    #[serde(default = "Config::default_port")]
    pub port: u16,

    #[serde(default = "Config::default_data_path")]
    pub data_path: String,

    #[serde(default = "Config::default_static_data_path")]
    pub static_data_path: String,
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
        "/var/lib/kindlyrss".to_owned()
    }

    fn default_static_data_path() -> String {
        "/usr/share/kindlyrss".to_owned()
    }

    fn default_port() -> u16 {
        3000
    }

    fn default_ip() -> IpAddr {
        IpAddr::from_str("0.0.0.0").expect("there was an error creating the default ip")
    }
}
