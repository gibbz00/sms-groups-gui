use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub open_api: OpenApiConfig,
    pub server: HostPort,
    pub surrealdb: SurrealDbConfig,
}
