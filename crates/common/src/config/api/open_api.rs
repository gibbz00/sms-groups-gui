use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OpenApiConfig {
    pub service_name: String,
    pub swagger_ui_path: String,
    pub version: String,
}
