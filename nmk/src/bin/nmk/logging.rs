use log::LevelFilter;
use simplelog::{SimpleLogger, TermLogger, TerminalMode};

pub fn setup(verbosity: u8) {
    let log_level = match verbosity {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };
    let config = simplelog::ConfigBuilder::new()
        .set_thread_level(LevelFilter::Trace)
        .set_target_level(LevelFilter::Trace)
        .build();
    if TermLogger::init(log_level, config.clone(), TerminalMode::Stderr).is_err() {
        SimpleLogger::init(log_level, config).expect("Failed to setup logging");
    }
}
