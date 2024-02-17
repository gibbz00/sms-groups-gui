#![allow(async_fn_in_trait)]

mod db_backend;
pub use db_backend::DefaultDbBackend;
pub(crate) use db_backend::*;

pub(crate) use rest_service::RestService;
mod rest_service {
    use poem_openapi::{payload::PlainText, OpenApi};

    pub struct RestService;

    #[OpenApi]
    impl RestService {
        #[oai(path = "/", method = "get")]
        async fn index(&self) -> PlainText<&'static str> {
            PlainText("Hello World")
        }
    }
}

pub use rest_server::RestServer;
mod rest_server {
    use anyhow::Context;
    use poem::{listener::TcpListener, middleware::Tracing, EndpointExt, Route, Server};
    use poem_openapi::OpenApiService;
    use sms_groups_common::*;

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

        pub async fn run(&self) -> anyhow::Result<()> {
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
}
