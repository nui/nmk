use std::convert::identity;
use std::io::Write;
use std::process::{Command, Stdio};

use serde::Serialize;

use nmk::NMK_INIT_SCRIPT;

#[derive(Serialize)]
struct Info {
    cargo: Cargo,
}

#[derive(Serialize)]
struct Cargo {
    cargo_target: &'static str,
    rustup_get_architecture: String,
}

pub fn display_info() {
    let info = Info {
        cargo: Cargo {
            cargo_target: env!("CARGO_TARGET"),
            rustup_get_architecture: get_current_arch_by_script().unwrap_or_else(identity),
        },
    };
    if let Ok(json) = serde_json::to_string_pretty(&info) {
        println!("{}", json);
    }
}

fn get_current_arch_by_script() -> Result<String, String> {
    let detect_arch_script = NMK_INIT_SCRIPT
        .lines()
        .take_while(|line| !line.starts_with(r##"main "$@""##))
        .chain(vec!["get_architecture || return 1", "echo $RETVAL"])
        // +100 is for last two lines to avoid reallocation
        .fold(
            String::with_capacity(NMK_INIT_SCRIPT.len() + 100),
            |mut acc, line| {
                acc.push_str(line);
                acc.push('\n');
                acc
            },
        );
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
