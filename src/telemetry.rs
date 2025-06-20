use crate::configuration::{LogSettings, RotationKind};
use tracing::level_filters::LevelFilter;
use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::Layer as _;
use tracing_subscriber::{
    EnvFilter, Registry,
    fmt::{Layer, format::Format},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
type DynLayer = Box<dyn tracing_subscriber::Layer<Registry> + Send + Sync + 'static>;

pub fn init_tracing(config: LogSettings) -> Vec<WorkerGuard> {
    let format = create_format();

    let (layers, guards) = create_layers(config, format);
    let filter = EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into());
    tracing_subscriber::registry()
        .with(layers)
        .with(filter)
        .init();
    guards
}

fn create_layers(
    config: LogSettings,
    format: Format,
) -> (
    Option<Box<dyn tracing_subscriber::Layer<Registry> + Send + Sync + 'static>>,
    Vec<WorkerGuard>,
) {
    let mut guards = vec![];
    let mut layers: Vec<Box<dyn tracing_subscriber::Layer<Registry> + Send + Sync + 'static>> =
        vec![];

    for c in config.targets {
        let guard;
        let layer;
        let format = format.clone();
        match c.kind {
            crate::configuration::TargetKind::Stdout => {
                let (stdout_nonblocking, stdout_guard) =
                    tracing_appender::non_blocking(std::io::stdout());
                guard = stdout_guard;
                layer = Box::new(
                    Layer::new()
                        .event_format(format)
                        .with_writer(stdout_nonblocking)
                        .with_ansi(true),
                ) as DynLayer;
            }
            crate::configuration::TargetKind::File => {
                let file_appender =
                    RollingFileAppender::new(c.rotation.into(), config.log_dir, c.filename);
                let (file_nonblocking, file_guard) = tracing_appender::non_blocking(file_appender);
                guard = file_guard;
                layer = Box::new(
                    Layer::new()
                        .event_format(format)
                        .with_writer(file_nonblocking)
                        .compact(),
                ) as DynLayer;
            }
        }
        layers.push(layer);
        guards.push(guard);
    }

    (
        layers.into_iter().reduce(|x, y| Box::new(x.and_then(y))),
        guards,
    )
}

fn create_format() -> Format {
    Format::default()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
}

impl From<RotationKind> for Rotation {
    fn from(kind: RotationKind) -> Self {
        match kind {
            RotationKind::MINUTELY => Rotation::MINUTELY,
            RotationKind::HOURLY => Rotation::HOURLY,
            RotationKind::DAILY => Rotation::DAILY,
            RotationKind::NEVER => Rotation::NEVER,
        }
    }
}
