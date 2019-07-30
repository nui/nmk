use std::env;
use std::ffi::OsString;

use crate::argument::Argument;
use crate::container;
use crate::core::*;

fn is_vec_contains_term(vec: Vec<&str>, term: Option<OsString>) -> bool {
    term.and_then(|x| x.into_string().ok())
        .map_or(false, |s| vec.contains(&s.as_str()))
}

fn is_256_term(term: Option<OsString>) -> bool {
    let terms = vec![
        "cygwin",
        "gnome-256color",
        "putty",
        "screen-256color",
        "xterm-256color",
    ];
    is_vec_contains_term(terms, term)
}

fn is_256_colorterm(term: Option<OsString>) -> bool {
    let terms = vec!["gnome-terminal", "rxvt-xpm", "xfce4-terminal"];
    is_vec_contains_term(terms, term)
}

fn support_256_color(arg: &Argument) -> bool {
    arg.force256color
        || is_256_term(env::var_os("TERM"))
        || is_256_colorterm(env::var_os("COLORTERM"))
        || (arg.autofix && container::detect_container())
}

pub fn setup(arg: &Argument) {
    let color = support_256_color(arg);
    set_env("NMK_TMUX_DEFAULT_TERMINAL", if color { "screen-256color" } else { "screen" });
    set_env("NMK_TMUX_256_COLOR", one_hot!(color));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(i: &str) -> Option<OsString> {
        Some(OsString::from(i))
    }

    #[test]
    fn test_is_256_term() {
        assert!(is_256_term(input("cygwin")));
        assert!(is_256_term(input("gnome-256color")));
        assert!(is_256_term(input("putty")));
        assert!(is_256_term(input("screen-256color")));
        assert!(is_256_term(input("xterm-256color")));
        assert!(!is_256_term(input("linux")));
        assert!(!is_256_term(input("")));
        assert!(!is_256_term(None));
    }

    #[test]
    fn test_is_256_colorterm() {
        assert!(is_256_colorterm(input("gnome-terminal")));
        assert!(is_256_colorterm(input("rxvt-xpm")));
        assert!(is_256_colorterm(input("xfce4-terminal")));
        assert!(!is_256_colorterm(input("unknown")));
        assert!(!is_256_colorterm(input("")));
        assert!(!is_256_colorterm(None));
    }
}
