package tmux

import (
	"fmt"
	"github.com/sirupsen/logrus"
	"github.com/urfave/cli"
	"nmk/internal/nmk"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"syscall"
	"time"
)

func GetVersion() string {
	cmd := exec.Command("tmux", "-V")
	byteOutput, err := cmd.Output()
	if err != nil {
		logrus.Fatal(err)
	}
	output := string(byteOutput)
	output = strings.TrimSpace(output)
	sp := strings.Split(output, " ")
	return sp[1]
}

func GetConf(version string, tmuxDir string) string {
	versionName := fmt.Sprintf("%s.conf", version)
	conf := filepath.Join(tmuxDir, versionName)
	if _, err := os.Stat(conf); err != nil {
		if os.IsNotExist(err) {
			logrus.Fatal(fmt.Sprintf("tmux %s is unsupported", version))
		} else {
			logrus.Fatal(err)
		}
	}
	return conf
}

func IsServerRunning(socket string) bool {
	cmd := exec.Command("tmux", "-L", socket, "list-sessions")
	err := cmd.Run()
	return err == nil
}

func EnsureExist() {
	if _, err := exec.LookPath("tmux"); err != nil {
		logrus.Fatal("Tmux not found")
	}
}

func FindTmux() string {
	prog, err := exec.LookPath("tmux")
	if err != nil {
		logrus.Fatal("Tmux not found")
	}
	return prog
}

func PrintUsageTime(arg *nmk.Arg, startTime time.Time) {
	t := time.Now().Sub(startTime)
	if arg.Usage {
		fmt.Printf("%d\n", t / 1E6)
	} else {
		logrus.Debugf("usage time %s", t)
	}
}

func LoginShell(arg *nmk.Arg, conf string, startTime time.Time) {
	execArgs := []string{"tmux", "-L", arg.Socket}
	if arg.Force256color {
		execArgs = append(execArgs, "-2")
	}
	execArgs = append(execArgs, "-f", conf, "-c", "exec zsh --login")
	PrintUsageTime(arg, startTime)
	logrus.Debugf("exec args: %s", strings.Join(execArgs, " "))
	if err := syscall.Exec(FindTmux(), execArgs, os.Environ()); err != nil {
		logrus.Fatal(err)
	}
}

func Exec(arg *nmk.Arg, conf string, startTime time.Time, args cli.Args) {
	execArgs := []string{"tmux", "-L", arg.Socket}
	if arg.Force256color {
		execArgs = append(execArgs, "-2")
	}
	tmuxArgs := make([]string, 0, len(args))
	tmuxArgs = append(tmuxArgs, args...)
	if len(tmuxArgs) > 0 && tmuxArgs[0] == "--" {
		tmuxArgs = tmuxArgs[1:]
	}
	if IsServerRunning(arg.Socket) {
		if len(tmuxArgs) > 0 {
			execArgs = append(execArgs, tmuxArgs...)
		} else {
			if _, found := os.LookupEnv("TMUX"); found && !arg.Inception {
				logrus.Fatal("add --inception to allow nested tmux sessions")
			}
			execArgs = append(execArgs, "attach")
		}
	} else {
		execArgs = append(execArgs, "-f", conf)
		execArgs = append(execArgs, tmuxArgs...)
	}
	PrintUsageTime(arg, startTime)
	envs := os.Environ()
	logrus.Debugf("exec args: %s", strings.Join(execArgs, " "))
	if err := syscall.Exec(FindTmux(), execArgs, envs); err != nil {
		logrus.Fatal(err)
	}
}
