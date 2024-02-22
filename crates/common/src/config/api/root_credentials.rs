use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize)]
pub struct RootCredentials {
    pub organization: CreateOrganization,
}
