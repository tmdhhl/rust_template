use anyhow::Result;
use tracing::level_filters::LevelFilter;
use tracing_appender::{
    non_blocking::{NonBlocking, WorkerGuard},
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{
    EnvFilter, Layer as _, Registry,
    fmt::{
        Layer,
        format::{Compact, DefaultFields, Format},
        writer::MakeWriterExt,
    },
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

// 类型别名简化复杂的类型签名
type StdoutLayer = Layer<Registry, DefaultFields, Format, NonBlocking>;
type FileLayer = Layer<Registry, DefaultFields, Format<Compact>, NonBlocking>;

pub fn tracing_init() -> Result<Vec<WorkerGuard>> {
    tracing_init_with_config("logs/", LevelFilter::INFO)
}

pub fn tracing_init_with_config(log_dir: &str, level: LevelFilter) -> Result<Vec<WorkerGuard>> {
    let mut guards = Vec::new();

    let format = create_format();

    let (stdout_guard, stdout_layer) = create_stdout_layer(&format);
    let (file_guard, file_layer) = create_file_layer(log_dir, &format);

    guards.extend([stdout_guard, file_guard]);

    let env_filter = EnvFilter::from_default_env().add_directive(level.into());
    tracing_subscriber::registry()
        .with(stdout_layer.and_then(file_layer))
        .with(env_filter)
        .init();

    Ok(guards)
}

fn create_format() -> Format {
    Format::default()
        .with_line_number(true)
        .with_file(true)
        .with_target(false)
}

fn create_stdout_layer(format: &Format) -> (WorkerGuard, StdoutLayer) {
    let (stdout, guard) = tracing_appender::non_blocking(std::io::stdout());
    let layer = Layer::new()
        .event_format(format.clone())
        .with_writer(stdout)
        .with_ansi(true);
    (guard, layer)
}

fn create_file_layer(log_dir: &str, format: &Format) -> (WorkerGuard, FileLayer) {
    let file_appender = RollingFileAppender::new(Rotation::DAILY, log_dir, "info.log").and(
        RollingFileAppender::new(Rotation::DAILY, log_dir, "error.log"),
    );

    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let layer = Layer::new()
        .event_format(format.clone())
        .with_writer(non_blocking)
        .compact();

    (guard, layer)
}
