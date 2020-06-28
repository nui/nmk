use std::path::Path;

use nmk::platform::{is_alpine, is_arch, is_mac};

use crate::cmdline::Opt;
use crate::core::*;

fn has_vendored_zsh(nmk_home: &Path) -> bool {
    nmk_home.join("vendor").join("bin").join("zsh").exists()
}

pub fn use_global_rcs(arg: &Opt, nmk_home: &Path) -> bool {
    // Some linux distribution global zprofile contains a line that will source everything
    // from /etc/profile. And they do reset $PATH completely.
    // It makes PATH set by nmk unusable
    let hostile = is_mac() || is_alpine() || is_arch();
    let no_global_rcs = !arg.no_autofix && hostile && !has_vendored_zsh(nmk_home);
    !no_global_rcs
}

pub fn setup(arg: &Opt, nmk_home: &Path) {
    let global_rcs = use_global_rcs(arg, nmk_home);
    if !global_rcs {
        log::debug!("ignore zsh global rcs");
    }
    set_env("NMK_ZSH_GLOBAL_RCS", one_hot!(global_rcs));
}
