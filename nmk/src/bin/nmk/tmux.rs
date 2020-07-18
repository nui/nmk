use std::convert::TryFrom;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;
use std::time::Instant;

use nmk::bin_name::{TMUX, ZSH};
use nmk::env_name::NMK_TMUX_VERSION;

use crate::cmdline::Opt;
use crate::core::*;
use crate::utils::{is_dev_machine, print_usage_time};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Version {
    V26,
    V27,
    V28,
    V29,
    V29a,
    V30,
    V30a,
    V31,
    V31a,
    V31b,
}

impl FromStr for Version {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Version::*;
        let v = match s {
            "2.6" => V26,
            "2.7" => V27,
            "2.8" => V28,
            "2.9" => V29,
            "2.9a" => V29a,
            "3.0" => V30,
            "3.0a" => V30a,
            "3.1" => V31,
            "3.1a" => V31a,
            "3.1b" => V31b,
            _ => return Err(()),
        };
        Ok(v)
    }
}

impl AsRef<str> for Version {
    fn as_ref(&self) -> &'static str {
        use Version::*;
        match *self {
            V26 => "2.6",
            V27 => "2.7",
            V28 => "2.8",
            V29 => "2.9",
            V29a => "2.9a",
            V30 => "3.0",
            V30a => "3.0a",
            V31 => "3.1",
            V31a => "3.1a",
            V31b => "3.1b",
        }
    }
}

#[derive(Debug)]
pub enum ParseVersionError {
    BadVersionOutput(String),
    UnsupportedVersion(String),
}

impl Version {
    // Try parse `tmux -v` result
    fn try_from_version_output(version_output: &str) -> Result<Self, ParseVersionError> {
        let version_number = version_output
            .trim()
            .split(" ")
            .nth(1)
            .ok_or_else(|| ParseVersionError::BadVersionOutput(version_output.to_string()))?;
        Self::try_from(version_number)
    }

    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl TryFrom<&str> for Version {
    type Error = ParseVersionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value).map_err(|_| ParseVersionError::UnsupportedVersion(value.to_owned()))
    }
}

pub struct Tmux {
    nmk_home: PathBuf,
    tmux_dir: PathBuf,
    config: PathBuf,
    pub bin: PathBuf,
    pub version: Version,
}

fn find_config(tmux_dir: &PathBuf, version: Version) -> PathBuf {
    let version: &str = version.as_ref();
    let config = tmux_dir.join(format!("{}.conf", version));
    assert!(
        config.exists(),
        "Unable to find config for tmux version: {}",
        version
    );
    config
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
    pub fn new(nmk_home: &Path) -> Tmux {
        let tmux_dir = nmk_home.join("tmux");
        assert!(
            tmux_dir.is_dir(),
            "{} is not directory",
            tmux_dir.to_string_lossy()
        );
        let bin = which::which(TMUX).expect("Cannot find tmux binary");
        let version = find_version().unwrap_or_else(|e| match e {
            ParseVersionError::BadVersionOutput(s) => panic!("Bad tmux output: {}", s),
            ParseVersionError::UnsupportedVersion(s) => panic!("Unsupported tmux version: {}", s),
        });
        let config = find_config(&tmux_dir, version);
        Tmux {
            nmk_home: nmk_home.to_owned(),
            tmux_dir,
            bin,
            version,
            config,
        }
    }

    pub fn setup_environment(&self, opt: &Opt, is_color_term: bool) {
        set_env(
            "NMK_TMUX_DEFAULT_SHELL",
            which::which(ZSH).expect("zsh not found"),
        );
        set_env("NMK_TMUX_DETACH_ON_DESTROY", on_off!(opt.detach_on_destroy));
        set_env("NMK_TMUX_HISTORY", self.tmux_dir.join(".tmux_history"));
        set_env("NMK_TMUX_VERSION", &self.version.as_str());
        let default_term = if is_color_term {
            "screen-256color"
        } else {
            "screen"
        };
        set_env("NMK_TMUX_DEFAULT_TERMINAL", default_term);
        set_env("NMK_TMUX_256_COLOR", one_hot!(is_color_term));
    }

    pub fn exec(&self, opt: &Opt, start: &Instant, is_color_term: bool) -> ! {
        let mut cmd = Command::new(TMUX);
        cmd.args(&["-L", &opt.socket]);
        if is_color_term {
            cmd.arg("-2");
        }
        if opt.unicode {
            cmd.arg("-u");
        }
        cmd.arg("-f");
        cmd.arg(&self.config);
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
        print_usage_time(&opt, &start);
        if self.is_vendored_tmux() && is_dev_machine() {
            log::warn!("Using vendored tmux on development machine")
        }
        let err = cmd.exec();
        panic!("exec {:?} fail with {:?}", cmd, err);
    }

    pub fn is_vendored_tmux(&self) -> bool {
        self.bin.starts_with(&self.nmk_home)
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
