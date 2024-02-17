use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub service_name: String,
    pub swagger_ui_path: String,
    pub version: String,
    pub host: String,
    pub port: u16,
}

impl ApiConfig {
    pub fn combined_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
