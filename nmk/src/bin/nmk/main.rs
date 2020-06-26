use structopt::StructOpt;

use crate::cmdline::Opt;

mod cmdline;
mod container;
#[macro_use]
mod core;
mod init;
mod logging;
mod pathenv;
mod platform;
mod terminal;
mod tmux;
mod utils;
mod version;
mod zsh;

fn main() {
    let start = std::time::Instant::now();
    let arg: Opt = Opt::from_args();
    crate::logging::setup(arg.verbosity);

    if arg.ssh {
        init::display_message_of_the_day();
    }

    let nmk_home = nmk::home::NmkHome::find().expect("Unable to locate dotfiles directory");
    assert!(nmk_home.exists(), "{:?} doesn't exist", nmk_home);

    log::debug!("Dotfiles directory: {:?}", nmk_home);
    init::setup_ld_library_path(&nmk_home);
    init::setup_path(&nmk_home);

    let tmux = tmux::Tmux::new(&nmk_home);
    log::debug!("tmux path = {:?}", tmux.bin);
    log::debug!("tmux version = {}", tmux.version);

    init::setup_environment(&nmk_home);
    init::setup_preferred_editor();
    zsh::setup(&arg, &nmk_home);
    if arg.login {
        tmux.login_shell(&arg, &start);
    } else {
        tmux.setup_environment(&arg);
        tmux.exec(&arg, &start);
    }
}
