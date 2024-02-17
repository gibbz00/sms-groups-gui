pub trait DbBackend: Sized {
    async fn client() -> anyhow::Result<Self>;
}
