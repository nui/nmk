use serde::Serialize;

use nmk::arch::detect_current_architecture;
use nmk::time::{seconds_since_build, HumanTime};

#[derive(Serialize)]
struct Info {
    nmk: Nmk,
    rustup: Rustup,
    toolchain: Toolchain,
}

#[derive(Serialize)]
struct Toolchain {
    rustc: &'static str,
    target: &'static str,
}

#[derive(Serialize)]
struct Rustup {
    get_architecture: String,
}

#[derive(Serialize)]
struct Nmk {
    commit: &'static str,
    build_on: Option<String>,
}

pub fn print_info() -> nmk::Result<()> {
    let build_on = seconds_since_build().map(|secs| format!("{} ago", HumanTime::new(secs)));
    let info = Info {
        nmk: Nmk {
            commit: option_env!("GIT_SHORT_SHA").unwrap_or("n/a"),
            build_on,
        },
        rustup: Rustup {
            get_architecture: detect_current_architecture()?,
        },
        toolchain: Toolchain {
            rustc: env!("BUILD_RUSTC_VERSION"),
            target: env!("BUILD_TARGET"),
        },
    };
    println!("{}", toml::to_string_pretty(&info)?);
    Ok(())
}
