use anyhow::Context;
use sms_groups_api::*;
use sms_groups_common::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _observability_guard = setup_observability!()?;

    let db = DefaultDbBackend::client().await.context(format!(
        "Failed to initialize database backend client, using {}",
        std::any::type_name::<DefaultDbBackend>()
    ))?;

    RestServer::new(db).await?.seed().await
}
