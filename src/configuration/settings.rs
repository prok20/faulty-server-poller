use serde_aux::field_attributes::deserialize_number_from_string;
use std::net::{IpAddr, SocketAddr};

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub polling: PollingSettings,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct ApplicationSettings {
    pub host: IpAddr,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct PollingSettings {
    pub polling_address: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub max_concurrent_runs: usize,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub max_pending_runs: usize,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub concurrent_requests_per_run: usize,
}

impl ApplicationSettings {
    pub fn address(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
}
