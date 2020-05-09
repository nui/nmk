use std::time::{SystemTime, UNIX_EPOCH};

macro_rules! rustc_env {
    ($name:expr, $value:expr) => {
        println!("cargo:rustc-env={}={}", $name, $value);
    };
}

fn main() {
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    rustc_env!("EPOCHSECONDS", secs);
    rustc_env!("CARGO_TARGET", std::env::var("TARGET").unwrap());
}