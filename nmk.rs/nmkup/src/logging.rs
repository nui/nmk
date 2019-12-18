use log::LevelFilter;
use simplelog::{TermLogger, TerminalMode};

pub fn setup(debug: bool) {
    let log_level = if debug { LevelFilter::Debug } else { LevelFilter::Info };
    let config = simplelog::ConfigBuilder::new()
        .set_thread_level(LevelFilter::Trace)
        .set_target_level(LevelFilter::Trace)
        .build();
    // silently ignore if error
    let _ = TermLogger::init(log_level,
                             config,
                             TerminalMode::Stderr);
}