use std::fs::File;

use configuration::Configuration;

mod configuration;

fn main() {
    let cfg = match Configuration::new("cfg.toml") {
        Err(e) => panic!("{}", e),
        Ok(cfg) => cfg,
    };   
}
