use sms_groups_api::*;
use sms_groups_common::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _observability_guard = setup_observability!()?;

    let db = MongoDbClient::new().await?;

    RestServer::new(db).await?.run().await
}
