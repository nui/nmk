use std::env;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Instant;

use crate::core::*;
use crate::nmk::cmdline::Opt;
use crate::nmk::is_dev_machine;
use crate::nmk::terminal;

const TMUX: &str = "tmux";

pub struct Tmux<'a> {
    nmk_dir: &'a PathBuf,
    tmux_dir: PathBuf,
    config: PathBuf,
    pub bin: PathBuf,
    pub version: String,
}

fn find_config(tmux_dir: &PathBuf, version: &str) -> PathBuf {
    let config = tmux_dir.join(format!("{}.conf", version));
    assert!(config.exists(), "unsupported tmux version: {}", version);
    config
}

fn find_version() -> String {
    if let Ok(output) = Command::new(TMUX).arg("-V").output() {
        if !output.status.success() {
            let code = output.status.code().expect("tmux is terminated by signal");
            panic!("tmux exit with status: {}", code);
        }
        let version_output = String::from_utf8(output.stdout)
            .expect("tmux version output contain non utf-8");
        version_output.trim().split(" ")
            .nth(1).unwrap_or_else(|| panic!("bad output: {}", version_output))
            .to_string()
    } else {
        panic!("{} not found", TMUX);
    }
}

fn is_server_running(socket: &str) -> bool {
    let running = Command::new(TMUX)
        .arg("-L")
        .arg(socket)
        .arg("list-sessions")
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false);
    log::debug!("server {} running", if running { "is" } else { "is not" });
    running
}

impl<'a> Tmux<'a> {
    pub fn new(nmk_dir: &PathBuf) -> Tmux {
        let tmux_dir = nmk_dir.join("tmux");
        assert!(tmux_dir.is_dir(), "{} is not directory", tmux_dir.to_string_lossy());
        let bin = which::which(TMUX).expect("Cannot find tmux binary");
        let version = find_version();
        let config = find_config(&tmux_dir, &version);
        Tmux {
            nmk_dir,
            tmux_dir,
            bin,
            version,
            config,
        }
    }

    pub fn setup_environment(&self, arg: &Opt) {
        set_env("NMK_TMUX_DEFAULT_SHELL", which::which("zsh").expect("zsh not found"));
        set_env("NMK_TMUX_DETACH_ON_DESTROY", on_off!(arg.detach_on_destroy));
        set_env("NMK_TMUX_HISTORY", self.tmux_dir.join(".tmux_history"));
        set_env("NMK_TMUX_VERSION", &self.version);
        let color = terminal::support_256_color(arg);
        set_env("NMK_TMUX_DEFAULT_TERMINAL", if color { "screen-256color" } else { "screen" });
        set_env("NMK_TMUX_256_COLOR", one_hot!(color));
    }

    fn print_usage_time(&self, arg: &Opt, start: &Instant) {
        let before_exec = start.elapsed().as_millis();
        if arg.usage {
            println!("{}", before_exec);
        } else {
            log::debug!("usage time: {}ms", before_exec);
        }
    }

    pub fn login_shell(&self, arg: &Opt, start: &Instant) -> ! {
        let mut cmd = Command::new(&self.bin);
        cmd.args(&["-L", &arg.socket]);
        if arg.force_256_color {
            cmd.arg("-2");
        }
        cmd.arg("-f");
        cmd.arg(&self.config);
        cmd.args(&["-c", "exec zsh --login"]);
        log::debug!("login command: {:?}", cmd);
        self.print_usage_time(&arg, &start);
        let err = cmd.exec();
        panic!("exec fail with {:?}", err);
    }

    pub fn exec(&self, arg: &Opt, start: &Instant) -> ! {
        let mut cmd = Command::new(&self.bin);
        cmd.args(&["-L", &arg.socket]);
        if arg.force_256_color {
            cmd.arg("-2");
        }
        if is_server_running(&arg.socket) {
            if !arg.tmux_args.is_empty() {
                cmd.args(arg.tmux_args.iter());
            } else {
                if env::var_os("TMUX").is_some() && !arg.inception {
                    panic!("add --inception to allow nested tmux sessions");
                }
                cmd.arg("attach");
            }
        } else {
            cmd.arg("-f");
            cmd.arg(&self.config);
            cmd.args(arg.tmux_args.iter());
        }
        log::debug!("exec command: {:?}", cmd);
        self.print_usage_time(&arg, &start);
        if self.is_local_tmux() && is_dev_machine() {
            log::warn!("Using local tmux on development machine")
        }
        let err = cmd.exec();
        panic!("exec fail with {:?}", err);
    }

    pub fn is_local_tmux(&self) -> bool {
        self.bin.starts_with(self.nmk_dir)
    }
}
