use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub open_api: OpenApiConfig,
    pub server: HostPort,
    pub mongodb: MongoDbConfig,
    pub root_credentials: RootCredentials,
}
