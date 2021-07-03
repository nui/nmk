use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use indoc::indoc;

use crate::config::on_off;
use crate::consts::env::NMK_HOME;
use crate::platform::is_mac;

use super::version::Version;

const COPY_MODE: &str = "copy-mode -u";
const COPY_MODE_BOTTOM_EXIT: &str = "copy-mode -eu";
const CWD: &str = "#{pane_current_path}";
const F12_TABLE: &str = "F12";
const LAST_SESSION: &str = "switch-client -l";
const NEXT_PANE: &str = r#"select-pane -t :.+ \; display-panes"#;
const NO_ENTER_COPY_MODE: &str = r##"#{?pane_in_mode,1,}#{?alternate_on,1,}"##;

pub fn render(w: &mut dyn Write, c: &Context, v: Version) -> io::Result<()> {
    writeln!(w, "# Tmux {} configuration", v)?;
    section(w, c, "Tmux Options", render_options)?;
    section(w, c, "Prefix Keys", |w, _| {
        writeln!(w, "bind-key -r C-b send-prefix")
    })?;
    writeln!(w, "bind-key -r o {}", NEXT_PANE)?;
    writeln!(w, "bind-key -r C-o rotate-window")?;
    writeln!(w, "bind-key C-c command-prompt")?;
    writeln!(w, "bind-key C-l {}", LAST_SESSION)?;
    w.write_all("bind-key C-t display-message '#{pane_tty}'\n".as_bytes())?;
    section(w, c, "Function Key Binding", |w, _| {
        for n in 1..=12 {
            writeln!(w, "bind-key -n S-F{n} send-keys F{n}", n = n)?;
        }
        writeln!(w, "bind-key -n F1 {}", NEXT_PANE)?;
        writeln!(w, "bind-key -n F2 last-window")?;
        writeln!(w, "bind-key -n F3 previous-window")?;
        writeln!(w, "bind-key -n F4 next-window")?;
        writeln!(w, "bind-key -n F5 resize-pane -Z")?;
        writeln!(w, "bind-key -n F6 {}", choose_tree(v))?;
        writeln!(w, "bind-key -n F8 switch-client -n")
    })?;
    section(w, c, "F12 Key Table", |w, _| {
        writeln!(w, "bind-key F12 send-keys F12")?;
        writeln!(w, "bind-key -n F12 switch-client -T {}", F12_TABLE)?;
        for n in 1..=9 {
            writeln!(
                w,
                "bind-key -T {} {n} select-window -t {n}",
                F12_TABLE,
                n = n
            )?;
        }
        for n in 1..=11 {
            writeln!(w, "bind-key -T {} F{n} send-keys F{n}", F12_TABLE, n = n)?;
        }
        writeln!(w, "bind-key -T {} F12 detach-client", F12_TABLE)?;
        writeln!(w, "bind-key -T {} -r Space next-layout", F12_TABLE)
    })?;
    section(w, c, "Pane Current Path", pane_current_path)?;
    section(w, c, "Copy Mode", |w, _| {
        writeln!(w, "bind-key C-u {}", COPY_MODE)?;
        copy_to_system_clipboard(w)?;
        // PageUp and PageDown special behaviors
        //  If the condition is match, PageUp should enter copy mode
        //  see https://www.reddit.com/r/tmux/comments/3paqoi/tmux_21_has_been_released/
        writeln!(
            w,
            r#"bind-key -T root PageUp if-shell -F "{}" "send-keys PageUp" "{}""#,
            NO_ENTER_COPY_MODE, COPY_MODE_BOTTOM_EXIT
        )?;
        half_pageup_pagedown(w)
    })?;
    // Colors
    section(w, c, "Colors", |w, c| {
        let color_config = if c.support_256_color {
            include_str!("256color.conf")
        } else {
            include_str!("8color.conf")
        };
        writeln!(w, "{}", color_config)
    })
}

fn render_options(w: &mut dyn Write, c: &Context) -> io::Result<()> {
    let options = indoc! {r##"
        set-option -g base-index 0
        set-option -g display-time 1200
        set-option -g history-limit 2500
        set-option -g status-keys emacs
        set-option -g status-left-length 20
        set-option -g status-right-length 60
        set-option -g status-right "#{?client_prefix,^B ,}'#[fg=colour51]#{=40:pane_title}#[default]' %H:%M %Z %a, %d"
        set-window-option -g mode-keys vi
    "##};
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
        on_off(c.detach_on_destroy)
    )?;
    writeln!(
        w,
        r#"set-option -g history-file "${}/.tmux_history""#,
        NMK_HOME
    )
}

fn section<F>(w: &mut dyn Write, c: &Context, name: &str, f: F) -> io::Result<()>
where
    F: FnOnce(&mut dyn Write, &Context) -> io::Result<()>,
{
    write_start_section(w, name)?;
    f(w, c)?;
    write_end_section(w, name)
}

fn write_start_section(c: &mut dyn Write, name: &str) -> io::Result<()> {
    // we need string here to get correct length
    let label = format!(" Start {} ", name);
    writeln!(c, "# {:-^118}", label)
}

fn write_end_section(c: &mut dyn Write, name: &str) -> io::Result<()> {
    // we need string here to get correct length
    let label = format!(" End {} ", name);
    writeln!(c, "# {:-^118}", label)
}

fn pane_current_path(w: &mut dyn Write, _: &Context) -> io::Result<()> {
    let key_binding = &[
        ("%", "split-window -h "),
        ("|", "split-window -h "),
        ("_", "split-window"),
        ("c", "new-window"),
        (r#"'"'"#, "split-window"),
    ];
    for (key, binding) in key_binding {
        writeln!(w, "bind-key {} {} -c '{}'", key, binding, CWD)?;
    }
    writeln!(
        w,
        r#"bind-key C command-prompt "new-session -c '{}' -s '%%'""#,
        CWD
    )
}

fn choose_tree(v: Version) -> String {
    let mut vec = Vec::with_capacity(4);
    vec.extend_from_slice(&["choose-tree", "-s"]);
    if v >= Version::V27 {
        vec.push("-Z");
    }
    vec.join(" ")
}

fn is_system_clipboard_available() -> bool {
    let mut cmd = Command::new("xclip");
    cmd.arg("-o").stdout(Stdio::null()).stderr(Stdio::null());
    cmd.output().map_or(false, |output| output.status.success())
}

fn copy_to_system_clipboard(w: &mut dyn Write) -> io::Result<()> {
    fn write_bind_config(w: &mut dyn Write, cmd: &str) -> io::Result<()> {
        writeln!(
            w,
            r#"bind-key -T copy-mode-vi y send-keys -X copy-pipe-and-cancel "{}""#,
            cmd
        )
    }
    if is_mac() {
        write_bind_config(w, "pbcopy")?;
    } else if is_system_clipboard_available() {
        write_bind_config(w, "xclip -selection clipboard")?;
    }
    Ok(())
}

fn half_pageup_pagedown(w: &mut dyn Write) -> io::Result<()> {
    let key_binding = &[("PageUp", "halfpage-up"), ("PageDown", "halfpage-down")];
    key_binding.iter().try_for_each(|(key, binding)| {
        writeln!(
            w,
            "bind-key -T copy-mode-vi {} send-keys -X {}",
            key, binding
        )
    })
}

pub struct Context {
    pub detach_on_destroy: bool,
    pub support_256_color: bool,
    pub default_shell: PathBuf,
    pub default_term: String,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            detach_on_destroy: false,
            support_256_color: false,
            default_shell: PathBuf::from("/bin/zsh"),
            default_term: String::from("screen"),
        }
    }
}
