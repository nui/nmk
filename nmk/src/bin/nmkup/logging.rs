use log::LevelFilter;
use simplelog::{ColorChoice, TermLogger, TerminalMode};

pub fn setup(verbosity: u8) {
    let mut config = simplelog::ConfigBuilder::new();
    config.set_thread_level(LevelFilter::Trace);
    config.set_target_level(LevelFilter::Trace);

    if matches!(verbosity, 0..=1) {
        config.add_filter_allow_str("nmkup");
    }
    let config = config.build();

    let log_level = if verbosity > 0 {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    TermLogger::init(log_level, config, TerminalMode::Stderr, ColorChoice::Always)
        .expect("failed to setup logging")
}
