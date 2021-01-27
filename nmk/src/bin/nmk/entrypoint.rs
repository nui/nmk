use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{env, io};

use nmk::env_name::{EDITOR, LD_LIBRARY_PATH, NMK_HOME, PATH, VIMINIT, ZDOTDIR};
use nmk::home::NmkHome;
use nmk::time::{seconds_since_build, HumanTime};

use crate::cmdline::CmdOpt;
use crate::path_vec::PathVec;
use crate::terminal;
use crate::tmux::{make_config_context, Tmux};

pub fn set_env<K: AsRef<str>, V: AsRef<OsStr>>(key: K, value: V) {
    let key = key.as_ref();
    let value = value.as_ref();
    env::set_var(key, value);
    log::debug!("export {}={:?}", key, value);
}

fn setup_environment_variable(nmk_home: &NmkHome) {
    set_env(NMK_HOME, nmk_home);
    set_env(ZDOTDIR, nmk_home.nmk_path().zsh());

    // Setup Vim
    let vim_dir = nmk_home.nmk_path().vim();
    let mut vim_init = OsString::from(r"source\ ");
    vim_init.push(vim_dir.join("init.vim"));
    set_env(VIMINIT, vim_init);
    setup_preferred_editor();
}

fn setup_preferred_editor() {
    const PREFERRED_EDITORS: &[&str] = &["nvim", "vim"];

    match env::var_os(EDITOR)
        .as_deref()
        .into_iter()
        .chain(PREFERRED_EDITORS.iter().map(OsStr::new))
        .find(|bin| which::which(bin).is_ok())
    {
        Some(editor) => {
            log::debug!("Using {:?} as preferred editor", editor);
            set_env(EDITOR, editor);
        }
        None => env::remove_var(EDITOR),
    }
}

fn setup_shell_search_path(nmk_home: &NmkHome) {
    let nmk_path = nmk_home.nmk_path();
    let mut search_path = PathVec::from(env::var_os(PATH).expect("$PATH not found"));
    let nmk_search_path = vec![
        nmk_path.bin(),
        // vendor directory
        nmk_path.vendor_bin(),
    ];
    search_path = nmk_search_path
        .into_iter()
        .filter(|p| p.exists())
        .chain(search_path)
        .collect();
    search_path = search_path.unique().without_version_managers();
    set_env(PATH, search_path.join());
}

/// Setup custom library path for precompiled tmux and zsh
fn setup_shell_library_path(nmk_home: &NmkHome) {
    let vendor_lib = nmk_home.nmk_path().vendor_lib();
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
        .filter(|p| p.exists())
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

    let nmk_home = NmkHome::locate().expect("Failed to locate dotfiles directory");
    log::debug!("dotfiles directory: {:?}", nmk_home);

    setup_shell_library_path(&nmk_home);
    setup_shell_search_path(&nmk_home);
    setup_environment_variable(&nmk_home);
    crate::zsh::init(&nmk_home);
    if cmd_opt.login {
        crate::zsh::exec_login_shell(&cmd_opt);
    } else {
        let tmux = Tmux::new();
        log::debug!("tmux path = {:?}", tmux.bin);
        log::debug!("tmux version = {}", tmux.version.as_str());
        let use_8bit_color = cmd_opt.force_256_color || terminal::support_256_color();
        let tmp_config;
        let config = if let Some(ref conf) = cmd_opt.tmux_conf {
            conf
        } else {
            let context = make_config_context(&cmd_opt, use_8bit_color);
            let mut buf = Vec::with_capacity(8192);
            nmk::tmux::config::render(&mut buf, &context, tmux.version)?;
            log::debug!("Config length: {}, capacity: {}", buf.len(), buf.capacity());
            if cmd_opt.render {
                return io::stdout().write_all(&buf);
            } else {
                tmp_config = tmux.write_config_in_temp_dir(&cmd_opt, &buf)?;
                &tmp_config
            }
        };
        tmux.exec(&cmd_opt, config, use_8bit_color);
    }
}
