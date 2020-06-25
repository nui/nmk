use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::path::Path;

use nmk::env::{EDITOR, LD_LIBRARY_PATH, NMK_BIN, NMK_HOME, PATH, VIMINIT, VIRTUAL_ENV, ZDOTDIR};

use crate::core::set_env;
use crate::pathenv::PathVec;

pub fn setup_environment(nmk_home: &Path) {
    let init_vim = nmk_home.join("vim").join("init.vim");
    let zdotdir = nmk_home.join("zsh");
    set_env(NMK_HOME, nmk_home);

    let quote_first_space = |s: &str| s.to_string().replace(" ", r"\ ");
    if let Some(path) = init_vim.to_str().map(quote_first_space) {
        set_env(VIMINIT, format!("source {}", path));
    }
    set_env(ZDOTDIR, zdotdir);

    env::remove_var(VIRTUAL_ENV);

    set_env(
        NMK_BIN,
        env::current_exe().expect("fail to get full path to executable"),
    );
}

pub fn setup_preferred_editor() {
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

pub fn setup_path(nmk_home: &Path) {
    let mut bin_path = PathVec::parse(env::var_os(PATH).expect("$PATH not found"));
    bin_path.push_front(nmk_home.join("local").join("bin"));
    bin_path.push_front(nmk_home.join("bin"));
    bin_path = bin_path.unique().no_version_managers();
    log::debug!("{} = {:#?}", PATH, bin_path);
    set_env(PATH, bin_path.make());
}

pub fn setup_ld_library_path(nmk_home: &Path) {
    let local_lib_dir = nmk_home.join("local").join("lib");
    if local_lib_dir.exists() {
        let mut lib_path = match env::var_os(LD_LIBRARY_PATH) {
            Some(value) => PathVec::parse(value),
            None => PathVec::new(),
        };
        lib_path.push_front(local_lib_dir);
        log::debug!("{} = {:#?}", LD_LIBRARY_PATH, lib_path);
        let next_ld = lib_path.make();
        set_env(LD_LIBRARY_PATH, next_ld);
    }
}

pub fn display_message_of_the_day() {
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
