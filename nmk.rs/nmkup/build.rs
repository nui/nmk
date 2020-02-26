use std::env;

fn main() {
    println!("cargo:rustc-env=CARGO_TARGET={}", env::var("TARGET").unwrap());
}