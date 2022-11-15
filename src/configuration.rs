use std::{io, fs};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Configuration {
}

impl Configuration {
    pub fn new(path: &str) -> io::Result<Self> {
        let content = fs::read_to_string(&path)?;
        let cfg: Configuration = toml::from_str(&content)?;
        Ok(cfg)
    }
}