use std::env;
use std::ffi::CString;
use std::path::PathBuf;
use std::process::{exit, Command};
use std::time::Instant;

use crate::argument::Argument;
use crate::core::*;

const TMUX: &str = "tmux";

pub struct Tmux<'a> {
    nmk_dir: &'a PathBuf,
    tmux_dir: PathBuf,
    bin: String,
    version: String,
}

impl<'a> Tmux<'a> {
    pub fn new(nmk_dir: &PathBuf, tmux_dir: PathBuf) -> Tmux {
        let mut version: String;

        if let Ok(o) = Command::new(TMUX).arg("-V").output() {
            if !o.status.success() {
                match o.status.code() {
                    Some(i) => error!("tmux exit with status: {}", i),
                    None => error!("terminated by signal"),
                };
                exit(1);
            }
            let res = String::from_utf8_lossy(&o.stdout);
            let sp: Vec<_> = res.trim().split(" ").collect();
            if sp.len() != 2 {
                error!("can't find tmux version from: {}", res);
                exit(1);
            }
            version = String::from(sp[1]);
        } else {
            error!("{} not found", TMUX);
            exit(1);
        }
        Tmux {
            nmk_dir,
            tmux_dir,
            bin: which::which(TMUX).unwrap().to_str().unwrap().to_string(),
            version,
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

    pub fn setup(&self, arg: &Argument) {
        set_env("NMK_TMUX_DEFAULT_SHELL", which::which("zsh").expect("zsh not found"));
        set_env("NMK_TMUX_DETACH_ON_DESTROY", if arg.detach_on_destroy { "on" } else { "off" });
        set_env("NMK_TMUX_HISTORY", self.nmk_dir.join("tmux").join(".tmux_history"));
        set_env("NMK_TMUX_VERSION", &self.version);
    }

    fn print_usage_time(&self, arg: &Argument, start: &Instant) {
        let before_exec = start.elapsed().as_millis();
        if arg.usage {
            println!("{}", before_exec);
        } else {
            debug!("usage time: {}ms", before_exec);
        }
    }

    pub fn login_shell(&self, arg: Argument, start: Instant) -> ! {
        let mut vec = Vec::new();

        vec.push(self.bin.as_str());
        vec.push("-L");
        vec.push(&arg.socket);
        if arg.force256color {
            vec.push("-2");
        }
        vec.push("-f");
        let config = self.conf();
        vec.push(config.to_str().unwrap());
        vec.push("-c");
        vec.push("exec zsh --login");
        let exec_args: Vec<_> = vec.into_iter().map(|x| CString::new(x).unwrap()).collect();
        let exec_name = CString::new(self.bin.as_bytes()).unwrap();
        debug!("{:#?}", exec_name);
        debug!("{:#?}", exec_args);
        self.print_usage_time(&arg, &start);
        if nix::unistd::execvp(&exec_name, &exec_args).is_err() {
            error!("Can't start login shell")
        }
        // this code is never reach if exec success
        exit(1);
    }

    pub fn exec(&self, arg: Argument, start: Instant) -> ! {
        let mut vec = Vec::new();
        let config = self.conf();
        vec.push(self.bin.as_str());
        vec.push("-L");
        let socket = arg.socket.as_str();
        vec.push(socket);
        if arg.force256color {
            vec.push("-2");
        }
        if is_running(socket) {
            if arg.tmux_args.len() > 0 {
                for i in arg.tmux_args.iter() {
                    vec.push(i);
                }
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
            vec.extend(arg.tmux_args.iter().map(|x| x.as_str()));
        }
        let exec_args: Vec<_> = vec.iter().map(|&x| CString::new(x).unwrap()).collect();
        let exec_name = CString::new(self.bin.as_bytes()).unwrap();
        debug!("{:#?}", exec_name);
        debug!("{:#?}", exec_args);
        self.print_usage_time(&arg, &start);
        if nix::unistd::execvp(&exec_name, &exec_args).is_err() {
            error!("Can't start tmux")
        }
        // this code is never reach if exec success
        exit(1);
    }
}

pub fn is_running(socket: &str) -> bool {
    let cmd = Command::new(TMUX)
        .arg("-L")
        .arg(socket)
        .arg("list-sessions")
        .output();
    let running = cmd.is_ok() && cmd.unwrap().status.success();
    debug!("server {} running", if running { "is" } else { "is not" });
    running
}

pub fn dir(nmk_dir: &PathBuf) -> PathBuf {
    let path = nmk_dir.join("tmux");
    if !path.exists() {
        panic!(format!("{} doesn't exist", path.to_str().unwrap_or_default()));
    }
    path
}
