use bson::oid::ObjectId;
use poem::{listener::TcpListener, middleware::Tracing, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;
use sms_groups_common::*;

use crate::*;

pub struct RestServer {
    db: MongoDbClient,
}

impl RestServer {
    pub async fn new(db: MongoDbClient) -> anyhow::Result<Self> {
        Ok(Self { db })
    }

    pub async fn seed(self) -> anyhow::Result<()> {
        let RootCredentials { organization } = SmsGroupsConfig::read()?.api.root_credentials;

        let root_organization = Organization {
            id: ObjectId::new(),
            parent_id: None,
            name: organization.name,
            authorization_server: organization.authorization_server,
        };

        self.db.create_document(&root_organization).await?;

        Ok(())
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let api_config = SmsGroupsConfig::read()?.api;
        let address = api_config.server.combined_address();
        let OpenApiConfig {
            service_name,
            swagger_ui_path,
            version,
        } = api_config.open_api;

        let api_service = OpenApiService::new(RestService { db: self.db }, service_name, version);
        let swagger_ui = api_service.swagger_ui();
        let app = Route::new()
            .nest("/", api_service.with(Tracing))
            .nest(swagger_ui_path, swagger_ui.with(Tracing));

        Server::new(TcpListener::bind(&address)).run(app).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use futures::prelude::*;

    use super::*;

    #[tokio::test]
    async fn seeds_root_credentials() {
        MongoDbClient::in_test_container(|db| async move {
            RestServer::new(db.clone()).await.unwrap().seed().await.unwrap();

            let create_organization = SmsGroupsConfig::read().unwrap().api.root_credentials.organization;
            let found_organization = db.stream::<Organization>(None).await.unwrap().next().await.unwrap().unwrap();

            assert!(create_organization.implies(found_organization))
        })
        .await;
    }
}
