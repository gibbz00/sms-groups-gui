pub trait DbBackendTestSuite {
    /// Should panic if no connection could be established.
    async fn assert_connection(&self) {}
}
