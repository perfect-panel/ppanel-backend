use std::net::IpAddr;
use std::time::Duration;

use serde::Deserialize;

const IPINFO: &str = "ipinfo.io";
const IPAPI: &str = "ipapi.co";
const IPBASE: &str = "api.ipbase.com";
const IPWHOIS: &str = "ipwhois.app";

const SERVICES: &[&str] = &[IPBASE, IPAPI, IPWHOIS, IPINFO];

#[derive(Debug, thiserror::Error)]
pub enum IpError {
    #[error("DNS resolution failed for {0}")]
    DnsResolutionFailed(String),
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON parsing failed: {0}")]
    Json(#[from] serde_json::Error),
    #[error("all geolocation services failed")]
    AllServicesFailed,
}

pub async fn resolve_ip(input: &str) -> Result<Vec<String>, IpError> {
    if let Ok(ip) = input.parse::<IpAddr>() {
        return Ok(vec![ip.to_string()]);
    }

    let addrs = tokio::net::lookup_host(input).await.map_err(|_| {
        IpError::DnsResolutionFailed(input.to_string())
    })?;

    let ips: Vec<String> = addrs.map(|sa| sa.ip().to_string()).collect();
    if ips.is_empty() {
        return Err(IpError::DnsResolutionFailed(input.to_string()));
    }
    Ok(ips)
}

pub async fn get_region_by_ip(ip: &str) -> Result<GeoLocationResponse, IpError> {
    let client = new_http_client();

    for service in SERVICES {
        match fetch_geolocation(&client, service, ip).await {
            Ok(resp) => return Ok(resp),
            Err(e) => {
                tracing::error!("Failed to fetch geolocation from {}: {:?}", service, e);
                continue;
            }
        }
    }

    Err(IpError::AllServicesFailed)
}

fn new_http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:134.0) Gecko/20100101 Firefox/134.0")
        .build()
        .expect("Failed to build reqwest::Client")
}

async fn fetch_geolocation(
    client: &reqwest::Client,
    service: &str,
    ip: &str,
) -> Result<GeoLocationResponse, IpError> {
    let api_url = match service {
        IPINFO => format!("https://ipinfo.io/{}/json", ip),
        IPAPI => format!("https://ipapi.co/{}/json", ip),
        IPBASE => format!("https://api.ipbase.com/v1/json/{}", ip),
        IPWHOIS => format!("https://ipwhois.app/json/{}", ip),
        _ => unreachable!(),
    };

    let resp = client
        .get(&api_url)
        .header("Host", service)
        .header("Accept", "application/json, text/html, application/xhtml+xml, */*;q=0.8")
        .header("Accept-Language", "en-US,en;q=0.5")
        .header("Accept-Encoding", "gzip, deflate, br, zstd")
        .header("Connection", "keep-alive")
        .header("Upgrade-Insecure-Requests", "1")
        .send()
        .await?;

    let bytes = resp.bytes().await?;
    let mut location: GeoLocationResponse = serde_json::from_slice(&bytes)?;

    if location.country.is_empty() {
        location.country = location.country_name.clone();
    }

    if !location.loc.is_empty() {
        if let Some((lat, lon)) = location.loc.split_once(',') {
            location.latitude = lat.trim().to_string();
            location.longitude = lon.trim().to_string();
        }
    }

    Ok(location)
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(default)]
pub struct GeoLocationResponse {
    pub country: String,
    pub country_name: String,
    pub region: String,
    pub city: String,
    pub latitude: String,
    pub longitude: String,
    pub loc: String,
}
