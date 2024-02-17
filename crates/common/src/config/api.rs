use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub open_api: OpenApiConfig,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct OpenApiConfig {
    pub service_name: String,
    pub swagger_ui_path: String,
    pub version: String,
}

impl ApiConfig {
    pub fn combined_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
