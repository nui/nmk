package nmk

import (
	"github.com/sirupsen/logrus"
	"os"
	"os/user"
	"path"
)

func FindNmkDir() string {
	const nmkDir = "NMK_DIR"
	if nmkDir, found := os.LookupEnv(nmkDir); found {
		return nmkDir
	} else {
		usr, err := user.Current()
		if err != nil {
			logrus.Fatal(err)
		}
		d := path.Join(usr.HomeDir, ".nmk")
		if _, err := os.Stat(d); err != nil {
			logrus.Fatalf("%s is not a directory", nmkDir)
		}
		return d
	}
}
