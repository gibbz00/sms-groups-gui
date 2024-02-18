use bson::oid::ObjectId;
use poem_openapi::Object;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize, Object)]
pub struct Organization {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    /// None if root organization.
    pub parent_id: Option<ObjectId>,
    pub name: String,
    pub idp: String,
}

impl MongoDbDocument for Organization {
    const COLLECTION_NAME: &'static str = "organization";
    type Id = ObjectId;
}

#[derive(Debug, Deserialize)]
pub struct CreateOrganization {
    pub name: String,
    pub idp: String,
}
