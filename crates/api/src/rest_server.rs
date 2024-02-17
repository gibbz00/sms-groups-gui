use anyhow::Context;
use poem::{listener::TcpListener, middleware::Tracing, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;
use sms_groups_common::*;
use uuid::Uuid;

use crate::*;

pub struct RestServer<D: DbBackend> {
    db: D,
}

impl<D: DbBackend> RestServer<D> {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            db: D::client()
                .await
                .with_context(|| format!("Failed to initialize database backend client, using {}", std::any::type_name::<D>()))?,
        })
    }

    pub async fn seed(self) -> anyhow::Result<()> {
        let RootCredentials { organization, admin } = SmsGroupsConfig::read()?.api.root_credentials;

        let organization_id = Uuid::new_v4();

        let root_organization = Organization {
            id: organization_id,
            parent_id: None,
            name: organization.name,
            idp: organization.idp,
        };

        self.db.create(root_organization).await?;

        let root_admin = Admin {
            id: admin.id,
            name: admin.name,
            organization: organization_id,
        };

        self.db.create(root_admin).await?;

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

        let api_service = OpenApiService::new(RestService, service_name, version).server(&address);
        let swagger_ui = api_service.swagger_ui();
        let app = Route::new()
            .nest("/", api_service.with(Tracing))
            .nest(swagger_ui_path, swagger_ui.with(Tracing));

        Server::new(TcpListener::bind(&address)).run(app).await?;

        Ok(())
    }
}
