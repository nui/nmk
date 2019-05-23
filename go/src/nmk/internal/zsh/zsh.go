package zsh

import (
	"nmk/internal/nmk"
	"os"
	"path"
	"runtime"
)

func isAlpine() bool {
	_, err := os.Stat("/etc/alpine-release")
	return err == nil
}

func isArch() bool {
	_, err := os.Stat("/etc/arch-release")
	return err == nil
}

func isMac() bool {
	return runtime.GOOS == "darwin"
}

func hasLocalZsh(nmkDir string) bool {
	localZshPath := path.Join(nmkDir, "local", "bin", "zsh")
	_, err := os.Stat(localZshPath)
	return err == nil
}

func UseGlobalRcs(arg *nmk.Arg, nmkDir string) bool {
	// Some linux distribution global zprofile contains a line that will source everything
	// from /etc/profile. And they do reset $PATH completely.
	// It makes PATH set by nmk unusable
	badGlobalRcs := isAlpine() || isArch() || isMac()

	noGlobalRcs := arg.AutoFix && badGlobalRcs && !hasLocalZsh(nmkDir)
	return !noGlobalRcs
}
