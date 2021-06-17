use std::env;

use nmk::container;

fn is_term_256_color(term: impl AsRef<str>) -> bool {
    let terms = [
        "cygwin",
        "gnome-256color",
        "putty",
        "screen-256color",
        "xterm-256color",
    ];
    terms.contains(&term.as_ref())
}

fn is_colorterm_256_color(term: impl AsRef<str>) -> bool {
    ["gnome-terminal", "rxvt-xpm", "xfce4-terminal"].contains(&term.as_ref())
}

pub fn support_256_color() -> bool {
    let arr = [
        || env::var("TERM").map_or(false, is_term_256_color),
        || env::var("COLORTERM").map_or(false, is_colorterm_256_color),
        || container::is_containerized(),
    ];
    arr.iter().any(|f| f())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_256_term() {
        assert!(is_term_256_color("cygwin"));
        assert!(is_term_256_color("gnome-256color"));
        assert!(is_term_256_color("putty"));
        assert!(is_term_256_color("screen-256color"));
        assert!(is_term_256_color("xterm-256color"));
        assert!(!is_term_256_color("linux"));
        assert!(!is_term_256_color(""));
    }

    #[test]
    fn test_is_256_colorterm() {
        assert!(is_colorterm_256_color("gnome-terminal"));
        assert!(is_colorterm_256_color("rxvt-xpm"));
        assert!(is_colorterm_256_color("xfce4-terminal"));
        assert!(!is_colorterm_256_color("unknown"));
        assert!(!is_colorterm_256_color(""));
    }
}
