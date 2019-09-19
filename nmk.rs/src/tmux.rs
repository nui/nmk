use std::env;
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;
use std::process::{Command, exit, Stdio};
use std::time::Instant;

use crate::argument::Argument;
use crate::core::*;
use crate::terminal;

const TMUX: &str = "tmux";

pub struct Tmux<'a> {
    nmk_dir: &'a PathBuf,
    tmux_dir: PathBuf,
    bin: PathBuf,
    version: String,
}

impl<'a> Tmux<'a> {
    pub fn new(nmk_dir: &'a PathBuf, tmux_dir: PathBuf) -> Tmux {
        Tmux {
            nmk_dir,
            tmux_dir,
            bin: which::which(TMUX).expect("Cannot find tmux"),
            version: Tmux::call_check_version(),
        }
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    fn conf(&self) -> PathBuf {
        let conf_file = format!("{}.conf", &self.version);
        let conf_path = self.tmux_dir.join(conf_file);
        if !conf_path.exists() {
            error!("tmux {} is not supported", &self.version)
        }
        conf_path
    }

    pub fn setup_environment(&self, arg: &Argument) {
        set_env("NMK_TMUX_DEFAULT_SHELL", which::which("zsh").expect("zsh not found"));
        set_env("NMK_TMUX_DETACH_ON_DESTROY", on_off!(arg.detach_on_destroy));
        set_env("NMK_TMUX_HISTORY", self.nmk_dir.join("tmux").join(".tmux_history"));
        set_env("NMK_TMUX_VERSION", &self.version);
        let color = terminal::support_256_color(arg);
        set_env("NMK_TMUX_DEFAULT_TERMINAL", if color { "screen-256color" } else { "screen" });
        set_env("NMK_TMUX_256_COLOR", one_hot!(color));
    }

    fn print_usage_time(&self, arg: &Argument, start: &Instant) {
        let before_exec = start.elapsed().as_millis();
        if arg.usage {
            println!("{}", before_exec);
        } else {
            debug!("usage time: {}ms", before_exec);
        }
    }

    fn call_check_version() -> String {
        if let Ok(o) = Command::new(TMUX).arg("-V").output() {
            if !o.status.success() {
                match o.status.code() {
                    Some(i) => error!("tmux exit with status: {}", i),
                    None => error!("terminated by signal"),
                };
                exit(1);
            }
            let version_output = String::from_utf8(o.stdout)
                .expect("tmux version output contain non utf-8");
            version_output.trim().split(" ")
                .nth(1).expect(&format!("bad output: {}", version_output))
                .to_string()
        } else {
            error!("{} not found", TMUX);
            exit(1);
        }
    }

    pub fn login_shell(&self, arg: Argument, start: Instant) -> ! {
        let mut vec = Vec::new();

        vec.push(TMUX);
        vec.push("-L");
        vec.push(arg.socket());
        if arg.force256color {
            vec.push("-2");
        }
        vec.push("-f");
        let config = self.conf();
        vec.push(config.to_str().unwrap());
        vec.push("-c");
        vec.push("exec zsh --login");
        let exec_args: Vec<_> = vec.into_iter().flat_map(CString::new).collect();
        let exec_name = CString::new(self.bin.as_os_str().as_bytes()).unwrap();
        debug!("{:#?}", exec_name);
        debug!("{:#?}", exec_args);
        self.print_usage_time(&arg, &start);
        if nix::unistd::execv(&exec_name, &exec_args).is_err() {
            error!("Can't start login shell")
        }
        // this code is never reach if exec success
        exit(1);
    }

    pub fn exec(&self, arg: Argument, start: Instant) -> ! {
        let mut vec = Vec::new();
        let config = self.conf();
        vec.push(TMUX);
        vec.push("-L");
        let socket = arg.socket();
        vec.push(socket);
        if arg.force256color {
            vec.push("-2");
        }
        let tmux_args = arg.tmux_args();
        if is_running(socket) {
            if tmux_args.len() > 0 {
                vec.extend(tmux_args);
            } else {
                if env::var_os("TMUX").is_some() && !arg.inception {
                    warn!("add --inception to allow nested tmux sessions");
                    exit(1);
                }
                vec.push("attach");
            }
        } else {
            vec.push("-f");
            vec.push(config.to_str().unwrap());
            vec.extend(tmux_args);
        }
        let exec_args: Vec<_> = vec.into_iter().flat_map(CString::new).collect();
        let exec_name = CString::new(self.bin.as_os_str().as_bytes()).unwrap();
        debug!("{:#?}", exec_name);
        debug!("{:#?}", exec_args);
        self.print_usage_time(&arg, &start);
        if nix::unistd::execv(&exec_name, &exec_args).is_err() {
            error!("Can't start tmux")
        }
        // this code is never reach if exec success
        exit(1);
    }
}

pub fn is_running(socket: &str) -> bool {
    let running = Command::new(TMUX)
        .arg("-L")
        .arg(socket)
        .arg("list-sessions")
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .status()
        .map(|o| o.success())
        .unwrap_or_default();
    debug!("server {} running", if running { "is" } else { "is not" });
    running
}

pub fn dir(nmk_dir: &PathBuf) -> PathBuf {
    let path = nmk_dir.join("tmux");
    if !path.exists() {
        panic!("{:?} doesn't exist", path);
    }
    path
}
