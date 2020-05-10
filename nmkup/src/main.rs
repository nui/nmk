use std::env;
use std::path::PathBuf;

mod container;
#[macro_use]
mod core;
mod logging;
mod nmk;
mod nmkup;
mod pathenv;
mod time;
mod version;
mod env_var;
mod platform;

fn main() {
    let arg0 = env::args().next().map(PathBuf::from);
    let name = arg0.as_ref()
        .and_then(|a| a.file_stem())
        .and_then(std::ffi::OsStr::to_str);
    match name {
        Some("nmk") => nmk::main(),
        Some(name) if name.starts_with("nmkup") => {
            if let Err(e) = nmkup::main() {
                eprintln!("{:?}", e);
                std::process::exit(-1);
            }
        }
        Some(name) => panic!("Not support command name: {}", name),
        None => unimplemented!()
    }
}
