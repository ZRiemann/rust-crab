use time::macros::format_description;
use tracing_subscriber::{fmt::time::UtcTime, filter::LevelFilter};

pub fn init_appender(
    path: Option<&std::path::Path>,
    with_max_level: impl Into<LevelFilter>,
    with_thread_names: bool,
    with_thread_ids: bool,
    with_file: bool,
    with_line_number: bool
) -> tracing_appender::non_blocking::WorkerGuard {
    let (non_blocking, guard) = match path {
        Some(p) => {
            let file_appender =
                tracing_appender::rolling::daily(p.parent().unwrap(), p.file_name().unwrap());
            tracing_appender::non_blocking(file_appender)
        }
        None => tracing_appender::non_blocking(std::io::stdout()),
    };

    tracing_subscriber::fmt()
        .with_timer(UtcTime::new(format_description!(
            "[hour]:[minute]:[second].[subsecond digits:9]"
        )))
        .with_writer(non_blocking)
        .with_max_level(with_max_level)
        .compact()
        .with_thread_ids(with_thread_ids) 
        .with_thread_names(with_thread_names)
        .with_target(false)
        .with_ansi(true)
        .with_line_number(with_line_number)
        .with_file(with_file)
        .init();
    guard
}
