use sms_groups_api::*;
use sms_groups_common::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _observability_guard = setup_observability!()?;

    RestServer::<DefaultDbBackend>::new().await?.seed().await
}
