use bson::{oid::ObjectId, Uuid};
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize)]
pub struct Admin {
    pub id: ObjectId,
    pub idp_id: Uuid,
    pub name: String,
    pub organization: ObjectId,
}

impl MongoDbDocument for Admin {
    const COLLECTION_NAME: &'static str = "admin";
    type Id = ObjectId;
}
