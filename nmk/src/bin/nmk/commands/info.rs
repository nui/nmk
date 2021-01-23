use serde::Serialize;

use nmk::arch::detect_current_architecture;

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
    commit: Option<&'static str>,
}

pub fn print_info() -> nmk::Result<()> {
    let info = Info {
        nmk: Nmk {
            commit: option_env!("GIT_SHORT_SHA"),
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
