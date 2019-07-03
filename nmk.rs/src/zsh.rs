use std::path::PathBuf;

use crate::argument::Argument;
use crate::core::*;
use crate::platform::{is_alpine, is_arch, is_mac};

fn has_local_zsh(nmk_dir: &PathBuf) -> bool {
    nmk_dir.join("local").join("bin").join("zsh").exists()
}

pub fn use_global_rcs(arg: &Argument, nmk_dir: &PathBuf) -> bool {
    // Some linux distribution global zprofile contains a line that will source everything
    // from /etc/profile. And they do reset $PATH completely.
    // It makes PATH set by nmk unusable
    let bad = is_alpine() || is_arch() || is_mac();
    let no_global_rcs = arg.autofix && bad && !has_local_zsh(nmk_dir);
    !no_global_rcs
}

pub fn setup(arg: &Argument, nmk_dir: &PathBuf) {
    let global_rcs = use_global_rcs(arg, nmk_dir);
    if !global_rcs {
        debug!("ignore zsh global rcs");
    }
    set_env("NMK_ZSH_GLOBAL_RCS", one_hot!(global_rcs));
}
