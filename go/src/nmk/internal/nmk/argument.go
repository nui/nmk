package nmk

import (
	"fmt"
	"github.com/urfave/cli"
)

type Arg struct {
	Force256color   bool
	Force8color     bool
	Socket          string
	Login           bool
	Unicode         bool
	ForceUnicode    bool
	DetachOnDestroy bool
	AutoFix         bool
	Inception       bool
	Debug           bool
}

func GetFlagArg(unicodeName string) ([]cli.Flag, *Arg) {
	arg := Arg{}
	flags := []cli.Flag{
		cli.BoolFlag{
			Name:        "2",
			Usage:       "Assume the terminal supports 256 colours",
			Destination: &arg.Force256color,
		},
		cli.BoolFlag{
			Name:        "8",
			Usage:       "Assume the terminal supports 8 colours",
			Destination: &arg.Force8color,
		},
		cli.StringFlag{
			Name:        "socket, L",
			Usage:       "use a different tmux `socket` name",
			Value:       "nmk",
			Destination: &arg.Socket,
		},
		cli.BoolFlag{
			Name:        "login, l",
			Usage:       "start a login shell",
			Destination: &arg.Login,
		},
		cli.BoolFlag{
			Name:        "unicode, u",
			Usage:       fmt.Sprintf("export LANG=%s", unicodeName),
			Destination: &arg.Unicode,
		},
		cli.BoolFlag{
			Name:        "force-unicode",
			Usage:       fmt.Sprintf("export LC_ALL=%s", unicodeName),
			Destination: &arg.ForceUnicode,
		},
		cli.BoolFlag{
			Name:        "detach-on-destroy",
			Usage:       "detach the client when the session is destroyed",
			Destination: &arg.DetachOnDestroy,
		},
		cli.BoolTFlag{
			Name:        "no-autofix",
			Usage:       "disable automatically fix",
			Destination: &arg.AutoFix,
		},
		cli.BoolFlag{
			Name:        "inception",
			Usage:       "allow nested tmux sessions",
			Destination: &arg.Inception,
		},
		cli.BoolFlag{
			Name:        "debug, d",
			Usage:       "print debug log",
			Destination: &arg.Debug,
		},
	}
	return flags, &arg
}
