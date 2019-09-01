package main

import (
	"fmt"
	"github.com/sirupsen/logrus"
	"github.com/urfave/cli"
	"nmk/internal/nmk"
	"nmk/internal/terminal"
	"nmk/internal/tmux"
	"nmk/internal/zsh"
	"nmk/pkg/pathstring"
	"os"
	"os/exec"
	"path"
	"path/filepath"
	"strconv"
	"strings"
)
import "time"

func Setenv(key, value string) {
	if err := os.Setenv(key, value); err != nil {
		logrus.Fatal(err)
	} else {
		logrus.Debugf("export %s=%s", key, value)
	}
}

func AddLocalLibrary(nmkDir string) {
	const LD = "LD_LIBRARY_PATH"
	localLibDir := filepath.Join(nmkDir, "local", "lib")

	if _, err := os.Stat(localLibDir); err == nil {
		before := os.Getenv(LD)
		logrus.Debugf("before %s=%s", LD, before)
		sp := pathstring.Parse(before)
		sp.Prepend(localLibDir)
		after := sp.Make()
		logrus.Debugf("after %s=%s", LD, after)
		Setenv(LD, after)
	}
}

func RunGetProcessId() int {
	cmd := exec.Command("sh", "-c", "echo $$")
	bytes, err := cmd.Output()
	if err != nil {
		logrus.Fatal(err)
	}

	pid, err := strconv.Atoi(strings.TrimSpace(string(bytes)))

	if err != nil {
		logrus.Fatal(err)
	}
	return pid
}

func CheckDependencies() {
	if _, err := exec.LookPath("zsh"); err != nil {
		logrus.Fatal("Zsh not found")
	}
	tmux.EnsureExist()
}

func SetupPath(arg *nmk.Arg, nmkDir string) {
	const PATH = "PATH"
	sp := pathstring.Parse(os.Getenv(PATH))
	sp.Prepend(path.Join(nmkDir, "local", "bin"))
	sp.Prepend(path.Join(nmkDir, "bin"))
	nextPath := sp.Make()
	Setenv(PATH, nextPath)
	if arg.Debug {
		for i, v := range filepath.SplitList(nextPath) {
			logrus.Debugf("%s[%d]=%s", PATH, i, v)
		}
	}
}

func SetupTerminal(arg *nmk.Arg) {
	term := "screen"
	flag := "0"
	if terminal.Support256Color(arg) {
		term = "screen-256color"
		flag = "1"
	}
	Setenv("NMK_TMUX_DEFAULT_TERMINAL", term)
	Setenv("NMK_TMUX_256_COLOR", flag)
}

func SetupEnvironment(arg *nmk.Arg, nmkDir string, tmuxVersion string) {
	initVim := path.Join(nmkDir, "vim", "init.vim")
	zdotdir := path.Join(nmkDir, "zsh")
	Setenv("NMK_DIR", nmkDir)
	if zsh, err := exec.LookPath("zsh"); err != nil {
		logrus.Fatal(err)
	} else {
		Setenv("NMK_TMUX_DEFAULT_SHELL", zsh)
	}
	detachOnDestroy := "off"
	if arg.DetachOnDestroy {
		detachOnDestroy = "on"
	}
	Setenv("NMK_TMUX_DETACH_ON_DESTROY", detachOnDestroy)
	tmuxHistoryFile := path.Join(nmkDir, "tmux", ".tmux_history")
	Setenv("NMK_TMUX_HISTORY", tmuxHistoryFile)
	Setenv("NMK_TMUX_VERSION", tmuxVersion)
	initVimSource := fmt.Sprintf("source %s", strings.Replace(initVim, " ", "\\ ", 1))
	Setenv("VIMINIT", initVimSource)
	Setenv("ZDOTDIR", zdotdir)

	_ = os.Unsetenv("VIRTUAL_ENV")

	if exe, err := os.Executable(); err != nil {
		logrus.Fatal(err)
	} else {
		Setenv("NMK_BIN", exe)
	}
}

func SetupZsh(arg *nmk.Arg, nmkDir string) {
	globalRcs := "0"
	if zsh.UseGlobalRcs(arg, nmkDir) {
		globalRcs = "1"
	} else {
		logrus.Debug("ignore zsh global rcs")
	}
	Setenv("NMK_ZSH_GLOBAL_RCS", globalRcs)
}

func SetupPreferEditor() {
	const Editor = "EDITOR"
	if _, found := os.LookupEnv(Editor); !found {
		found := false
		prog := ""
		for _, v := range []string{"nvim", "vim"} {
			if _, err := exec.LookPath(v); err == nil {
				found = true
				prog = v
				break
			}
		}
		if found {
			Setenv(Editor, prog)
			logrus.Debugf("using %s as prefer editor", prog)
		}
	}
}

func ClearTempEnv(nmkDir string) {
	config := nmk.ReadConfig(nmkDir)
	for _, v := range config.TmuxSettingEnvs {
		_ = os.Unsetenv(v)
	}
}

func main() {
	startTime := time.Now()

	app := cli.NewApp()
	app.Usage = "An entrypoint for nmk"
	app.Version = "1.0"

	var arg *nmk.Arg
	app.Flags, arg = nmk.GetFlagArg()

	app.Action = func(c *cli.Context) error {
		startPid := 0

		if arg.Debug {
			startPid = RunGetProcessId()
			logrus.SetLevel(logrus.DebugLevel)
		}

		if arg.Ssh {
			// TODO: Display MOTD
		}

		nmkDir := nmk.FindNmkDir()
		tmuxDir := filepath.Join(nmkDir, "tmux")

		AddLocalLibrary(nmkDir)
		SetupPath(arg, nmkDir)
		CheckDependencies()
		tmuxVersion := tmux.GetVersion()
		logrus.Debugf("using tmux %s", tmuxVersion)

		SetupTerminal(arg)
		SetupEnvironment(arg, nmkDir, tmuxVersion)
		SetupZsh(arg, nmkDir)
		SetupPreferEditor()

		endPid := 0
		if arg.Debug {
			endPid = RunGetProcessId()
			logrus.Debugf("created %d processes during initialization", endPid-startPid-1)
		}

		tmuxConf := tmux.GetConf(tmuxVersion, tmuxDir)

		if arg.Login {
			ClearTempEnv(nmkDir)
			tmux.LoginShell(arg, tmuxConf, startTime)
		} else {
			tmux.Exec(arg, tmuxConf, startTime, c.Args())
		}

		return nil
	}

	if err := app.Run(os.Args); err != nil {
		logrus.Fatal(err)
	}
}
