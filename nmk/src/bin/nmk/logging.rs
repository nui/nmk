use log::{LevelFilter, SetLoggerError};
use simplelog::{SimpleLogger, TermLogger, TerminalMode};

pub fn setup(verbosity: u8) -> Result<(), SetLoggerError> {
    let log_level = match verbosity {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };
    let config = simplelog::ConfigBuilder::new()
        .set_thread_level(LevelFilter::Trace)
        .set_target_level(LevelFilter::Trace)
        .build();
    TermLogger::init(log_level, config.clone(), TerminalMode::Stderr)
        .or_else(|_| SimpleLogger::init(log_level, config))
}
