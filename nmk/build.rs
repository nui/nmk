use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

macro_rules! rustc_env {
    ($name:expr, $value:expr) => {
        println!("cargo:rustc-env={}={}", $name, $value);
    };
}

fn get_rustc_version() -> String {
    let Output { stdout, .. } = Command::new("rustc").arg("--version").output().unwrap();
    String::from_utf8(stdout).unwrap()
}

fn main() {
    let epoch_seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    rustc_env!("BUILD_RUSTC_VERSION", get_rustc_version());
    rustc_env!("BUILD_TARGET", std::env::var("TARGET").unwrap());
    rustc_env!("EPOCHSECONDS", epoch_seconds);
}
