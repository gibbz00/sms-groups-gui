use bson::Uuid;
use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize)]
pub struct RootCredentials {
    pub organization: CreateOrganization,
    pub admin: RootAdminCredentials,
}

#[derive(Debug, Deserialize)]
pub struct RootAdminCredentials {
    pub name: String,
    pub idp_id: Uuid,
}
