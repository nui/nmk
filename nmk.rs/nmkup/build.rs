use std::time::{SystemTime, UNIX_EPOCH};
use std::env;

fn main() {
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    println!("cargo:rustc-env=EPOCHSECONDS={}", secs);
    println!("cargo:rustc-env=CARGO_TARGET={}", env::var("TARGET").unwrap());
}