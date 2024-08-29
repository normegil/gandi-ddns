use std::error::Error;

use serde::Serialize;

use crate::configuration::Configuration;

#[derive(Serialize)]
struct UpdateBody {
    rrset_values: Vec<String>,
}

pub fn get(discovery_url: &str) -> Result<String, Box<dyn Error>> {
    let ip = reqwest::blocking::get(discovery_url)?.text()?;
    Ok(ip)
}

pub fn update(cfg: &Configuration, new_ip: &str) -> Option<Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();

    let value: Vec<String> = vec![new_ip.to_string()];
    let body = UpdateBody {
        rrset_values: value,
    };

    let body = match serde_json::to_string(&body) {
        Err(e) => return Some(Box::new(e)),
        Ok(body) => body,
    };

    let record_type = if new_ip.contains(":") {
        "AAAA"
    } else {
        "A"
    };

    let url = format!(
        "https://api.gandi.net/v5/livedns/domains/{fqdn}/records/{subdomain}/{record_type}",
        fqdn = cfg.gandi.fully_qualified_domain_name,
        subdomain = cfg.gandi.subdomain
    );
    println!("URL: {}", url);
    let request = client
        .put(url)
        .header("Authorization", format!("Apikey {}", cfg.gandi.api_key))
        .body(body);

    let resp = match request.send() {
        Err(e) => return Some(Box::new(e)),
        Ok(resp) => resp,
    };

    println!("{}", resp.status());
    None
}
