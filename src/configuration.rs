use std::{fs, io};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Configuration {
    ip_discovery_url: Option<String>,
    pub gandi: Gandi,
}

impl Configuration {
    pub fn new(path: &str) -> io::Result<Self> {
        let content = fs::read_to_string(&path)?;
        let cfg: Configuration = toml::from_str(&content)?;
        Ok(cfg)
    }

    pub fn ip_discovery_url(&self) -> &str {
        match self.ip_discovery_url.as_ref() {
            None => "https://ifconfig.me/ip",
            Some(url) => url,
        }
    }
}

#[derive(Deserialize)]
pub struct Gandi {
    pub api_key: String,
    pub fully_qualified_domain_name: String,
    pub subdomain: String,
}
