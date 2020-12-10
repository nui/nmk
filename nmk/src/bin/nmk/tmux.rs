use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use nmk::bin_name::{TMUX, ZSH};
use nmk::env_name::NMK_TMUX_VERSION;
use nmk::home::NmkHome;
use nmk::tmux::config::Context;
use nmk::tmux::version::{TmuxVersionError, Version};

use crate::cmdline::Opt;
use crate::utils::{is_dev_machine, print_usage_time};

pub struct Tmux {
    nmk_home: NmkHome,
    pub bin: PathBuf,
    pub version: Version,
}

fn find_version() -> Result<Version, TmuxVersionError> {
    if let Ok(s) = std::env::var(NMK_TMUX_VERSION) {
        log::debug!("Using tmux version from environment variable");
        Version::from_version_number(&s)
    } else {
        let output = Command::new(TMUX)
            .arg("-V")
            .output()
            .expect("tmux not found");
        if !output.status.success() {
            let code = output.status.code().expect("tmux is terminated by signal");
            panic!("tmux exit with status: {}", code);
        }
        let version_output =
            std::str::from_utf8(&output.stdout).expect("tmux version output contain non utf-8");
        Version::from_version_output(version_output)
    }
}

impl Tmux {
    pub fn new(nmk_home: NmkHome) -> Tmux {
        let bin = which::which(TMUX).expect("Cannot find tmux binary");
        let version = find_version().unwrap_or_else(|e| match e {
            TmuxVersionError::BadOutput(s) => panic!("Bad tmux output: {}", s),
            TmuxVersionError::Unsupported(s) => panic!("Unsupported tmux version: {}", s),
        });
        Tmux {
            nmk_home,
            bin,
            version,
        }
    }

    pub fn exec(&self, opt: &Opt, config: &Path, is_color_term: bool) -> ! {
        let mut cmd = Command::new(TMUX);
        cmd.args(&["-L", &opt.socket]);
        if is_color_term {
            cmd.arg("-2");
        }
        if opt.unicode {
            cmd.arg("-u");
        }
        cmd.arg("-f");
        cmd.arg(config);
        if opt.args.is_empty() {
            // Attach to tmux or create new session
            cmd.args(&["new-session", "-A"]);
            if self.version < Version::V31 {
                cmd.args(&["-s", "0"]);
            }
        } else {
            log::debug!("positional arguments: {:?}", opt.args);
            cmd.args(opt.args.iter());
        }
        log::debug!("exec command: {:?}", cmd);
        print_usage_time(&opt);
        if self.is_vendored_tmux() && is_dev_machine() {
            log::warn!("Using vendored tmux on development machine")
        }
        let err = cmd.exec();
        panic!("exec {:?} fail with {:?}", cmd, err);
    }

    pub fn is_vendored_tmux(&self) -> bool {
        self.bin.starts_with(&self.nmk_home)
    }

    pub fn render_config_in_temp_dir(&self, opt: &Opt, context: Context) -> io::Result<PathBuf> {
        let uid = nix::unistd::Uid::current();
        let tmp_dir = std::env::var("TMPDIR").unwrap_or_else(|_| "/tmp".to_owned());
        let config_name = format!("nmk.{}.{}.tmux.conf", uid, opt.socket);
        let config_path = Path::new(&tmp_dir).join(&config_name);
        let config = File::create(&config_path)?;
        let mut config = BufWriter::new(config);
        nmk::tmux::config::render(&mut config, &context, self.version)?;
        config.flush()?;
        Ok(config_path)
    }
}

pub fn make_config_context(opt: &Opt, is_color_term: bool) -> Context {
    let default_term = if is_color_term {
        "screen-256color"
    } else {
        "screen"
    };
    Context {
        support_256_color: is_color_term,
        detach_on_destroy: opt.detach_on_destroy,
        default_term: default_term.to_owned(),
        default_shell: which::which(ZSH).expect("zsh not found").to_owned(),
    }
}
