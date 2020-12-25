use std::os::unix::process::CommandExt;
use std::process::Command;

use nmk::bin_name::ZSH;
use nmk::env_name::NMK_ZSH_GLOBAL_RCS;
use nmk::home::NmkHome;
use nmk::one_hot;
use nmk::platform::{is_alpine, is_arch, is_mac};

use crate::cmdline::Opt;
use crate::core::*;
use crate::utils::print_usage_time;

fn has_vendor_zsh(nmk_home: &NmkHome) -> bool {
    nmk_home.vendor_bin_dir().join(ZSH).exists()
}

pub fn use_global_rcs(_opt: &Opt, nmk_home: &NmkHome) -> bool {
    // Disable global resource files on some platform
    //   - Some linux distributions force sourcing /etc/profile, they do reset PATH set by nmk.
    //   - MacOs doesn't respect PATH set by nmk, it changes the order.
    let not_friendly_global_rcs = is_mac() || is_alpine() || is_arch();
    has_vendor_zsh(nmk_home) || !not_friendly_global_rcs
}

pub fn setup(opt: &Opt, nmk_home: &NmkHome) {
    let global_rcs = use_global_rcs(opt, nmk_home);
    if !global_rcs {
        log::debug!("ignore zsh global rcs");
    }
    set_env(NMK_ZSH_GLOBAL_RCS, one_hot!(global_rcs));
}

pub fn exec_login_shell(opt: &Opt) -> ! {
    let mut cmd = Command::new(ZSH);
    // Signal zsh that it is a login shell
    cmd.arg0("-zsh");
    print_usage_time(&opt);
    let err = cmd.exec();
    panic!("exec {:?} fail with {:?}", cmd, err);
}
