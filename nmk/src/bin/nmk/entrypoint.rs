use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::path::Path;

use nmk::env_name::{EDITOR, LD_LIBRARY_PATH, NMK_HOME, PATH, VIMINIT, VIRTUAL_ENV, ZDOTDIR};

use crate::cmdline::Opt;
use crate::core::set_env;
use crate::pathenv::PathVec;
use crate::tmux::Tmux;
use nmk::home::NmkHome;

fn setup_environment(nmk_home: &Path) {
    let zdotdir = nmk_home.join("zsh");
    set_env(NMK_HOME, nmk_home);
    set_env(ZDOTDIR, zdotdir);

    let init_vim = nmk_home.join("vim").join("init.vim");
    if let Some(path) = init_vim.to_str() {
        set_env(VIMINIT, format!(r"source\ {}", path));
    }

    env::remove_var(VIRTUAL_ENV);
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
            std::io::copy(&mut f, &mut stdout).expect("fail to print motd");
        });
}

pub fn setup_then_exec(start: std::time::Instant, arg: Opt) -> ! {
    if arg.ssh {
        display_message_of_the_day();
    }

    let nmk_home = NmkHome::find().expect("Unable to locate NMK_HOME");
    assert!(nmk_home.exists(), "{:?} doesn't exist", nmk_home);

    log::debug!("Dotfiles directory: {:?}", nmk_home);
    setup_ld_library_path(&nmk_home);
    setup_path(&nmk_home);

    let tmux = Tmux::new(&nmk_home);
    log::debug!("tmux path = {:?}", tmux.bin);
    log::debug!("tmux version = {}", tmux.version);

    setup_environment(&nmk_home);
    setup_preferred_editor();
    crate::zsh::setup(&arg, &nmk_home);
    if arg.login {
        tmux.login_shell(&arg, &start);
    } else {
        tmux.setup_environment(&arg);
        tmux.exec(&arg, &start);
    }
}
