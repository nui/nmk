use std::convert::TryFrom;
use std::io::{BufWriter, Write};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, io};

use tempfile::NamedTempFile;

use nmk::bin_name::{TMUX, ZSH};
use nmk::env_name::NMK_TMUX_VERSION;
use nmk::tmux::version::{ParseVersionError, Version};

use crate::cmdline::Opt;
use crate::core::*;
use crate::utils::{is_dev_machine, print_usage_time};

const TMP_FILE_PREFIX: &str = "tmp.nmk.";
const TMP_FILE_SUFFIX: &str = ".tmux.conf";

pub struct Tmux {
    nmk_home: PathBuf,
    tmux_dir: PathBuf,
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

        Tmux {
            nmk_home: nmk_home.to_owned(),
            tmux_dir,
            bin,
            version,
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

    fn clean_old_config_file(&self, config: &Path) {
        if let Some(parent) = config.parent().and_then(|p| p.to_str()) {
            let pattern = format!("{}/{}*{}", parent, TMP_FILE_PREFIX, TMP_FILE_SUFFIX);
            if let Ok(paths) = glob::glob(&pattern) {
                for entry in paths {
                    if let Ok(p) = entry {
                        if p != config {
                            let _ = fs::remove_file(p);
                        }
                    }
                }
            }
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
        let named_temp_config = self
            .create_temporary_config_file()
            .expect("Unable to create temporary config file");
        let config_path = named_temp_config.path();
        cmd.arg(config_path);
        self.clean_old_config_file(config_path);
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

    fn create_temporary_config_file(&self) -> io::Result<NamedTempFile> {
        let mut builder = tempfile::Builder::new();
        builder.prefix(TMP_FILE_PREFIX).suffix(TMP_FILE_SUFFIX);
        let config = builder.tempfile()?;
        let mut config = BufWriter::new(config);
        nmk::tmux::config::render(self.version, &mut config)?;
        config.flush()?;
        Ok(config
            .into_inner()
            .expect("Unable to get inner from BufWriter"))
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
