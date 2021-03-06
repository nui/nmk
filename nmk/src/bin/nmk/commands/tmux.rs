use std::os::unix::process::CommandExt;
use std::process::{exit, Command};

use log::{debug, error};

use nmk::consts::bin::TMUX;
use nmk::consts::env::NMK_START_MODE;

use crate::cmdline::{CmdOpt, Tmux};
use crate::terminal;

pub fn command(cmd_opt: &CmdOpt, options: Tmux) -> ! {
    if std::env::var(NMK_START_MODE).is_err() {
        error!("nmk tmux subcommand need shell started from entrypoint");
        exit(1);
    }
    let mut cmd = Command::new(TMUX);
    cmd.args(&["-L", &cmd_opt.socket]);
    let support_256_color = cmd_opt.force_256_color || terminal::support_256_color();
    if support_256_color {
        cmd.arg("-2");
    }
    if cmd_opt.unicode {
        cmd.arg("-u");
    }
    cmd.args(options.args.iter());
    debug!("exec command: {:?}", cmd);
    let err = cmd.exec();
    panic!("exec {:?} fail with {:?}", cmd, err);
}
