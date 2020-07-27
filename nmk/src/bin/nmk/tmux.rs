use std::convert::TryFrom;
use std::io;
use std::io::BufWriter;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;

use nmk::bin_name::{TMUX, ZSH};
use nmk::env_name::NMK_TMUX_VERSION;
use nmk::home::NmkHome;
use nmk::tmux::config::Context;
use nmk::tmux::version::{ParseVersionError, Version};

use crate::cmdline::Opt;
use crate::core::*;
use crate::utils::{is_dev_machine, print_usage_time};

const TMP_FILE_PREFIX: &str = "tmp.nmk.";
const TMP_FILE_SUFFIX: &str = ".tmux.conf";

pub struct Tmux {
    nmk_home: NmkHome,
    pub bin: PathBuf,
    pub version: Version,
}

fn find_version() -> Result<Version, ParseVersionError> {
    if let Ok(s) = std::env::var(NMK_TMUX_VERSION) {
        log::debug!("Using tmux version from environment variable");
        Version::try_from(s.as_str())
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
        Version::try_from_version_output(version_output)
    }
}

impl Tmux {
    pub fn new(nmk_home: NmkHome) -> Tmux {
        let bin = which::which(TMUX).expect("Cannot find tmux binary");
        let version = find_version().unwrap_or_else(|e| match e {
            ParseVersionError::BadVersionOutput(s) => panic!("Bad tmux output: {}", s),
            ParseVersionError::UnsupportedVersion(s) => panic!("Unsupported tmux version: {}", s),
        });
        Tmux {
            nmk_home,
            bin,
            version,
        }
    }

    pub fn exec(&self, opt: &Opt, is_color_term: bool) -> ! {
        let mut cmd = Command::new(TMUX);
        cmd.args(&["-L", &opt.socket]);
        if is_color_term {
            cmd.arg("-2");
        }
        if opt.unicode {
            cmd.arg("-u");
        }
        cmd.arg("-f");
        let render_context = make_config_context(&self.nmk_home, opt, is_color_term);
        let named_temp_config_path = self
            .create_temporary_config(&render_context)
            .expect("Unable to create temporary config file");
        if !opt.keep {
            set_env("NMK_TMUX_TEMP_CONF", &named_temp_config_path);
        }
        cmd.arg(named_temp_config_path);
        let tmux_args = opt.args();
        if tmux_args.is_empty() {
            // Attach to tmux or create new session
            cmd.args(&["new-session", "-A"]);
            if self.version < Version::V31 {
                cmd.args(&["-s", "0"]);
            }
        } else {
            cmd.args(tmux_args);
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

    fn create_temporary_config(&self, context: &Context) -> io::Result<PathBuf> {
        let mut builder = tempfile::Builder::new();
        builder.prefix(TMP_FILE_PREFIX).suffix(TMP_FILE_SUFFIX);
        let config = builder.tempfile()?;
        let mut config = BufWriter::new(config);
        nmk::tmux::config::render(&mut config, context, self.version)?;
        config
            .into_inner()?
            .into_temp_path()
            .keep()
            .map_err(|e| e.error)
    }
}

fn make_config_context(nmk_home: &NmkHome, opt: &Opt, is_color_term: bool) -> Context {
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
        tmux_history_file: Some(nmk_home.join(".tmux_history")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let tmux_output = "tmux 3.1b";

        let actual = Version::try_from_version_output(tmux_output);
        assert!(matches!(actual, Ok(Version::V31b)));
    }
}
