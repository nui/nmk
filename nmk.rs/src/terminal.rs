use std::env;
use std::ffi::OsStr;

use crate::argument::Argument;
use crate::container;
use crate::core::*;

fn is_256_term<T: AsRef<OsStr>>(term: T) -> bool {
    let arr: [&str; 5] = [
        "cygwin",
        "gnome-256color",
        "putty",
        "screen-256color",
        "xterm-256color",
    ];
    arr.contains(&term.as_ref().to_str().unwrap_or_default())
}

fn is_256_colorterm<T: AsRef<OsStr>>(term: T) -> bool {
    let arr: [&str; 3] = ["gnome-terminal", "rxvt-xpm", "xfce4-terminal"];
    arr.contains(&term.as_ref().to_str().unwrap_or_default())
}

fn support_256_color(arg: &Argument) -> bool {
    arg.force256color
        || is_256_term(env::var_os("TERM").unwrap_or_default())
        || is_256_colorterm(env::var_os("COLORTERM").unwrap_or_default())
        || (arg.autofix && container::check_container())
}

pub fn setup(arg: &Argument) {
    let color = support_256_color(arg);
    set_env("NMK_TMUX_DEFAULT_TERMINAL", if color { "screen-256color" } else { "screen" });
    set_env("NMK_TMUX_256_COLOR", bit_str(color));
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use super::*;

    #[test]
    fn test_is_256_term() {
        assert_eq!(is_256_term(OsString::from("cygwin")), true);
        assert_eq!(is_256_term(OsString::from("gnome-256color")), true);
        assert_eq!(is_256_term(OsString::from("putty")), true);
        assert_eq!(is_256_term(OsString::from("screen-256color")), true);
        assert_eq!(is_256_term(OsString::from("xterm-256color")), true);
        assert_eq!(is_256_term(OsString::from("linux")), false);
        assert_eq!(is_256_term(OsString::from("")), false);
    }

    #[test]
    fn test_is_256_colorterm() {
        assert_eq!(is_256_colorterm(OsString::from("gnome-terminal")), true);
        assert_eq!(is_256_colorterm(OsString::from("rxvt-xpm")), true);
        assert_eq!(is_256_colorterm(OsString::from("xfce4-terminal")), true);
        assert_eq!(is_256_colorterm(OsString::from("unknown")), false);
        assert_eq!(is_256_term(OsString::from("")), false);
    }
}
