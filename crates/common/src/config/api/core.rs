use serde::Deserialize;
use url::Url;

use crate::*;

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub origin: Url,
    pub open_api: OpenApiConfig,
    pub server: HostPort,
    pub mongodb: MongoDbConfig,
    pub root_credentials: RootCredentials,
}
