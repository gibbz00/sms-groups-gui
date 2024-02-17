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

pub(crate) mod file_layer {
    use anyhow::Context;
    use tracing::Subscriber;
    use tracing_appender::non_blocking::WorkerGuard;
    use tracing_subscriber::{registry::LookupSpan, Layer};

    use crate::SmsGroupsConfig;

    pub fn setup<S>(service_name: &str) -> anyhow::Result<(impl Layer<S>, WorkerGuard)>
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        let observability_config = SmsGroupsConfig::read()?.instrumentation;

        let path = observability_config.log_dir.join(format!("{service_name}.log"));

        let log_file = std::fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .with_context(|| format!("Unable to open log file {}", path.display()))?;

        let (log_file_appender, log_file_flush_guard) = tracing_appender::non_blocking(log_file);

        let layer = tracing_subscriber::fmt::Layer::default()
            .with_ansi(false)
            .with_writer(log_file_appender);

        Ok((layer, log_file_flush_guard))
    }
}
