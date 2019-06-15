#[macro_use]
extern crate log;

use std::env;
use std::path::PathBuf;

use crate::argument::Argument;
use crate::core::*;
use crate::tmux::Tmux;

mod argument;
mod config;
mod container;
mod core;
mod nmk;
mod pathenv;
mod platform;
mod terminal;
mod time;
mod tmux;
mod zsh;

fn get_unicode() -> &'static str {
    match platform::get() {
        platform::PlatformType::OSX => "en_US.UTF-8",
        _ => "C.UTF-8",
    }
}

fn setup_path(arg: &Argument, nmk_dir: &PathBuf) {
    const PATH: &str = "PATH";
    let mut p = pathenv::UniquePath::parse(env::var_os(PATH).expect("$PATH not found"));
    p.push_front(nmk_dir.join("local").join("bin"));
    p.push_front(nmk_dir.join("bin"));
    if arg.debug {
        for (i, path) in p.unique().iter().enumerate() {
            debug!("PATH[{}]={:?}", i + 1, path);
        }
    }
    env::set_var(PATH, p.make());
}

fn setup_environment(arg: &Argument, nmk_dir: &PathBuf, tmux: &Tmux, unicode_name: &str) {
    let init_vim = nmk_dir.join("vim").join("init.vim");
    let zdotdir = nmk_dir.join("zsh");
    set_env("NMK_DIR", nmk_dir);

    tmux.setup(arg);
    let init_vim_source = format!("source {}", init_vim.to_string_lossy().replace(" ", "\\ "));
    set_env("VIMINIT", init_vim_source);
    set_env("ZDOTDIR", zdotdir);

    env::remove_var("VIRTUAL_ENV");
    const LANG: &str = "LANG";
    let lang_defined = || env::var_os(LANG).is_some();
    if arg.unicode || (arg.autofix && !lang_defined()) {
        set_env(LANG, unicode_name);
    }

    if arg.force_unicode {
        set_env("LC_ALL", unicode_name);
    }

    match env::current_exe() {
        Ok(p) => set_env("NMK_ENTRYPOINT", p),
        Err(e) => panic!(e),
    }
}

fn setup_prefer_editor() {
    const EDITOR: &str = "EDITOR";
    if env::var_os(EDITOR).is_none() {
        for &editor in ["nvim", "vim"].iter() {
            if which::which(editor).is_ok() {
                set_env(EDITOR, editor);
                debug!("using {} as prefer editor", editor);
                break;
            }
        }
    }
}

fn unset_temp_env(config: config::Config) {
    for i in config.tmux_setting_envs {
        env::remove_var(i);
    }
}

fn main() {
    let start = std::time::Instant::now();
    let unicode_name = get_unicode();
    let arg = argument::parse(unicode_name);
    let verbosity = if arg.debug { 3 } else { 0 };
    stderrlog::new()
        .module(module_path!())
        .verbosity(verbosity)
        .init()
        .expect("Cannot setup logger");
    debug!("{:#?}", arg);

    let nmk_dir = nmk::dir();
    let tmux_dir = tmux::dir(&nmk_dir);
    debug!("nmk_dir: {:?}, tmux_dir: {:?}", nmk_dir, tmux_dir);
    nmk::add_local_library(&nmk_dir);
    setup_path(&arg, &nmk_dir);

    let tmux = Tmux::new(&nmk_dir, tmux_dir);
    debug!("using tmux version {}", tmux.version());

    terminal::setup(&arg);
    setup_environment(&arg, &nmk_dir, &tmux, unicode_name);
    zsh::setup(&arg, &nmk_dir);
    setup_prefer_editor();
    if arg.login {
        unset_temp_env(config::load(&nmk_dir));
        tmux.login_shell(arg, start);
    } else {
        tmux.exec(arg, start);
    }
}
