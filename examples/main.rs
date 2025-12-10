use tracing::level_filters::LevelFilter;
use tracing_subscriber::{filter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    let filter = filter::Targets::new()
        .with_default(LevelFilter::WARN)
        .with_targets(vec![
            ("bloboon", LevelFilter::DEBUG),
            ("main", LevelFilter::DEBUG),
        ]);
    let fmt_layer = fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_ansi(true)
        .without_time()
        .with_thread_names(true);
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(filter)
        .init();

    tracing::info!("Starting.");


}

