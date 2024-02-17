use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::Thing,
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

    async fn create_document<D: DbDocument>(&self, document: &D) -> anyhow::Result<Option<D>>
    where
        surrealdb::sql::Id: From<D::Id>,
    {
        self.create::<Option<D>>((D::NAME, document.id()))
            .content(document)
            .await
            .map_err(Into::into)
    }

    async fn get_document<D: DbDocument>(&self, document_id: D::Id) -> anyhow::Result<Option<D>>
    where
        surrealdb::sql::Id: From<D::Id>,
    {
        self.select::<Option<D>>((D::NAME, document_id)).await.map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use crate::*;

    // TODO: place behind macro once another backend is introduced

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct MockDocument {
        id: Uuid,
    }

    impl MockDocument {
        fn mock() -> Self {
            Self { id: Uuid::new_v4() }
        }
    }

    impl DbDocument for MockDocument {
        const NAME: &'static str = "mocks";

        type Id = Uuid;

        fn id(&self) -> Self::Id {
            self.id
        }
    }

    #[tokio::test]
    async fn initializes_connection() {
        SurrealBackend::client().await.unwrap().assert_connection().await;
    }

    #[tokio::test]
    async fn creates_mock_document() {
        SurrealBackend::in_test_container(|db| async move {
            let mock = MockDocument::mock();
            let created_mock = db.create_document(&mock).await.unwrap().unwrap();
            assert_eq!(mock, created_mock)
        })
        .await
    }

    #[tokio::test]
    async fn gets_mock_document() {
        SurrealBackend::in_test_container(|db| async move {
            let mock = MockDocument::mock();
            db.create_document(&mock).await.unwrap();
            let got_mock = db.get_document::<MockDocument>(mock.id()).await.unwrap().unwrap();
            assert_eq!(mock, got_mock);
        })
        .await
    }
}
