package terminal

import (
	"nmk/internal/nmk"
	"os"
	"sort"
)

func is256Term() bool {
	terms := []string{"cygwin", "gnome-256color", "putty", "screen-256color", "xterm-256color"}
	sort.Strings(terms)
	Term := os.Getenv("TERM")
	i := sort.SearchStrings(terms, Term)
	return i < len(terms) && terms[i] == Term
}

func is256ColorTerm() bool {
	colorTerms := []string{"gnome-terminal", "rxvt-xpm", "xfce4-terminal"}
	sort.Strings(colorTerms)
	ColorTerm := os.Getenv("COLORTERM")
	i := sort.SearchStrings(colorTerms, ColorTerm)
	return i < len(colorTerms) && colorTerms[i] == ColorTerm
}

func Support256Color(arg *nmk.Arg) bool {
	return arg.Force256color || is256Term() || is256ColorTerm() || (arg.AutoFix && nmk.IsInsideDocker())
}
