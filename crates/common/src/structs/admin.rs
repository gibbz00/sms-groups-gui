use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize)]
pub struct Admin {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub organization: ObjectId,
}

impl MongoDbDocument for Admin {
    const COLLECTION_NAME: &'static str = "admin";
    type Id = ObjectId;
}
