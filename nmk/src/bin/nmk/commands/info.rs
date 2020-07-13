use std::io::Write;
use std::process::{Command, Stdio};

use serde::Serialize;

use nmk::NMK_INIT_SCRIPT;
use std::convert::identity;

#[derive(Serialize)]
struct Info {
    cargo: Cargo,
}

#[derive(Serialize)]
struct Cargo {
    compiled_target: &'static str,
    detect_target: String,
}

pub fn display_info() {
    let info = Info {
        cargo: Cargo {
            compiled_target: env!("CARGO_TARGET"),
            detect_target: get_current_arch_by_script().unwrap_or_else(identity),
        },
    };
    if let Ok(json) = serde_json::to_string_pretty(&info) {
        println!("{}", json);
    }
}

fn get_current_arch_by_script() -> Result<String, String> {
    let detect_arch_script = NMK_INIT_SCRIPT
        .lines()
        .filter(|l| !l.starts_with("main "))
        .chain(
            ["get_architecture || return 1", "echo $RETVAL"]
                .iter()
                .map(|&x| x),
        )
        .fold(
            String::with_capacity(NMK_INIT_SCRIPT.len()),
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
