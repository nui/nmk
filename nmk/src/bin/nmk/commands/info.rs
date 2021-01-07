use std::io::Write;
use std::process::{Command, Stdio};

use indoc::indoc;
use serde::Serialize;

use nmk::NMK_INIT_SCRIPT;

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

pub fn print_info() -> crate::Result<()> {
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

const GET_ARCHITECTURE: &str = indoc! {r##"
    get_architecture || return 1
    echo $RETVAL
"##};

fn detect_current_architecture() -> crate::Result<String> {
    // capacity should be bigger than final script size to avoid reallocation
    let capacity = NMK_INIT_SCRIPT.len() + GET_ARCHITECTURE.len();
    let mut detect_arch_script = NMK_INIT_SCRIPT
        .lines()
        .take_while(|line| !line.starts_with(r##"main "$@""##))
        .fold(String::with_capacity(capacity), |mut acc, line| {
            acc.push_str(line);
            acc.push('\n');
            acc
        });
    detect_arch_script.push_str(GET_ARCHITECTURE);
    let mut shell = Command::new("sh")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;
    let stdin = shell.stdin.as_mut().expect("Shell must have stdin");
    write!(stdin, "{}", detect_arch_script)?;
    let output = shell.wait_with_output()?;
    let arch = std::str::from_utf8(&output.stdout)?.trim().to_string();
    Ok(arch)
}
