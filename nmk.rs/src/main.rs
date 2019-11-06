#[macro_use]
extern crate log;

use crate::tmux::Tmux;

mod argument;
mod container;
#[macro_use]
mod core;
mod nmk;
mod pathenv;
mod platform;
mod terminal;
mod time;
mod tmux;
mod zsh;


fn main() {
    let start = std::time::Instant::now();
    let arg = argument::parse();
    nmk::setup_logging(arg.debug);
    debug!("{:#?}", arg);

    if arg.ssh {
        nmk::print_message_of_the_day();
    }

    let nmk_dir = nmk::nmk_dir();
    debug!("nmk_dir={:?}", nmk_dir);
    nmk::setup_ld_library_path(&nmk_dir);
    nmk::setup_path(&nmk_dir);

    let tmux = Tmux::new(&nmk_dir);
    debug!("using tmux version {}", tmux.version());

    nmk::setup_environment(&nmk_dir);
    nmk::setup_preferred_editor();
    zsh::setup(&arg, &nmk_dir);
    if arg.login {
        tmux.login_shell(arg, start);
    } else {
        tmux.setup_environment(&arg);
        tmux.exec(arg, start);
    }
}
