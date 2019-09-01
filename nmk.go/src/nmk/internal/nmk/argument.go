package nmk

import (
	"github.com/urfave/cli"
)

type Arg struct {
	Force256color   bool
	Login           bool
	Socket          string
	DetachOnDestroy bool
	AutoFix         bool
	Inception       bool
	Debug           bool
	Usage           bool
	Ssh             bool
}

func GetFlagArg() ([]cli.Flag, *Arg) {
	arg := Arg{}
	flags := []cli.Flag{
		cli.BoolFlag{
			Name:        "2",
			Usage:       "Assume the terminal supports 256 colours",
			Destination: &arg.Force256color,
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
		cli.BoolFlag{
			Name:        "ssh",
			Destination: &arg.Ssh,
		},
		cli.BoolFlag{
			Name:        "usage",
			Usage:       "print usage time",
			Destination: &arg.Usage,
		},
	}
	return flags, &arg
}
