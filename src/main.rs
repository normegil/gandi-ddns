use configuration::Configuration;
use reqwest::Error;

mod configuration;

fn main() {
    let cfg = match Configuration::new("cfg.toml") {
        Err(e) => panic!("{}", e),
        Ok(cfg) => cfg,
    };

    println!("Configuration loaded");

    let ip = match get_ip(cfg.ip_discovery_url()) {
        Err(e) => panic!("{}", e),
        Ok(ip) => ip,
    };

    println!("Detected IP: {}", ip);

    println!("API Key: {}", cfg.gandi.api_key)
}

fn get_ip(discovery_url: &str) -> Result<String, Error> {
    let ip = reqwest::blocking::get(discovery_url)?
        .text()?;
    Ok(ip)
}
