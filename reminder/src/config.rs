use std::{
    fs::File,
    io::{BufReader, Read, Write},
};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct PartialConfig {
    grpc_port: Option<u16>,
    db_uri: Option<String>,
    db_user: Option<String>,
    db_pass: Option<String>,
    notification_cache_interval: Option<u8>,
}

#[derive(Debug, Serialize)]
pub(crate) struct Config {
    pub(crate) grpc_port: u16,
    pub(crate) db_uri: String,
    pub(crate) db_user: String,
    pub(crate) db_pass: String,
    pub(crate) notification_cache_interval: u8,
}
impl Config {
    pub(crate) fn from_file() -> Result<Self> {
        let path = "config.toml";

        let mut content = String::new();
        let reader = File::open(path).map(|f| BufReader::new(f));
        if reader.is_err() {
            let mut file = File::create(path).unwrap();
            let toml = toml::to_string(&Self::default()).unwrap();
            write!(file, "{}", toml).unwrap();
            file.flush().unwrap();

            return Ok(Self::default());
        }

        let mut reader = reader.unwrap();
        reader
            .read_to_string(&mut content)
            .map_err(|e| anyhow!(e))
            .unwrap();
        let config_str = content;

        let partial_config: Result<PartialConfig, toml::de::Error> = toml::from_str(&config_str);
        if partial_config.is_err() {
            return Ok(Self::default());
        }
        let partial_config = partial_config.unwrap();
        let default = Self::default();

        Ok(Self {
            grpc_port: partial_config.grpc_port.unwrap_or(default.grpc_port),
            db_uri: partial_config.db_uri.unwrap_or(default.db_uri),
            db_user: partial_config.db_user.unwrap_or(default.db_user),
            db_pass: partial_config.db_pass.unwrap_or(default.db_pass),
            notification_cache_interval: partial_config
                .notification_cache_interval
                .unwrap_or(default.notification_cache_interval),
        })
    }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            grpc_port: 58946,
            db_uri: "ws://localhost:8000".to_string(),
            db_user: "root".to_string(),
            db_pass: "root".to_string(),
            notification_cache_interval: 10,
        }
    }
}
