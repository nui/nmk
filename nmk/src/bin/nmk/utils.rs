use std::env;

use nmk::env_name::{DISPLAY, WINDOWID};
use nmk::platform::is_mac;

use crate::cmdline::Opt;

pub fn is_dev_machine() -> bool {
    is_mac() || (env::var_os(DISPLAY).is_some() && env::var_os(WINDOWID).is_some())
}

pub fn print_usage_time(opt: &Opt) {
    let before_exec = opt.start_time.elapsed().as_micros();
    if opt.usage {
        println!("{}", before_exec);
    } else {
        log::debug!("usage time: {} μs", before_exec);
    }
}
