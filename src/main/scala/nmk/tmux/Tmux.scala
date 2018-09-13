package nmk.tmux

import javax.inject.{Inject, Singleton}
import nmk.tmux.Version._

import scala.collection.mutable.ListBuffer

@Singleton
class Tmux @Inject()() {

  implicit class StringWithToConfig(s: String) {
    def toConfig: String = s.stripMargin.stripPrefix("\n").stripSuffix("\n")
  }

  private val Table = "F12"
  private val TmuxSettingEnv = Seq(
    "NMK_TMUX_256_COLOR",
    "NMK_TMUX_DEFAULT_SHELL",
    "NMK_TMUX_DEFAULT_TERMINAL",
    "NMK_TMUX_DETACH_ON_DESTROY",
    "NMK_TMUX_HISTORY",
  )
  private val CopyMode = "copy-mode -eu"
  private val Cwd = "#{pane_current_path}"
  private val NextPane = """select-pane -t :.+ \; display-panes"""
  private val NoEnterCopyMode = "#{?pane_in_mode,1,}#{?alternate_on,1,}"
  private val LastSession = "switch-client -l"
  private val NextSession = "switch-client -n"
  private val PrevSession = "switch-client -p"

  def render(implicit version: Version): String = {
    val r = ListBuffer.empty[String]
    r += s"# tmux $version configuration"
    section("tmux option", r)(_ += options)
    section("prefix key", r) { r =>
      r += "unbind-key C-b"
      r += "bind-key -r C-b send-prefix"
      r += "bind-key -r b " + NextPane
    }
    r += s"bind-key C-c command-prompt"
    r += s"bind-key C-l $LastSession"
    r += s"bind-key C-t display-message '#{pane_tty}'"
    section("function key binding", r) { r =>
      r += s"bind-key -n F1 $NextPane"
      r += s"bind-key -n F2 last-window"
      r += s"bind-key -n F3 previous-window"
      r += s"bind-key -n F4 next-window"
      r += s"bind-key -n F5 resize-pane -Z"
      r += s"bind-key -n F6 $chooseTree"
      r += s"bind-key -n S-F2 $LastSession"
      r += s"bind-key -n S-F3 $PrevSession"
      r += s"bind-key -n S-F4 $NextSession"
    }
    section("F12 key table", r) { r =>
      r += s"bind-key F12 send-keys F12"
      r += s"bind-key -n F12 switch-client -T $Table"
      r ++= 1 to 11 map { n => s"bind-key -T $Table F$n send-keys F$n" }
      r += s"bind-key -T $Table F12 detach-client"
      r += s"bind-key -T $Table -r Space next-layout"
      r ++= 1 to 9 map { n => s"bind-key -T $Table $n select-window -t $n" }
    }
    section("pane_current_path", r)(_ ++= paneCurrentPath)
    section("copy mode", r) { r =>
      r += s"bind-key C-u $CopyMode"
      r += copyToSystemClipboard
      // Fix mouse scrolling in 2.1 and later, https://github.com/tmux/tmux/issues/145
      r +=
        s"""
           |bind-key -T root WheelUpPane if-shell -F "#{mouse_any_flag}" "send-keys -M" "if-shell -F '$NoEnterCopyMode' 'send-keys -M' '$CopyMode'"
           |""".toConfig
      // PageUp and PageDown special behaviors
      //  If the condition is match, PageUp should enter copy mode
      //  see https://www.reddit.com/r/tmux/comments/3paqoi/tmux_21_has_been_released/
      r +=
        s"""
           |bind-key -T root PageUp if-shell -F "$NoEnterCopyMode" "send-keys PageUp" "$CopyMode"
           |""".toConfig
      r ++= halfPageUpDown
    }
    // Colors
    r +=
      """
        |if-shell '[ x$NMK_TMUX_256_COLOR = x1 ]' 'source-file $NMK_DIR/tmux/256color.conf' 'source-file $NMK_DIR/tmux/8color.conf'
        |""".toConfig
    // Unset temporary environment variables that were used during tmux initialization
    r ++= unsetTmuxSettingEnv
    // Render final config
    r.flatMap(_.split('\n')).mkString("\n") + "\n"
  }

  private def chooseTree(implicit version: Version): String = {
    val list = ListBuffer.empty[String]
    list += "choose-tree"
    if (version >= V26)
      list += "-s"
    if (version >= V27)
      list += "-Z"
    list.mkString(" ")
  }

  private def options = {
    """
      |set-option -g base-index 1
      |set-option -g default-shell "$NMK_TMUX_DEFAULT_SHELL"
      |set-option -g default-terminal "$NMK_TMUX_DEFAULT_TERMINAL"
      |set-option -g detach-on-destroy "$NMK_TMUX_DETACH_ON_DESTROY"
      |set-option -g display-time 1200
      |set-option -g history-file "$NMK_TMUX_HISTORY"
      |set-option -g history-limit 2500
      |set-option -g status-keys emacs
      |set-option -g status-left-length 20
      |set-option -g status-right-length 60
      |set-option -g status-right "#{?client_prefix,^B ,}'#[fg=colour51]#{=40:pane_title}#[default]' %H:%M %Z %a, %d"
      |set-window-option -g mode-keys vi
      |""".toConfig
  }

  private def copyToSystemClipboard(implicit version: Version) = {
    val copyToClipboard = "xclip -selection clipboard"
    val head = "if-shell 'xclip -o > /dev/null 2>&1'"
    val tail = version match {
      case x if x >= V24 =>
        s"""
           |'bind-key -T copy-mode-vi y send-keys -X copy-pipe-and-cancel "$copyToClipboard"'
           |""".toConfig
      case _ =>
        s"""
           |'bind-key -t vi-copy y copy-pipe "$copyToClipboard"'
           |""".toConfig
    }
    s"$head $tail"
  }

  private def paneCurrentPath = {
    val r = ListBuffer.empty[String]
    r ++= Map(
      "'\"'" -> "split-window",
      "_" -> "split-window",
      "%" -> "split-window -h ",
      "|" -> "split-window -h ",
      "c" -> "new-window"
    ) flatMap { case (k, v) =>
      Seq(
        s"unbind-key $k",
        s"bind-key $k $v -c '$Cwd'"
      )
    }
    r +=
      s"""
         |bind-key C command-prompt "new-session -c '$Cwd' -s '%%'"
         |""".toConfig
    r
  }

  private def halfPageUpDown(implicit version: Version) = {
    Map(
      "PageUp" -> "halfpage-up",
      "PageDown" -> "halfpage-down"
    ) flatMap { case (k, v) =>
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

  private def unsetTmuxSettingEnv = TmuxSettingEnv.map("set-environment -gr " + _)

  private def section(name: String, r: ListBuffer[String])
                     (block: ListBuffer[String] => Unit): Unit = {
    val buffer = ListBuffer.empty[String]
    block(buffer)
    r += lineComment(name, filler = ">")
    r ++= buffer
    r += lineComment(name, filler = "<")
  }

  private def lineComment(s: String, length: Int = 80, filler: String = "=") = {
    val result = new StringBuilder(s" $s ")
    val prefix = "# "
    while (result.length < length - prefix.length) {
      result.insert(0, filler)
      result.append(filler)
    }
    result.insert(0, prefix)
    result.toString.substring(0, length)
  }
}
