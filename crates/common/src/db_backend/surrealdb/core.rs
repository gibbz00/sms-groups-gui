use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

use crate::*;

pub type SurrealBackend = Surreal<Client>;

impl DbBackend for SurrealBackend {
    async fn client() -> anyhow::Result<Self> {
        let SurrealDbConfig {
            username,
            password,
            host_port,
            namespace,
            database,
        } = SmsGroupsConfig::read()?.api.surrealdb;

        let client = SurrealBackend::new::<Ws>(host_port.combined_address()).await?;

        client
            .signin(Root {
                username: &username,
                password: &password,
            })
            .await?;

        client.use_ns(namespace).use_db(database).await?;

        Ok(client)
    }

    async fn create<D: DbDocument>(&self, document: D) -> anyhow::Result<Option<D>> {
        self.create::<Option<D>>((D::NAME, document.id().into()))
            .content(document)
            .await
            .map_err(Into::into)
    }
}
