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
    pub authorization_server: AuthorizationServer,
}

impl MongoDbDocument for Organization {
    const COLLECTION_NAME: &'static str = "organization";
    type Id = ObjectId;
}

pub use create::CreateOrganization;
mod create {
    use serde::Deserialize;

    use crate::*;

    #[derive(Debug, Deserialize)]
    pub struct CreateOrganization {
        pub name: String,
        pub authorization_server: AuthorizationServer,
    }

    impl CreateOrganization {
        pub fn implies(&self, organization: Organization) -> bool {
            self.name == organization.name && self.authorization_server == organization.authorization_server
        }
    }
}

pub use authorization_server::AuthorizationServer;
mod authorization_server {
    use poem_openapi::Object;
    use serde::{Deserialize, Serialize};
    use url::Url;

    #[derive(Debug, PartialEq, Serialize, Deserialize, Object)]
    pub struct AuthorizationServer {
        pub issuer_url: Url,
        pub client_id: String,
        pub client_password: String,
    }
}
