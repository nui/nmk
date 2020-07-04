use std::path::Path;

use nmk::platform::{is_alpine, is_arch, is_mac};

use crate::cmdline::Opt;
use crate::core::*;

fn has_vendored_zsh(nmk_home: &Path) -> bool {
    nmk_home.join("vendor").join("bin").join("zsh").exists()
}

pub fn use_global_rcs(_arg: &Opt, nmk_home: &Path) -> bool {
    // Disable global resource files on some platform
    //   - Some linux distributions force sourcing /etc/profile, they do reset PATH set by nmk.
    //   - MacOs doesn't respect PATH set by nmk, it change the order.
    let not_friendly_global_rcs = is_mac() || is_alpine() || is_arch();
    let no_global_rcs = not_friendly_global_rcs && !has_vendored_zsh(nmk_home);
    !no_global_rcs
}

pub fn setup(arg: &Opt, nmk_home: &Path) {
    let global_rcs = use_global_rcs(arg, nmk_home);
    if !global_rcs {
        log::debug!("ignore zsh global rcs");
    }
    set_env("NMK_ZSH_GLOBAL_RCS", one_hot!(global_rcs));
}
