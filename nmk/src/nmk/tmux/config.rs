use std::io;
use std::io::Write;
use std::path::PathBuf;

use indoc::indoc;

use super::version::Version;

const TABLE: &str = "F12";
const COPY_MODE: &str = "copy-mode -u";
const COPY_MODE_BOTTOM_EXIT: &str = "copy-mode -eu";
const CWD: &str = "#{pane_current_path}";
const NEXT_PANE: &str = r###"select-pane -t :.+ \; display-panes"###;
const NO_ENTER_COPY_MODE: &str = r###"#{?pane_in_mode,1,}#{?alternate_on,1,}"###;
const LAST_SESSION: &str = "switch-client -l";
#[allow(dead_code)]
const NEXT_SESSION: &str = "switch-client -n";
#[allow(dead_code)]
const PREV_SESSION: &str = "switch-client -p";

pub fn render(w: &mut dyn Write, c: &Context, v: Version) -> Result<(), io::Error> {
    writeln!(w, "# tmux {} configuration", v.as_str())?;
    section(w, c, "tmux options", render_options)?;
    section(w, c, "prefix keys", |w, _| {
        writeln!(w, "unbind-key C-b")?;
        writeln!(w, "bind-key -r C-b send-prefix")?;
        writeln!(w, "bind-key -r b {}", NEXT_PANE)
    })?;
    writeln!(w, "bind-key C-c command-prompt")?;
    writeln!(w, "bind-key C-l {}", LAST_SESSION)?;
    writeln!(w, "{}", "bind-key C-t display-message '#{pane_tty}'")?;
    section(w, c, "function key binding", |w, _| {
        writeln!(w, "bind-key -n F1 {}", NEXT_PANE)?;
        writeln!(w, "bind-key -n F2 last-window")?;
        writeln!(w, "bind-key -n F3 previous-window")?;
        writeln!(w, "bind-key -n F4 next-window")?;
        writeln!(w, "bind-key -n F5 resize-pane -Z")?;
        writeln!(w, "bind-key -n F6 {}", choose_tree(v))?;
        writeln!(w, "bind-key -n F8 switch-client -n")?;
        for n in 1..=12 {
            writeln!(w, "bind-key -n S-F{n} send-keys F{n}", n = n)?;
        }
        Ok(())
    })?;
    section(w, c, "F12 Key table", |w, _| {
        writeln!(w, "bind-key F12 send-keys F12")?;
        writeln!(w, "bind-key -n F12 switch-client -T {}", TABLE)?;
        for n in 1..=11 {
            writeln!(w, "bind-key -T {} F{n} send-keys F{n}", TABLE, n = n)?;
        }
        writeln!(w, "bind-key -T {} F12 detach-client", TABLE)?;
        writeln!(w, "bind-key -T {} -r Space next-layout", TABLE)?;
        for n in 1..=9 {
            writeln!(w, "bind-key -T {} {n} select-window -t {n}", TABLE, n = n)?;
        }
        Ok(())
    })?;
    section(w, c, "Pane current path", pane_current_path)?;
    section(w, c, "Copy mode", |w, _| {
        writeln!(w, "bind-key C-u {}", COPY_MODE)?;
        copy_to_system_clipboard(w)?;
        // PageUp and PageDown special behaviors
        //  If the condition is match, PageUp should enter copy mode
        //  see https://www.reddit.com/r/tmux/comments/3paqoi/tmux_21_has_been_released/
        writeln!(
            w,
            r##"bind-key -T root PageUp if-shell -F "{}" "send-keys PageUp" "{}""##,
            NO_ENTER_COPY_MODE, COPY_MODE_BOTTOM_EXIT
        )?;
        half_pageup_pagedown(w)
    })?;
    // Colors
    section(w, c, "colors", |w, c| {
        if c.support_256_color {
            writeln!(w, "{}", include_str!("256color.conf"))
        } else {
            writeln!(w, "{}", include_str!("8color.conf"))
        }
    })
}

