use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::*;

#[derive(Serialize, Deserialize, Object)]
pub struct Organization {
    pub id: Uuid,
    /// None if root organization.
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub idp: String,
}

impl DbDocument for Organization {
    const NAME: &'static str = "organizations";
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateOrganization {
    pub name: String,
    pub idp: String,
}
