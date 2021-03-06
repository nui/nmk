use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::{env, fs, io};

use log::debug;
use nix::unistd::Uid;

use nmk::consts::bin::{TMUX, ZSH};
use nmk::consts::env::NMK_TMUX_VERSION;
use nmk::tmux::config::Context;
use nmk::tmux::version::{TmuxVersionError, Version};

use crate::cmdline::CmdOpt;
use crate::utils::print_usage_time;

pub struct Tmux {
    pub bin: PathBuf,
    pub version: Version,
}

fn find_version() -> Result<Version, TmuxVersionError> {
    if let Ok(s) = std::env::var(NMK_TMUX_VERSION) {
        debug!("Using tmux version from environment variable");
        Version::from_version(&s)
    } else {
        let Output {
            status,
            stderr,
            stdout,
        } = Command::new(TMUX)
            .arg("-V")
            .output()
            .expect("failed to get tmux version output");
        if !status.success() {
            panic!(
                "tmux exit with status code: {:?}, error: {}",
                status.code(),
                String::from_utf8_lossy(&stderr)
            );
        }
        Version::from_version_output(&stdout)
    }
}

impl Tmux {
    pub fn new() -> Tmux {
        let bin = which::which(TMUX).expect("cannot find tmux binary");
        let version = find_version().expect("find tmux version error");
        Tmux { bin, version }
    }

    pub fn exec(&self, cmd_opt: &CmdOpt, config: &Path, is_color_term: bool) -> ! {
        let mut cmd = Command::new(TMUX);
        cmd.args(&["-L", &cmd_opt.socket]);
        if is_color_term {
            cmd.arg("-2");
        }
        if cmd_opt.unicode {
            cmd.arg("-u");
        }
        cmd.arg("-f");
        cmd.arg(config);
        // Attach to tmux or create new session
        cmd.args(&["new-session", "-A"]);
        if self.version < Version::V31 {
            cmd.args(&["-s", "0"]);
        }
        debug!("exec command: {:?}", cmd);
        print_usage_time(&cmd_opt);
        let err = cmd.exec();
        panic!("exec {:?} fail with {:?}", cmd, err);
    }

    pub fn write_config_in_temp_dir(
        &self,
        cmd_opt: &CmdOpt,
        contents: &[u8],
    ) -> io::Result<PathBuf> {
        let nmk_tmp_dir = create_nmk_tmp_dir()?;
        let config = nmk_tmp_dir.join(format!("{}.tmux.conf", cmd_opt.socket));
        fs::write(&config, contents)?;
        Ok(config)
    }
}

fn create_nmk_tmp_dir() -> io::Result<PathBuf> {
    let tmp_dir = env::temp_dir();
    let nmk_tmp_dir = tmp_dir.join(format!("nmk-{}", Uid::current()));
    if !nmk_tmp_dir.exists() {
        fs::create_dir(&nmk_tmp_dir)?;
        let mut permissions = nmk_tmp_dir.metadata()?.permissions();
        permissions.set_mode(0o700);
        fs::set_permissions(&nmk_tmp_dir, permissions)?;
    }
    Ok(nmk_tmp_dir)
}

pub fn make_config_context(cmd_opt: &CmdOpt, support_256_color: bool) -> Context {
    let default_term = if support_256_color {
        "screen-256color"
    } else {
        "screen"
    };
    Context {
        support_256_color,
        detach_on_destroy: cmd_opt.detach_on_destroy,
        default_term: default_term.to_owned(),
        default_shell: which::which(ZSH).expect("zsh not found"),
    }
}
