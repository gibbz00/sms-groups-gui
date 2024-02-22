pub use core::MongoDbClient;
mod core {
    use ::mongodb::options::ClientOptions;
    use ::mongodb::{error::Error as MongoDbError, Client, Collection, Database};
    use bson::{doc, Bson, Document};
    use derive_more::Deref;
    use futures::prelude::*;

    use crate::*;

    #[derive(Clone, Deref)]
    pub struct MongoDbClient(Database);

    impl MongoDbClient {
        pub async fn new() -> anyhow::Result<Self> {
            Self::new_impl(SmsGroupsConfig::read()?.api.mongodb).await
        }

        pub(super) async fn new_impl(mongodb_config: MongoDbConfig) -> anyhow::Result<Self> {
            let MongoDbConfig {
                host_port,
                application,
                database,
            } = mongodb_config;

            let mut client_options = ClientOptions::parse(format!("mongodb://{}", host_port.combined_address())).await?;
            client_options.app_name = Some(application);

            let client = Client::with_options(client_options)?;
            let db = client.database(&database);

            Ok(Self(db))
        }
    }

    impl MongoDbClient {
        pub fn get_collection<T: MongoDbDocument>(&self) -> Collection<T> {
            self.collection::<T>(T::COLLECTION_NAME)
        }

        pub async fn create_document<T: MongoDbDocument>(&self, document: &T) -> anyhow::Result<T::Id> {
            let collection = self.get_collection::<T>();
            let inserted_id = collection.insert_one(document, None).await?.inserted_id;

            bson::from_bson(inserted_id).map_err(Into::into)
        }

        pub async fn get_document<T: MongoDbDocument>(&self, document_id: T::Id) -> Result<Option<T>, MongoDbError>
        where
            T: Unpin + Send + Sync,
            T::Id: Into<Bson>,
        {
            let collection = self.get_collection::<T>();
            collection.find_one(doc! {"_id": document_id}, None).await
        }

        pub async fn stream<T: MongoDbDocument>(
            &self,
            filter: Option<Document>,
        ) -> Result<impl Stream<Item = Result<T, MongoDbError>>, MongoDbError> {
            let collection = self.get_collection::<T>();
            collection.find(filter, None).await.map_err(Into::into)
        }
    }

    // TODO: unit test create and get
    #[cfg(test)]
    mod tests {
        use bson::oid::ObjectId;
        use serde::{Deserialize, Serialize};

        use super::*;

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct MockDocument {
            #[serde(rename = "_id")]
            id: ObjectId,
            value: usize,
        }

        impl MockDocument {
            fn new() -> Self {
                Self {
                    id: ObjectId::new(),
                    value: 10,
                }
            }
        }

        impl MongoDbDocument for MockDocument {
            const COLLECTION_NAME: &'static str = "mock-document";
            type Id = ObjectId;
        }

        #[tokio::test]
        async fn creates_document() {
            MongoDbClient::in_test_container(|db| async move {
                let mock = MockDocument::new();
                let created_id = db.create_document(&mock).await.unwrap();
                assert_eq!(mock.id, created_id);
            })
            .await;
        }

        #[tokio::test]
        async fn gets_document() {
            MongoDbClient::in_test_container(|db| async move {
                let mock = MockDocument::new();

                let created_id = db.create_document(&mock).await.unwrap();

                let found_mock = db.get_document::<MockDocument>(created_id).await.unwrap().unwrap();
                assert_eq!(mock.value, found_mock.value);
            })
            .await;
        }

        #[tokio::test]
        async fn streams_documents() {
            MongoDbClient::in_test_container(|db| async move {
                let mock = MockDocument::new();

                db.create_document(&mock).await.unwrap();

                let found_mock = db.stream(None).await.unwrap().try_next().await.unwrap().unwrap();
                assert_eq!(mock, found_mock);
            })
            .await;
        }
    }
}

pub use document::MongoDbDocument;
mod document {
    use serde::{de::DeserializeOwned, Serialize};

    pub trait MongoDbDocument: Serialize + DeserializeOwned {
        const COLLECTION_NAME: &'static str;
        type Id: DeserializeOwned;
    }
}

#[cfg(feature = "test-utils")]
mod test_container {
    use testcontainers_modules::mongo::Mongo;

    use crate::*;

    const TEST_CONTAINER_PORT: u16 = 27017;
    const TEST_CONTAINER_HOST: &str = "127.0.0.1";

    impl TestContainer for MongoDbClient {
        type Image = Mongo;
        type RunnableImage = Mongo;

        async fn init_test_client(container: &testcontainers::Container<'_, Self::Image>) -> Self {
            let mut mongodb_config = SmsGroupsConfig::read().unwrap().api.mongodb;
            mongodb_config.host_port.host = TEST_CONTAINER_HOST.to_string();
            mongodb_config.host_port.port = container.get_host_port_ipv4(TEST_CONTAINER_PORT);

            MongoDbClient::new_impl(mongodb_config).await.unwrap()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test]
        async fn asserts_test_container_connection() {
            MongoDbClient::in_test_container(|db| async move {
                db.assert_connection().await;
            })
            .await;
        }
    }
}

#[cfg(feature = "test-utils")]
mod test_connection {
    use crate::*;

    impl TestConnection for MongoDbClient {
        async fn assert_connection(&self) {
            self.list_collection_names(None).await.unwrap();
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test]
        async fn asserts_connection() {
            let db = MongoDbClient::new().await.unwrap();
            db.assert_connection().await;
        }
    }
}