fn render_options(w: &mut dyn Write, c: &Context) -> io::Result<()> {
    let options = indoc!(
        r###"
        set-option -g base-index 0
        set-option -g display-time 1200
        set-option -g history-limit 2500
        set-option -g status-keys emacs
        set-option -g status-left-length 20
        set-option -g status-right-length 60
        set-option -g status-right "#{?client_prefix,^B ,}'#[fg=colour51]#{=40:pane_title}#[default]' %H:%M %Z %a, %d"
        set-window-option -g mode-keys vi
    "###
    );
    write!(w, "{}", options)?;
    writeln!(
        w,
        r#"set-option -g default-shell "{}""#,
        c.default_shell.display()
    )?;
    writeln!(w, r#"set-option -g default-terminal "{}""#, c.default_term)?;
    writeln!(
        w,
        r#"set-option -g detach-on-destroy "{}""#,
        on_off!(c.detach_on_destroy)
    )?;
    if let Some(ref path) = c.tmux_history_file {
        writeln!(w, r#"set-option -g history-file "{}""#, path.display())?;
    }
    Ok(())
}

fn section<F>(w: &mut dyn Write, c: &Context, name: &str, mut f: F) -> io::Result<()>
where
    F: FnMut(&mut dyn Write, &Context) -> io::Result<()>,
{
    write_start_section(w, name)?;
    f(w, c)?;
    write_end_section(w, name)
}

fn write_start_section(c: &mut dyn Write, name: &str) -> io::Result<()> {
    let label = format!(" start {} ", name);
    writeln!(c, "# {:-^100}", label)
}

fn write_end_section(c: &mut dyn Write, name: &str) -> io::Result<()> {
    let label = format!(" end {} ", name);
    writeln!(c, "# {:-^100}", label)
}

fn pane_current_path(w: &mut dyn Write, _: &Context) -> io::Result<()> {
    let key_binding = &[
        ("%", "split-window -h "),
        ("|", "split-window -h "),
        ("_", "split-window"),
        ("c", "new-window"),
        ("'\"'", "split-window"),
    ];
    for (key, binding) in key_binding {
        writeln!(w, "unbind-key {}", key)?;
        writeln!(w, "bind-key {} {} -c '{}'", key, binding, CWD)?;
    }
    writeln!(
        w,
        r##"bind-key C command-prompt "new-session -c '{}' -s '%%'""##,
        CWD
    )
}

fn choose_tree(v: Version) -> String {
    let mut vec = vec!["choose-tree"];
    if v >= Version::V26 {
        vec.push("-s");
    }
    if v >= Version::V27 {
        vec.push("-Z");
    }
    vec.join(" ")
}

fn copy_to_system_clipboard(w: &mut dyn Write) -> io::Result<()> {
    let to_system_clipboard = "xclip -selection clipboard";
    writeln!(
        w,
        "{head} {tail}",
        head = "if-shell 'xclip -o > /dev/null 2>&1'",
        tail = format_args!(
            r#"'bind-key -T copy-mode-vi y send-keys -X copy-pipe-and-cancel "{}"'"#,
            to_system_clipboard
        )
    )
}

fn half_pageup_pagedown(w: &mut dyn Write) -> io::Result<()> {
    let key_binding = &[("PageUp", "halfpage-up"), ("PageDown", "halfpage-down")];
    for (key, binding) in key_binding {
        writeln!(w, "unbind-key -T copy-mode-vi {}", key)?;
        writeln!(
            w,
            "bind-key -T copy-mode-vi {} send-keys -X {}",
            key, binding
        )?;
    }
    Ok(())
}

pub struct Context {
    pub detach_on_destroy: bool,
    pub support_256_color: bool,
    pub default_shell: PathBuf,
    pub default_term: String,
    pub tmux_history_file: Option<PathBuf>,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            detach_on_destroy: false,
            support_256_color: false,
            default_shell: PathBuf::from("/bin/zsh"),
            default_term: String::from("screen"),
            tmux_history_file: None,
        }
    }
}
