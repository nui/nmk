use std::env;

use nmk::container;

fn is_256_color_term<T: AsRef<str>>(term: T) -> bool {
    let term = term.as_ref();
    const TERM_LIST: &[&str] = &[
        "cygwin",
        "gnome-256color",
        "putty",
        "screen-256color",
        "xterm-256color",
    ];
    TERM_LIST.contains(&term)
}

fn is_256_color_colorterm<T: AsRef<str>>(term: T) -> bool {
    let term = term.as_ref();
    const COLORTERM_LIST: &[&str] = &["gnome-terminal", "rxvt-xpm", "xfce4-terminal"];
    COLORTERM_LIST.contains(&term)
}

pub fn support_256_color() -> bool {
    env::var("TERM").iter().any(is_256_color_term)
        || env::var("COLORTERM").iter().any(is_256_color_colorterm)
        || container::is_containerized()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_256_term() {
        assert!(is_256_color_term("cygwin"));
        assert!(is_256_color_term("gnome-256color"));
        assert!(is_256_color_term("putty"));
        assert!(is_256_color_term("screen-256color"));
        assert!(is_256_color_term("xterm-256color"));
        assert!(!is_256_color_term("linux"));
        assert!(!is_256_color_term(""));
    }

    #[test]
    fn test_is_256_colorterm() {
        assert!(is_256_color_colorterm("gnome-terminal"));
        assert!(is_256_color_colorterm("rxvt-xpm"));
        assert!(is_256_color_colorterm("xfce4-terminal"));
        assert!(!is_256_color_colorterm("unknown"));
        assert!(!is_256_color_colorterm(""));
    }
}
