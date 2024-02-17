use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::*;

#[macro_export]
macro_rules! setup_observability {
    () => {
        $crate::Observability::setup(env!("CARGO_BIN_NAME"))
    };
}

pub struct Observability;

/// Flushes remaining instrumentation content upon drop.
#[must_use = "Guard must be assigned to a binding to avoid premature termination of log output, `_` is not enough."]
pub struct ObservabilityGuard {
    #[allow(dead_code)]
    log_file: tracing_appender::non_blocking::WorkerGuard,
}

impl Observability {
    pub fn setup(service_name: &'static str) -> anyhow::Result<ObservabilityGuard> {
        let (layer, log_file_flush_guard) = file_layer::setup(service_name)?;
        tracing_subscriber::registry()
            .with(stdout_layer::setup())
            .with(layer)
            // .with(open_telemetry_layer::setup(service_name)?)
            .with(Self::setup_targets_filter())
            .try_init()?;

        std::panic::set_hook(Box::new(tracing_panic::panic_hook));

        Ok(ObservabilityGuard {
            log_file: log_file_flush_guard,
        })
    }

    fn setup_targets_filter() -> tracing_subscriber::filter::Targets {
        let targets_filter = tracing_subscriber::filter::Targets::new();

        // Example:
        // #[cfg(feature = "aws-s3")]
        // let targets_filter = targets_filter.with_target("aws_smithy_runtime", Level::WARN);

        targets_filter.with_default(Level::INFO)
    }
}
