use log::LevelFilter;
use simplelog::{SimpleLogger, TermLogger, TerminalMode};

pub fn setup(verbosity: u8) {
    let log_level = if verbosity > 1 {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    let config = simplelog::ConfigBuilder::new()
        .set_thread_level(LevelFilter::Trace)
        .set_target_level(LevelFilter::Trace)
        .build();
    if TermLogger::init(log_level, config.clone(), TerminalMode::Stderr).is_err() {
        SimpleLogger::init(log_level, config).expect("Unable to setup logging");
    }
}
