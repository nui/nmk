use std::io::Write;
use std::iter;
use std::process::{Command, Stdio};

use indoc::indoc;

use crate::NMK_INIT_SCRIPT;

const GET_ARCHITECTURE: &str = indoc! {r##"
    get_architecture || return 1
    echo $RETVAL
"##};

pub fn detect_current_architecture() -> crate::Result<String> {
    let detect_arch_script = NMK_INIT_SCRIPT
        .lines()
        .take_while(|line| !line.starts_with(r##"main "$@""##))
        .chain(iter::once(GET_ARCHITECTURE))
        .collect::<Vec<_>>()
        .join("\n");
    let mut shell = Command::new("sh")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;
    let stdin = shell.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(detect_arch_script.as_bytes())?;
    let output = shell.wait_with_output()?;
    let arch = std::str::from_utf8(&output.stdout)?.trim().to_string();
    Ok(arch)
}
