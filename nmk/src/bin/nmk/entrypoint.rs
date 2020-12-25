use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{env, io};

use nmk::env_name::{EDITOR, LD_LIBRARY_PATH, NMK_HOME, NMK_TMP_TMUX_CONF, PATH, VIMINIT, ZDOTDIR};
use nmk::home::NmkHome;
use nmk::time::{seconds_since_build, HumanTime};

use crate::cmdline::Opt;
use crate::core::set_env;
use crate::path_vec::PathVec;
use crate::terminal;
use crate::tmux::{make_config_context, Tmux};

fn setup_environment_variable(nmk_home: &Path) {
    let zsh_dir = nmk_home.join("zsh");
    set_env(NMK_HOME, nmk_home);
    set_env(ZDOTDIR, zsh_dir);

    let vim_init = nmk_home.join("vim").join("init.vim");
    if let Some(path) = vim_init.to_str() {
        set_env(VIMINIT, format!(r"source\ {}", path));
    }
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
            log::debug!("using {:?} as preferred editor", editor);
            set_env(EDITOR, editor);
        }
        None => env::remove_var(EDITOR),
    }
}

fn setup_shell_search_path(nmk_home: &NmkHome) {
    let mut path = PathVec::from(env::var_os(PATH).expect("$PATH not found"));
    let nmk_path = vec![
        nmk_home.bin_dir(),
        // vendor directory
        nmk_home.vendor_bin_dir(),
    ];
    path = nmk_path
        .into_iter()
        .filter(|p| p.exists())
        .chain(path)
        .collect();
    path = path.unique().without_version_managers();
    log::debug!("{} = {:#?}", PATH, path);
    set_env(PATH, path.join());
}

/// Setup custom library path for precompiled tmux and zsh
fn setup_shell_library_path(nmk_home: &NmkHome) {
    let vendor_lib = nmk_home.vendor_lib_dir();
    if vendor_lib.exists() {
        let mut path = env::var_os(LD_LIBRARY_PATH)
            .map(PathVec::from)
            .unwrap_or_default();
        path.push_front(vendor_lib);
        log::debug!("{} = {:#?}", LD_LIBRARY_PATH, path);
        set_env(LD_LIBRARY_PATH, path.join());
    }
}

fn display_message_of_the_day() {
    let mut stdout = std::io::stdout();
    const MOTD: &[&str] = &["/var/run/motd.dynamic", "/etc/motd"];
    MOTD.iter()
        .map(Path::new)
        .filter(|p| p.exists())
        .flat_map(File::open)
        .for_each(|mut f| {
            io::copy(&mut f, &mut stdout).expect("fail to print motd");
        });
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

pub fn main(opt: Opt) -> ! {
    if opt.motd {
        display_message_of_the_day();
        check_for_update_suggest()
    }

    let nmk_home = NmkHome::find().expect("Unable to locate NMK_HOME");
    assert!(nmk_home.exists(), "{:?} doesn't exist", nmk_home);

    log::debug!("Dotfiles directory: {:?}", nmk_home);
    setup_shell_library_path(&nmk_home);
    setup_shell_search_path(&nmk_home);
    setup_environment_variable(&nmk_home);
    setup_preferred_editor();
    crate::zsh::setup(&opt, &nmk_home);
    if opt.login {
        crate::zsh::exec_login_shell(&opt);
    } else {
        let tmux = Tmux::new();
        log::debug!("tmux path = {:?}", tmux.bin);
        log::debug!("tmux version = {}", tmux.version.as_str());
        let is_color_term = opt.force_256_color || terminal::support_256_color();
        let tmp_config;
        let config = match opt.tmux_conf {
            Some(ref config) => config,
            None => {
                let context = make_config_context(&opt, is_color_term);
                let mut buf = Vec::with_capacity(8192);
                nmk::tmux::config::render(&mut buf, &context, tmux.version)
                    .expect("Unable to render config");
                log::debug!("Config length: {}, capacity: {}", buf.len(), buf.capacity());
                if opt.render {
                    io::stdout()
                        .write_all(&buf)
                        .expect("Unable to print config");
                    std::process::exit(0);
                } else {
                    tmp_config = tmux
                        .write_config_in_temp_dir(&opt, &buf)
                        .expect("Unable to create temporary config file");
                    set_env(NMK_TMP_TMUX_CONF, &tmp_config);
                    &tmp_config
                }
            }
        };
        tmux.exec(&opt, config, is_color_term);
    }
}
