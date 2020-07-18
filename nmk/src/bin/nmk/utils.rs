use std::env;
use std::time::Instant;

use nmk::env_name::{DISPLAY, WINDOWID};

use crate::cmdline::Opt;

pub fn is_dev_machine() -> bool {
    env::var_os(DISPLAY).is_some() && env::var_os(WINDOWID).is_some()
}

pub fn print_usage_time(opt: &Opt, start: &Instant) {
    let before_exec = start.elapsed().as_micros();
    if opt.usage {
        println!("{}", before_exec);
    } else {
        log::debug!("usage time: {} μs", before_exec);
    }
}
