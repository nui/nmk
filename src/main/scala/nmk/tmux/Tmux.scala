package nmk.tmux

import javax.inject.{Inject, Singleton}
import nmk.tmux.Version._

import scala.collection.mutable.ListBuffer

@Singleton
class Tmux @Inject()() {
  private val Table = "fast"
  private val TmpEnvs = Seq(
    "NMK_TMUX_256_COLOR",
    "NMK_TMUX_DEFAULT_SHELL",
    "NMK_TMUX_DEFAULT_TERMINAL",
    "NMK_TMUX_DETACH_ON_DESTROY",
    "NMK_TMUX_HISTORY",
  )

  def render(implicit version: Version): String = {
    val r = ListBuffer.empty[String]
    r += s"# tmux $version"
    r += s"bind-key -n F1 $nextPane"
    r += s"bind-key -n F2 last-window"
    r += s"bind-key -n F3 switch-client -n"
    r += s"bind-key -n F4 $chooseTree"
    r += s"bind-key -n F5 resize-pane -Z"
    r += s"bind-key -n F12 switch-client -T $Table"
    r += s"bind-key -T $Table F12 detach-client"
    r += s"bind-key -T $Table -r Space next-layout"
    r ++= (for (i <- 1 to 9) yield s"bind-key -T $Table $i select-window -t $i")
    //  Use Shift-Fx or <prefix> Fx to send Fx key
    r ++= (1 to 12) flatMap { n =>
      Seq(
        s"bind-key    F$n   send-keys F$n",
        s"bind-key -n S-F$n send-keys F$n"
      )
    }
    r += "unbind-key  C-b"
    r += "bind-key -r C-b send-prefix"
    r += s"bind-key -r b   $nextPane"
    r += "bind-key    C-c command-prompt"
    r += """bind-key    C   command-prompt "new-session -c '#{pane_current_path}' -s '%%'""""
    r += tmuxOptions
    r += copyToSystemClipboard
    r += """set-option -g status-right "#{?client_prefix,^B ,}'#[fg=colour51]#{=40:pane_title}#[default]' %H:%M %Z %a, %d""""
    r += newPaneWindowCurrentDirectory
    r += "bind-key C-l switch-client -l"
    // ---- COPY-MODE BINDING ----
    // -- C-u to enter copy-mode --
    r += "bind-key C-u copy-mode -eu"
    r ++= pageUpDown
    // Fix mouse scrolling in 2.1 and later
    // Credit https://github.com/tmux/tmux/issues/145
    r +=
      """bind-key -T root WheelUpPane if-shell -Ft= "#{mouse_any_flag}" "send-keys -M" "if-shell -Ft= '#{pane_in_mode}' 'send-keys -M' 'copy-mode -e'"
        |""".stripMargin
    // PageUp and PageDown special behaviors
    //  If the condition is match, PageUp should enter copy mode
    //  see https://www.reddit.com/r/tmux/comments/3paqoi/tmux_21_has_been_released/
    r +=
      """bind-key -T root PageUp if-shell -F "#{?pane_in_mode,1,}#{?alternate_on,1,}" "send-keys PageUp" "copy-mode -eu"
        |""".stripMargin
    // Colors
    r +=
      """if-shell 'zsh -c "[[ $NMK_TMUX_256_COLOR == 1 ]]"' 'source-file $NMK_DIR/tmux/256color.conf' 'source-file $NMK_DIR/tmux/8color.conf'
        |""".stripMargin
    // Unset temporary environment variables that were used during tmux initialization
    r ++= unsetTempEnvs
    // make final config
    r.flatMap(_.split('\n')).filterNot(_.trim.isEmpty).mkString("\n") + "\n"
  }

  private def chooseTree(implicit version: Version): String = {
    val options = version match {
      case v if v >= V27 => "-sZ"
      case _ => "-s"
    }
    "choose-tree " + options
  }

  private def tmuxOptions = {
    """set-option -g base-index 1
      |set-option -g     default-shell "$NMK_TMUX_DEFAULT_SHELL"
      |set-option -g  default-terminal "$NMK_TMUX_DEFAULT_TERMINAL"
      |set-option -g detach-on-destroy "$NMK_TMUX_DETACH_ON_DESTROY"
      |set-option -g display-time 1200
      |set-option -g history-limit 2500
      |set-option -g status-keys emacs
      |set-option -g status-right-length 60
      |set-window-option -g mode-keys vi
      |set-option -g history-file "$NMK_TMUX_HISTORY"
      |""".stripMargin
  }

  private def nextPane = """select-pane -t :.+ \; display-panes"""

  private def copyToSystemClipboard(implicit version: Version) = {
    val head = "if-shell 'xclip -o > /dev/null 2>&1'"
    val tail = version match {
      case x if x >= V24 =>
        """'bind-key -T copy-mode-vi y send-keys -X copy-pipe-and-cancel "xclip -selection clipboard"'"""
      case _ =>
        """'bind-key -t vi-copy y copy-pipe "xclip -selection clipboard"'"""
    }
    s"$head $tail"
  }

  private def newPaneWindowCurrentDirectory = {
    """unbind-key '"'; bind-key '"' split-window    -c "#{pane_current_path}"
      |unbind-key %;   bind-key %   split-window -h -c "#{pane_current_path}"
      |unbind-key c;   bind-key c   new-window      -c "#{pane_current_path}"
      |""".stripMargin
  }

  private def pageUpDown(implicit version: Version) = {
    val keyMap = Map(
      "PageUp" -> "halfpage-up",
      "PageDown" -> "halfpage-down"
    )
    keyMap flatMap { case (k, v) =>
      version match {
        case x if x >= V24 => Seq(
          s"unbind-key -T copy-mode-vi $k",
          s"bind-key -T copy-mode-vi $k send-keys -X $v"
        )
        case _ => Seq(
          s"unbind-key -t vi-copy $k",
          s"bind-key -t vi-copy $k $v"
        )
      }
    }
  }

  private def unsetTempEnvs = {
    TmpEnvs.map(env => s"set-environment -gr $env")
  }
}
