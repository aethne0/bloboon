use std::{thread, time};

use tracing::level_filters::LevelFilter;
use tracing_subscriber::{filter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    let filter = filter::Targets::new()
        .with_default(LevelFilter::WARN)
        .with_targets(vec![
            ("bloboon", LevelFilter::DEBUG),
            ("basic", LevelFilter::DEBUG),
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

    let dir = tempfile::TempDir::with_prefix("bloboon-").unwrap();
    tracing::info!(
        "Starting bloboon | {} threads | {}",
        gdt_cpus::num_physical_cores().unwrap(),
        dir.path().to_str().unwrap(),
    );

    gdt_cpus::pin_thread_to_core(0).expect("Couldn't set thread affinity");

    let mut handles = vec![];
    for id in 1..gdt_cpus::num_physical_cores().unwrap() {
        handles.push(
            thread::Builder::new()
                .name(format!("worker-{}", id.to_string()))
                .spawn(move || {
                    gdt_cpus::pin_thread_to_core(id).expect("Couldn't set thread affinity");
                    let mut counter: u64 = 0;
                    let mut buf = [0u8; 64];
                    let mut start = time::Instant::now();
                    loop {
                        fastrand::fill(&mut buf);
                        counter += 1;
                        if counter > 2_000_000_000 {
                            let dur = time::Instant::now().duration_since(start);
                            tracing::info!(
                                "haha | {} | {:?}",
                                monke::fmt_size(
                                    (64.0 * 2_000_000_000.0 / dur.as_secs_f64()).round() as u64
                                ),
                                buf
                            );
                            start = time::Instant::now();
                            counter = 0;
                        }
                    }
                })
                .unwrap(),
        );
    }

    bloboon::wew();
    for h in handles {
        let _ = h.join();
    }

}
