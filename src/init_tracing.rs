use tracing::Level;
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{
    filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt, Layer, Registry,
};

pub fn init_tracing() -> (non_blocking::WorkerGuard, non_blocking::WorkerGuard) {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));

    let formatting_layer = fmt::layer()
        .pretty()
        .with_file(false)
        .with_writer(std::io::stderr);

    let file_appender = rolling::hourly("./logs/info", "hsrdict-info.log");
    let (non_blocking_appender, guard1) = non_blocking(file_appender);
    let file_layer = fmt::layer()
        .pretty()
        .with_ansi(false)
        .with_writer(non_blocking_appender);

    let error_file_appender = rolling::daily("./logs/warn", "hsrdict-warn.log");
    let (error_non_blocking_appender, guard2) = non_blocking(error_file_appender);
    let error_file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(error_non_blocking_appender)
        .with_filter(tracing_subscriber::filter::LevelFilter::from_level(
            Level::WARN,
        ));

    Registry::default()
        .with(env_filter)
        .with(formatting_layer)
        .with(error_file_layer)
        .with(file_layer)
        .init();

    (guard1, guard2)
}
