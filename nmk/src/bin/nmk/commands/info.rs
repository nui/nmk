use std::convert::identity;
use std::io::Write;
use std::process::{Command, Stdio};

use indoc::indoc;
use serde::Serialize;

use nmk::NMK_INIT_SCRIPT;

#[derive(Serialize)]
struct Info {
    cargo: Cargo,
    rustup: Rustup,
    nmk: Nmk,
}

#[derive(Serialize)]
struct Cargo {
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

pub fn display_info() {
    let info = Info {
        cargo: Cargo {
            target: env!("CARGO_TARGET"),
        },
        rustup: Rustup {
            get_architecture: get_current_arch_by_script().unwrap_or_else(identity),
        },
        nmk: Nmk {
            commit: option_env!("GIT_SHORT_SHA"),
        },
    };
    if let Ok(s) = toml::to_string_pretty(&info) {
        println!("{}", s);
    }
}

const GET_ARCHITECTURE: &str = indoc! {r##"
    get_architecture || return 1
    echo $RETVAL
"##};

fn get_current_arch_by_script() -> Result<String, String> {
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
        .spawn()
        .expect("Spawn process failed");
    let stdin = shell.stdin.as_mut().expect("Shell must have stdin");
    write!(stdin, "{}", detect_arch_script)
        .map_err(|e| format!("Write shell stdin error: {}", e))?;
    let output = shell
        .wait_with_output()
        .map_err(|e| format!("Wait shell failed with: {}", e))?;
    Ok(String::from_utf8_lossy(output.stdout.as_slice())
        .trim()
        .to_string())
}
