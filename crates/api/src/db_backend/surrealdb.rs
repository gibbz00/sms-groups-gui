pub(crate) use core::SurrealBackend;
mod core {
    use sms_groups_common::*;
    use surrealdb::{
        engine::remote::ws::{Client, Ws},
        opt::auth::Root,
        Surreal,
    };

    use crate::*;

    pub type SurrealBackend = Surreal<Client>;

    impl DbBackend for SurrealBackend {
        async fn client() -> anyhow::Result<Self> {
            #[rustfmt::skip]
        let SurrealDbConfig {username, password, host_port, namespace, database} = SmsGroupsConfig::read()?.api.surrealdb;

            let client = SurrealBackend::new::<Ws>(host_port.combined_address()).await?;

            client
                .signin(Root {
                    username: &username,
                    password: &password,
                })
                .await?;

            client.use_ns(namespace).use_db(database).await?;

            Ok(client)
        }
    }
}

#[cfg(test)]
mod test_suite {
    use crate::*;

    impl DbBackendTestSuite for SurrealBackend {
        async fn assert_connection(&self) {
            self.query("select * from count(1)").await.unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    // TODO: place behind macro once another backend is introduced

    #[tokio::test]
    async fn initializes_connection() {
        SurrealBackend::client().await.unwrap().assert_connection().await;
    }
}
