use sms_groups_common::*;

use poem::{listener::TcpListener, middleware::Tracing, EndpointExt, Route, Server};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("Hello World")
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _observability_guard = setup_observability!()?;

    let api_config = SmsGroupsConfig::read()?.api;
    let address = api_config.combined_address();
    let OpenApiConfig {
        service_name,
        swagger_ui_path,
        version,
    } = api_config.open_api;

    let api_service = OpenApiService::new(Api, service_name, version).server(&address);
    let swagger_ui = api_service.swagger_ui();
    let app = Route::new()
        .nest("/", api_service.with(Tracing))
        .nest(swagger_ui_path, swagger_ui.with(Tracing));

    Server::new(TcpListener::bind(&address)).run(app).await?;

    Ok(())
}
