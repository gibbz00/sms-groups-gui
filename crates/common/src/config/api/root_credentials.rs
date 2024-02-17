use serde::Deserialize;
use uuid::Uuid;

use crate::*;

#[derive(Debug, Deserialize)]
pub struct RootCredentials {
    pub organization: CreateOrganization,
    pub admin: RootAdminCredentials,
}

#[derive(Debug, Deserialize)]
pub struct RootAdminCredentials {
    pub name: String,
    pub id: Uuid,
}
