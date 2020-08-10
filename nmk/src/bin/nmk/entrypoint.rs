use std::ffi::OsString;
use std::fs::File;
use std::path::Path;
use std::process::exit;
use std::{env, fs, io};

use nmk::env_name::{EDITOR, LD_LIBRARY_PATH, NMK_HOME, PATH, VIMINIT, ZDOTDIR};
use nmk::home::NmkHome;
use nmk::time::{human_time, seconds_since_build};

use crate::cmdline::Opt;
use crate::core::set_env;
use crate::pathenv::PathVec;
use crate::terminal;
use crate::tmux::{make_config_context, Tmux};

fn setup_environment(nmk_home: &Path) {
    let zdotdir = nmk_home.join("zsh");
    set_env(NMK_HOME, nmk_home);
    set_env(ZDOTDIR, zdotdir);

    let init_vim = nmk_home.join("vim").join("init.vim");
    if let Some(path) = init_vim.to_str() {
        set_env(VIMINIT, format!(r"source\ {}", path));
    }
}

fn setup_preferred_editor() {
    const PREFERRED_EDITORS: &[&str] = &["nvim", "vim"];

    match env::var_os(EDITOR)
        .into_iter()
        .chain(PREFERRED_EDITORS.iter().map(OsString::from))
        .find(|bin| which::which(bin).is_ok())
    {
        Some(editor) => {
            log::debug!("using {:?} as preferred editor", editor);
            set_env(EDITOR, editor);
        }
        None => env::remove_var(EDITOR),
    }
}

fn setup_path(nmk_home: &Path) {
    let mut bin_path = PathVec::parse(env::var_os(PATH).expect("$PATH not found"));
    bin_path.push_front(nmk_home.join("vendor").join("bin"));
    bin_path.push_front(nmk_home.join("bin"));
    bin_path = bin_path.unique().no_version_managers();
    log::debug!("{} = {:#?}", PATH, bin_path);
    set_env(PATH, bin_path.make());
}

fn setup_ld_library_path(nmk_home: &Path) {
    let vendored_lib_dir = nmk_home.join("vendor").join("lib");
    if vendored_lib_dir.exists() {
        let mut lib_path = match env::var_os(LD_LIBRARY_PATH) {
            Some(value) => PathVec::parse(value),
            None => PathVec::new(),
        };
        lib_path.push_front(vendored_lib_dir);
        log::debug!("{} = {:#?}", LD_LIBRARY_PATH, lib_path);
        let next_ld = lib_path.make();
        set_env(LD_LIBRARY_PATH, next_ld);
    }
}

fn display_message_of_the_day() {
    let mut stdout = std::io::stdout();
    ["/var/run/motd.dynamic", "/etc/motd"]
        .iter()
        .map(Path::new)
        .filter(|p| p.exists())
        .flat_map(File::open)
        .for_each(|mut f| {
            io::copy(&mut f, &mut stdout).expect("fail to print motd");
        });
}

const DAY_SECS: i64 = 24 * 60 * 60;

fn check_for_update_suggest() {
    if let Some(secs) = seconds_since_build() {
        if secs > 30 * DAY_SECS {
            println!("\nnmk: I's been {} since build.\n", human_time(secs));
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
    setup_ld_library_path(&nmk_home);
    setup_path(&nmk_home);
    setup_environment(&nmk_home);
    setup_preferred_editor();
    crate::zsh::setup(&opt, &nmk_home);
    if opt.login {
        crate::zsh::exec_login_shell(&opt);
    } else {
        let tmux = Tmux::new(nmk_home.clone());
        log::debug!("tmux path = {:?}", tmux.bin);
        log::debug!("tmux version = {}", tmux.version.as_str());
        let is_color_term = terminal::support_256_color(&opt);
        let config = match opt.tmux_conf {
            Some(ref config) => config.clone(),
            None => {
                let context = make_config_context(&opt, is_color_term);
                let config = tmux
                    .render_config_in_temp_dir(&opt, context)
                    .expect("Unable to create temporary config file");
                if opt.render {
                    print_config_then_remove(&config).expect("Unable to print config");
                    exit(0);
                }
                set_env("NMK_TMP_TMUX_CONF", &config);
                config
            }
        };
        tmux.exec(&opt, &config, is_color_term);
    }
}

fn print_config_then_remove(config: &Path) -> io::Result<()> {
    io::copy(&mut File::open(&config)?, &mut io::stdout())?;
    fs::remove_file(config)
}
