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
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }
}
