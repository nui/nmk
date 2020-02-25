use log::LevelFilter;
use simplelog::{SimpleLogger, TerminalMode, TermLogger};

pub fn setup(debug: bool) {
    let log_level = if debug { LevelFilter::Debug } else { LevelFilter::Info };
    let config = simplelog::ConfigBuilder::new()
        .set_thread_level(LevelFilter::Trace)
        .set_target_level(LevelFilter::Trace)
        .build();
    if TermLogger::init(log_level,
                        config.clone(),
                        TerminalMode::Stderr).is_err() {
        SimpleLogger::init(log_level, config).expect("Unable to setup logging");
    }
}