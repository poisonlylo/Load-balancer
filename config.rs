// config.rs

pub struct AppConfig {
    pub bind_address: String,
    pub upstream_addresses: Vec<String>,
    // Add more configuration options as needed
}

impl AppConfig {
    pub fn new(bind_address: &str, upstream_addresses: Vec<&str>) -> AppConfig {
        AppConfig {
            bind_address: bind_address.to_string(),
            upstream_addresses: upstream_addresses.iter().map(|&s| s.to_string()).collect(),
            // Initialize other configuration options here
        }
    }
}
