use configuration::Configuration;

mod configuration;
mod ip;

fn main() {
    let cfg = match Configuration::new("cfg.toml") {
        Err(e) => panic!("{}", e),
        Ok(cfg) => cfg,
    };

    println!("Configuration loaded");

    let ip = match ip::get(cfg.ip_discovery_url()) {
        Err(e) => panic!("{}", e),
        Ok(ip) => ip,
    };

    println!("Detected IP: {}", ip);

    if let Some(err) = ip::update(&cfg, &ip) {
        panic!("{}", err);
    }
}
