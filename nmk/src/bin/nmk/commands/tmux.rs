use log::debug;
use std::os::unix::process::CommandExt;
use std::process::Command;

use nmk::consts::bin::TMUX;

use crate::cmdline::{CmdOpt, Tmux};
use crate::terminal;

pub fn command(cmd_opt: &CmdOpt, options: Tmux) -> ! {
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
