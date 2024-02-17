use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HostPort {
    pub host: String,
    pub port: u16,
}

impl HostPort {
    pub fn combined_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
