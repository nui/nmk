use std::env;

use nmk::env::{DISPLAY, WINDOWID};

pub fn is_dev_machine() -> bool {
    env::var_os(DISPLAY).is_some() && env::var_os(WINDOWID).is_some()
}
