use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize)]
pub struct SurrealDbConfig {
    pub username: String,
    pub password: String,
    #[serde(flatten)]
    pub host_port: HostPort,
    pub namespace: String,
    pub database: String,
}
