use sms_groups_common::*;

use poem::{listener::TcpListener, Route, Server};
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
    let api_config = SmsGroupsConfig::read()?.api;
    let address = api_config.combined_address();

    let api_service = OpenApiService::new(Api, api_config.service_name, api_config.version).server(&address);
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest(api_config.swagger_ui_path, ui);

    Server::new(TcpListener::bind(&address)).run(app).await?;

    Ok(())
}
