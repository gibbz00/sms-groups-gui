use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::*;

#[derive(Serialize, Deserialize)]
pub struct Admin {
    pub id: Uuid,
    pub name: String,
    pub organization: Uuid,
}

impl DbDocument for Admin {
    const NAME: &'static str = "admins";

    fn id(&self) -> impl Into<String> {
        self.id
    }
}
