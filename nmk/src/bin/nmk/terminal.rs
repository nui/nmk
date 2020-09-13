use std::env;
use std::ffi::{OsStr, OsString};
use std::ops::Deref;

use nmk::container;

use crate::cmdline::Opt;

fn slice_contains_term<T: Deref<Target = OsStr>>(slice: &[&str], term: Option<T>) -> bool {
    term.as_deref()
        .and_then(OsStr::to_str)
        .map_or(false, |s| slice.contains(&s))
}

fn is_256_color_term(term: Option<OsString>) -> bool {
    const TERM_LIST: &[&str] = &[
        "cygwin",
        "gnome-256color",
        "putty",
        "screen-256color",
        "xterm-256color",
    ];
    slice_contains_term(TERM_LIST, term)
}

fn is_256_color_colorterm(term: Option<OsString>) -> bool {
    const COLORTERM_LIST: &[&str] = &["gnome-terminal", "rxvt-xpm", "xfce4-terminal"];
    slice_contains_term(COLORTERM_LIST, term)
}

pub fn support_256_color(opt: &Opt) -> bool {
    opt.force_256_color
        || is_256_color_term(env::var_os("TERM"))
        || is_256_color_colorterm(env::var_os("COLORTERM"))
        || container::detect_container()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(i: &str) -> Option<OsString> {
        Some(OsString::from(i))
    }

    #[test]
    fn test_is_256_term() {
        assert!(is_256_color_term(input("cygwin")));
        assert!(is_256_color_term(input("gnome-256color")));
        assert!(is_256_color_term(input("putty")));
        assert!(is_256_color_term(input("screen-256color")));
        assert!(is_256_color_term(input("xterm-256color")));
        assert!(!is_256_color_term(input("linux")));
        assert!(!is_256_color_term(input("")));
        assert!(!is_256_color_term(None));
    }

    #[test]
    fn test_is_256_colorterm() {
        assert!(is_256_color_colorterm(input("gnome-terminal")));
        assert!(is_256_color_colorterm(input("rxvt-xpm")));
        assert!(is_256_color_colorterm(input("xfce4-terminal")));
        assert!(!is_256_color_colorterm(input("unknown")));
        assert!(!is_256_color_colorterm(input("")));
        assert!(!is_256_color_colorterm(None));
    }
}
