use std::os::unix::process::CommandExt;
use std::process::Command;

use nmk::config::one_hot;
use nmk::consts::bin::ZSH;
use nmk::consts::env::NMK_ZSH_GLOBAL_RCS;
use nmk::home::NmkHome;

use crate::cmdline::CmdOpt;
use crate::entrypoint::set_env;
use crate::utils::print_usage_time;

fn has_vendor_zsh(nmk_home: &NmkHome) -> bool {
    nmk_home.nmk_path().vendor_bin().join(ZSH).exists()
}

/// Determine if we should use global zsh resource files
///
/// The primary reason that we need to check this because PATH environment set by us is ignored
/// on some platform.
///
///   - on MacOs, zprofile call /usr/libexec/path_helper which will change order in PATH
///   - on Alpine, global zprofile source /etc/profile which overwrite PATH environment
pub fn use_global_rcs(nmk_home: &NmkHome) -> bool {
    use nmk::platform::PlatformType::{self, *};
    let platform = PlatformType::detect();
    let not_friendly_global_rcs = matches!(platform, MacOs | Alpine | Arch);
    has_vendor_zsh(nmk_home) || !not_friendly_global_rcs
}

pub fn init(nmk_home: &NmkHome) {
    let global_rcs = use_global_rcs(nmk_home);
    if !global_rcs {
        log::debug!("Ignored zsh global resource files");
    }
    set_env(NMK_ZSH_GLOBAL_RCS, one_hot(global_rcs));
}

pub fn exec_login_shell(cmd_opt: &CmdOpt) -> ! {
    let zsh = which::which(ZSH).expect("failed to locate zsh");
    let mut cmd = Command::new(&zsh);
    cmd.env("SHELL", zsh);
    // Signal zsh that it is a login shell by prepend - to arg0
    cmd.arg0("-zsh");
    print_usage_time(&cmd_opt);
    let err = cmd.exec();
    panic!("exec {:?} fail with {:?}", cmd, err);
}
