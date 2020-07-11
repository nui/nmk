use std::ffi::CStr;
use std::path::Path;
use std::time::Instant;

use nix::unistd::execvp;

use nmk::bin_name::ZSH;
use nmk::platform::{is_alpine, is_arch, is_mac};

use crate::cmdline::Opt;
use crate::core::*;
use crate::utils::print_usage_time;

fn has_vendored_zsh(nmk_home: &Path) -> bool {
    nmk_home.join("vendor").join("bin").join(ZSH).exists()
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

fn prepare_exec_args() -> Option<(&'static CStr, [&'static CStr; 1])> {
    let filename = CStr::from_bytes_with_nul(b"zsh\0").ok()?;
    let args = [CStr::from_bytes_with_nul(b"-zsh\0").ok()?];
    Some((filename, args))
}

pub fn exec_login_shell(arg: &Opt, start: &Instant) -> ! {
    let (filename, args) = prepare_exec_args().expect("Unable to prepare zsh login args");
    print_usage_time(&arg, &start);
    if let Err(e) = execvp(filename, &args) {
        panic!("exec zsh login failed with {:?}", e);
    }
    unreachable!()
}
