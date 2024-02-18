use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize)]
pub struct MongoDbConfig {
    #[serde(flatten)]
    pub host_port: HostPort,
    pub application: String,
    pub database: String,
}
