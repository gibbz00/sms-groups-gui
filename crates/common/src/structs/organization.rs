use uuid::Uuid;

pub struct Organization {
    pub id: Uuid,
    /// None if root organization.
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub idp: String,
}
