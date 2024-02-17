mod core;
pub use core::Observability;

pub(crate) use layers::*;
mod layers {
    pub(crate) mod stdout_layer {
        use tracing::Subscriber;
        use tracing_subscriber::{registry::LookupSpan, Layer};

        pub fn setup<S>() -> impl Layer<S>
        where
            S: Subscriber + for<'a> LookupSpan<'a>,
        {
            tracing_subscriber::fmt::Layer::default()
                .with_file(true)
                .with_line_number(true)
                .with_target(false)
                .pretty()
        }
    }
}
