use std::array::IntoIter;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{env, io};

use log::debug;

use nmk::consts::env::{
    EDITOR, LD_LIBRARY_PATH, NMK_HOME, NMK_INITIALIZED, NMK_TMUX_VERSION, PATH, VIMINIT, ZDOTDIR,
};
use nmk::home::NmkHome;
use nmk::human_time::{seconds_since_build, HumanTime};

use crate::cmdline::CmdOpt;
use crate::path_vec::PathVec;
use crate::terminal;
use crate::tmux::{make_config_context, Tmux};

pub fn set_env<K: AsRef<str>, V: AsRef<OsStr>>(key: K, value: V) {
    let key = key.as_ref();
    let value = value.as_ref();
    env::set_var(key, value);
    debug!("export {}={:?}", key, value);
}

fn setup_environment_variable(nmk_home: &NmkHome) {
    set_env(NMK_HOME, nmk_home.path());
    set_env(ZDOTDIR, nmk_home.path().zsh());

    // Setup Vim
    let vim_dir = nmk_home.path().vim();
    set_env(VIMINIT, build_vim_init(&vim_dir));
    setup_preferred_editor();
}

fn build_vim_init(vim_dir: &Path) -> String {
    let init_dot_vim = vim_dir.join("init.vim");
    shell_words::join(&["source".into(), init_dot_vim.to_string_lossy()])
}

/// Set EDITOR to preferred editor
///
/// - if set, check it value for existent, print warning if doesn't exist, do not fix it
/// - otherwise, check hard coded list of preferred editor and set to the one that exist
fn setup_preferred_editor() {
    if let Some(editor) = env::var_os(EDITOR) {
        if which::which(&editor).is_err() {
            log::warn!("Invalid {} or value does not exist: {:?}", EDITOR, editor)
        }
    } else {
        let preferred_editor = ["nvim", "vim"];
        if let Some(ed) = IntoIter::new(preferred_editor).find(|bin| which::which(bin).is_ok()) {
            debug!("Using {} as preferred editor", ed);
            set_env(EDITOR, ed);
        }
    }
}

/// Modify PATH environment
fn setup_shell_search_path(nmk_home: &NmkHome) {
    let nmk_path = nmk_home.path();
    let nmk_search_path = [
        nmk_path.bin(),
        // vendor directory
        nmk_path.vendor_bin(),
    ];
    let search_path = env::var_os(PATH).unwrap_or_else(|| panic!("{} doesn't exist", PATH));
    let mut search_path = PathVec::from(search_path);
    search_path = IntoIter::new(nmk_search_path)
        .filter(|p| p.exists())
        .chain(search_path)
        .collect();
    search_path = search_path.unique().without_version_managers();
    set_env(PATH, search_path.join());
}

/// Setup custom library path for precompiled tmux and zsh
fn setup_shell_library_path(nmk_home: &NmkHome) {
    let vendor_lib = nmk_home.path().vendor_lib();
    if vendor_lib.exists() {
        let mut path = env::var_os(LD_LIBRARY_PATH)
            .map(PathVec::from)
            .unwrap_or_default();
        path.prepend(vendor_lib);
        set_env(LD_LIBRARY_PATH, path.join());
    }
}

fn display_message_of_the_day() -> io::Result<()> {
    let mut stdout = io::stdout();
    ["/var/run/motd.dynamic", "/etc/motd"]
        .iter()
        .map(Path::new)
        .flat_map(File::open)
        .try_for_each(|mut f| io::copy(&mut f, &mut stdout).map(drop))
}

const DAY_SECS: u64 = 24 * 60 * 60;

fn check_for_update_suggest() {
    if let Some(secs) = seconds_since_build() {
        if secs > 45 * DAY_SECS {
            println!(
                "\nnmk: I's been {} since build.\n",
                HumanTime::new(secs).to_human(2)
            );
        }
    }
}

pub fn main(cmd_opt: CmdOpt) -> io::Result<()> {
    if cmd_opt.motd {
        display_message_of_the_day()?;
        check_for_update_suggest()
    }

    let nmk_home = NmkHome::locate().expect("failed to locate dotfiles directory");
    debug!("dotfiles directory: {}", nmk_home);

    setup_shell_library_path(&nmk_home);
    setup_shell_search_path(&nmk_home);
    setup_environment_variable(&nmk_home);
    crate::zsh::init(&nmk_home);
    set_env(NMK_INITIALIZED, "1");
    if cmd_opt.login {
        crate::zsh::exec_login_shell(&cmd_opt);
    } else {
        let tmux = Tmux::new();
        debug!("tmux path = {}", tmux.bin.display());
        debug!("tmux version = {}", tmux.version);
        set_env(NMK_TMUX_VERSION, tmux.version.as_str());
        let support_256_color = cmd_opt.force_256_color || terminal::support_256_color();
        let tmp_config;
        let config = if let Some(ref conf) = cmd_opt.tmux_conf {
            conf
        } else {
            let context = make_config_context(&cmd_opt, support_256_color);
            let mut buf = Vec::with_capacity(8192);
            nmk::tmux::config::render(&mut buf, &context, tmux.version)?;
            debug!(
                "tmux configuration length: {}, capacity: {}, remaining bytes before re-alloc: {}",
                buf.len(),
                buf.capacity(),
                buf.capacity() - buf.len(),
            );
            if cmd_opt.render {
                return io::stdout().write_all(&buf);
            } else {
                tmp_config = tmux.write_config_in_temp_dir(&cmd_opt, &buf)?;
                &tmp_config
            }
        };
        tmux.exec(&cmd_opt, config, support_256_color);
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_build_vim_init() {
        let vim_dir = PathBuf::from("/home/user/.nmk/vim");
        let actual = build_vim_init(&vim_dir);
        assert_eq!(actual, "source /home/user/.nmk/vim/init.vim");

        let vim_dir = PathBuf::from("/home/user with space/.nmk/vim");
        let actual = build_vim_init(&vim_dir);
        assert_eq!(actual, "source '/home/user with space/.nmk/vim/init.vim'");
    }
}
