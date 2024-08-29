use std::{fs, io, path::PathBuf, str::FromStr};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Configuration {
    ip_discovery_url: Option<String>,
    pub gandi: Gandi,
}

impl Configuration {
    pub fn new(path: &str) -> io::Result<Self> {
        let default_path = PathBuf::from_str("./").unwrap().join(path);
        let conf = match dirs::config_dir() {
            Some(dir) => {
                if dir.join(path).exists() {
                    dir.join(path)
                } else {
                    default_path
                }
            }
            None => default_path,
        };

        println!(
            "Configuration: {}",
            conf.as_os_str()
                .to_str()
                .unwrap_or_else(|| "Path could not be printed")
        );

        let content = fs::read_to_string(&conf)?;
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
