use crate::*;

pub trait DbBackend: Sized {
    async fn client() -> anyhow::Result<Self>;

    async fn create_document<D: DbDocument>(&self, document: &D) -> anyhow::Result<Option<D>>
    where
        surrealdb::sql::Id: From<D::Id>;

    async fn get_document<D: DbDocument>(&self, document_id: D::Id) -> anyhow::Result<Option<D>>
    where
        surrealdb::sql::Id: From<D::Id>;
}
