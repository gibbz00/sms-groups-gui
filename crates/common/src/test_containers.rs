use std::future::Future;

use testcontainers::{Container, Image, RunnableImage};
use testcontainers_modules::testcontainers::clients::Cli;

pub trait TestContainer: Sized {
    type Image: Image;
    type RunnableImage: Default + Into<RunnableImage<Self::Image>>;

    async fn in_test_container<F, O>(f: F)
    where
        F: Fn(Self) -> O,
        O: Future<Output = ()>,
    {
        let docker = Cli::default();
        let container = docker.run(Self::RunnableImage::default());
        let client = Self::init_test_client(&container).await;
        f(client).await;

        drop(container);
    }

    async fn init_test_client(container: &Container<'_, Self::Image>) -> Self;
}
