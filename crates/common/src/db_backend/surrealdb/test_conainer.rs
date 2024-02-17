use surrealdb::{engine::remote::ws::Ws, opt::auth::Root};
use testcontainers::Container;
use testcontainers_modules::surrealdb::{SurrealDb, SURREALDB_PORT};

use crate::*;

impl TestContainer for SurrealBackend {
    type Image = SurrealDb;
    type RunnableImage = SurrealDb;

    async fn init_test_client(container: &Container<'_, Self::Image>) -> Self {
        // TEMP(XXX):
        // let client = SurrealBackend::new::<Ws>(format!("127.0.0.1:{}", container.get_host_port_ipv4(SURREALDB_PORT)))
        //     .await
        //     .unwrap();
        let client = SurrealBackend::new::<Ws>(format!("127.0.0.1:{}", 8000)).await.unwrap();

        client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await
            .unwrap();

        client.use_ns("test").use_db("test").await.unwrap();

        client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn assert_test_container_connection() {
        SurrealBackend::in_test_container(|client| async move { client.assert_connection().await }).await;
    }
}
