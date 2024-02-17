use crate::*;

pub trait DbBackend: Sized {
    async fn client() -> anyhow::Result<Self>;

    async fn create<D: DbDocument>(&self, document: D) -> anyhow::Result<Option<D>>;
}
