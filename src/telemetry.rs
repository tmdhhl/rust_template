use tracing::level_filters::LevelFilter;
use tracing_appender::{non_blocking::WorkerGuard, rolling::RollingFileAppender};
use tracing_subscriber::{
    Layer as _, Registry,
    filter::Filtered,
    fmt::{
        format::{Format, Json},
        time::{ChronoLocal, FormatTime},
    },
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

use crate::configuration::{self, LogSettings};

type FilteredLayer = Filtered<
    tracing_subscriber::fmt::Layer<
        Registry,
        tracing_subscriber::fmt::format::DefaultFields,
        Format<Json, ChronoLocal>,
        tracing_appender::non_blocking::NonBlocking,
    >,
    LevelFilter,
    Registry,
>;

pub fn init_tracing(config: LogSettings) -> Vec<WorkerGuard> {
    let format = create_format();
    let (layers, guards) = create_layers(config, format);
    tracing_subscriber::registry().with(layers).init();
    guards
}

fn create_layers<T, F>(
    config: LogSettings,
    format: Format<T, F>,
) -> (Vec<FilteredLayer>, Vec<WorkerGuard>)
where
    T: Clone,
    F: Clone + FormatTime + 'static,
{
    let mut guards = vec![];
    let mut layers = vec![];
    for c in config.targets {
        let guard;
        let layer;
        let level_filter = LevelFilter::from_level(c.level);
        match c.kind {
            configuration::logging::TargetKind::Stdout => {
                let (stdout_nonblocking, stdout_guard) =
                    tracing_appender::non_blocking(std::io::stdout());
                guard = stdout_guard;
                layer = tracing_subscriber::fmt::layer()
                    .event_format(format.clone().json())
                    .with_writer(stdout_nonblocking)
                    .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S".to_string()))
                    .with_ansi(true)
                    .with_filter(level_filter);
            }
            configuration::logging::TargetKind::File => {
                let file_appender = RollingFileAppender::new(
                    c.rotation,
                    config.log_dir.as_str(),
                    c.filename.as_str(),
                );
                let (file_nonblocking, file_guard) = tracing_appender::non_blocking(file_appender);
                guard = file_guard;
                layer = tracing_subscriber::fmt::layer()
                    .event_format(format.clone().json())
                    .with_timer(ChronoLocal::rfc_3339())
                    .with_writer(file_nonblocking)
                    .with_ansi(false)
                    .with_filter(level_filter);
            }
        }
        layers.push(layer);
        guards.push(guard);
    }

    (layers, guards)
}

fn create_format() -> Format<Json, impl FormatTime + Clone> {
    tracing_subscriber::fmt::format()
        .with_file(true)
        .with_line_number(true)
        .with_timer(ChronoLocal::rfc_3339())
        .with_source_location(true)
        .with_target(false)
        .json()
}
