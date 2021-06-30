use std::io::Write;
use std::process::{Command, Stdio};

/// A modified version of https://sh.rustup.rs
///
/// N.B. we could compress this but include-flate (0.1.3) doesn't support stable yet.
const NMKUP_INIT_SCRIPT: &str = include_str!("../../nmkup.nuimk.com/nmkup-init.sh");

pub fn detect_current_architecture() -> crate::Result<String> {
    let get_architecture_script = generate_get_architecture_script();
    let mut shell = Command::new("sh")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;
    let stdin = shell.stdin.as_mut().expect("failed to open stdin");
    stdin.write_all(get_architecture_script.as_bytes())?;
    let output = shell.wait_with_output()?;
    let arch = std::str::from_utf8(&output.stdout)?.trim().to_string();
    Ok(arch)
}

fn generate_get_architecture_script() -> String {
    #[rustfmt::skip]
    let get_architecture_lines = [
        "get_architecture || return 1",
        "echo $RETVAL",
    ];
    NMKUP_INIT_SCRIPT
        .lines()
        .take_while(|line| !line.starts_with(r##"main "$@""##))
        .chain(get_architecture_lines)
        .collect::<Vec<_>>()
        .join("\n")
}
