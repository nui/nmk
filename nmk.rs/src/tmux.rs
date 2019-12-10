use std::env;
use std::ffi::{CStr, CString};
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Instant;

use crate::arg::Argument;
use crate::core::*;
use crate::nmk::is_dev_machine;
use crate::terminal;

const TMUX: &str = "tmux";

pub struct Tmux<'a> {
    nmk_dir: &'a PathBuf,
    tmux_dir: PathBuf,
    bin: PathBuf,
    version: String,
}

impl<'a> Tmux<'a> {
    pub fn new(nmk_dir: &PathBuf) -> Tmux {
        let tmux_dir = nmk_dir.join("tmux");
        assert!(tmux_dir.is_dir());
        Tmux {
            nmk_dir,
            tmux_dir,
            bin: which::which(TMUX).expect("Cannot find tmux binary"),
            version: Tmux::call_check_version(),
        }
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    fn conf(&self) -> PathBuf {
        let conf_file = format!("{}.conf", &self.version);
        let conf_path = self.tmux_dir.join(conf_file);
        assert!(conf_path.exists(), "tmux {} is not supported", &self.version);
        conf_path
    }

    pub fn setup_environment(&self, arg: &Argument) {
        set_env("NMK_TMUX_DEFAULT_SHELL", which::which("zsh").expect("zsh not found"));
        set_env("NMK_TMUX_DETACH_ON_DESTROY", on_off!(arg.detach_on_destroy));
        set_env("NMK_TMUX_HISTORY", self.tmux_dir.join(".tmux_history"));
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
                    Some(i) => panic!("tmux exit with status: {}", i),
                    None => panic!("terminated by signal"),
                };
            }
            let version_output = String::from_utf8(o.stdout)
                .expect("tmux version output contain non utf-8");
            version_output.trim().split(" ")
                .nth(1).expect(&format!("bad output: {}", version_output))
                .to_string()
        } else {
            panic!("{} not found", TMUX);
        }
    }

    pub fn login_shell(&self, arg: Argument, start: Instant) -> ! {
        let owned_args = {
            let mut vec = vec![TMUX, "-L", arg.socket()];
            if arg.force256color {
                vec.push("-2");
            }
            let config = self.conf();
            vec.extend_from_slice(&["-f", config.to_str().unwrap(), "-c", "exec zsh --login"]);
            vec.into_iter().map(|arg| CString::new(arg).unwrap()).collect::<Vec<_>>()
        };
        let path = CString::new(self.bin.as_os_str().as_bytes()).unwrap();
        let args: Vec<&CStr> = owned_args.iter().map(CString::as_c_str).collect();
        debug!("execv path: {:#?}", path);
        debug!("execv args: {:#?}", args);
        self.print_usage_time(&arg, &start);
        nix::unistd::execv(&path, &args).expect("Can't start login shell");
        unreachable!()
    }

    pub fn exec(&self, arg: Argument, start: Instant) -> ! {
        let socket = arg.socket();
        let owned_args = {
            let mut vec = vec![TMUX, "-L", arg.socket()];
            if arg.force256color {
                vec.push("-2");
            }
            let tmux_args = arg.tmux_args();

            let config = self.conf();
            if is_server_running(socket) {
                if tmux_args.len() > 0 {
                    vec.extend(tmux_args);
                } else {
                    if env::var_os("TMUX").is_some() && !arg.inception {
                        panic!("add --inception to allow nested tmux sessions");
                    }
                    vec.push("attach");
                }
            } else {
                vec.push("-f");
                vec.push(config.to_str().unwrap());
                vec.extend(tmux_args);
            }
            vec.into_iter().map(|arg| CString::new(arg).unwrap()).collect::<Vec<_>>()
        };
        let path = CString::new(self.bin.as_os_str().as_bytes()).unwrap();
        let args: Vec<&CStr> = owned_args.iter().map(CString::as_c_str).collect();
        debug!("execv path: {:#?}", path);
        debug!("execv args: {:#?}", args);
        self.print_usage_time(&arg, &start);
        if self.is_local_tmux() && is_dev_machine() {
            warn!("Using local tmux on development machine")
        }
        nix::unistd::execv(&path, &args).expect("Can't start tmux");
        unreachable!()
    }

    pub fn bin_path(&self) -> &PathBuf {
        &self.bin
    }

    pub fn is_local_tmux(&self) -> bool {
        self.bin.starts_with(self.nmk_dir)
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
        .map(|o| o.success())
        .unwrap_or_default();
    debug!("server {} running", if running { "is" } else { "is not" });
    running
}
