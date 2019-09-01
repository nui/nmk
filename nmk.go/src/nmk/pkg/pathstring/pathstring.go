package pathstring

import (
	"container/list"
	"os"
	"path/filepath"
	"strings"
)

type PathString struct {
	l *list.List
}

func Parse(input string) *PathString {
	return new(PathString).Init(input)
}

func (sp *PathString) Init(input string) *PathString {
	input = strings.TrimSpace(input)
	sp.l = list.New()
	for _, v := range filepath.SplitList(input) {
		sp.l.PushBack(v)
	}
	return sp
}

func (sp *PathString) Prepend(s string) *PathString {
	sp.l.PushFront(s)
	return sp
}

func (sp *PathString) Append(s string) *PathString {
	sp.l.PushBack(s)
	return sp
}

func (sp *PathString) Make() string {
	set := make(map[string]bool)
	unique := make([]string, 0, sp.l.Len())
	for e := sp.l.Front(); e != nil; e = e.Next() {
		if v, ok := e.Value.(string); ok {
			if _, exists := set[v]; !exists {
				set[v] = true
				unique = append(unique, v)
			}
		}
	}
	return strings.Join(unique, string(os.PathListSeparator))
}
