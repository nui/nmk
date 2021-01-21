use log::LevelFilter;
use simplelog::{SimpleLogger, TermLogger, TerminalMode};

pub fn setup(verbosity: u8) {
    let mut config = simplelog::ConfigBuilder::new();
    config.set_thread_level(LevelFilter::Trace);
    config.set_target_level(LevelFilter::Trace);

    if matches!(verbosity, 0..=1) {
        config.add_filter_allow_str("nmkup");
    }

    let log_level = if verbosity > 0 {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    if TermLogger::init(log_level, config.build(), TerminalMode::Stderr).is_err() {
        SimpleLogger::init(log_level, config.build()).expect("Failed to setup logging");
    }
}
