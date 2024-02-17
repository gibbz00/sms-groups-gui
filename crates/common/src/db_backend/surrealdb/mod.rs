mod core;
pub(crate) use core::SurrealBackend;

#[cfg(feature = "test-utils")]
mod test_conainer;

#[cfg(feature = "test-utils")]
mod test_connection {
    use crate::*;

    impl TestConnection for SurrealBackend {
        async fn assert_connection(&self) {
            self.query("select * from count(1)").await.unwrap();
        }
    }
}
