mod core;
pub(crate) use core::SurrealBackend;

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
