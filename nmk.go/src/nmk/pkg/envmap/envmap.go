package envmap

import (
	"strings"
)

type EnvMap struct {
	m map[string]string
}

func New(envv []string) *EnvMap {
	return new(EnvMap).Init(envv)
}

func SplitKeyValue(s string) (string, string) {
	slice := strings.SplitN(s, "=", 2)
	k := slice[0]
	v := slice[1]
	return k, v
}

func MergeKeyValue(k string, v string) string {
	return strings.Join([]string{k, v}, "=")
}

func (em *EnvMap) Init(envv []string) *EnvMap {
	em.m = make(map[string]string)
	for _, kv := range envv {
		k, v := SplitKeyValue(kv)
		em.m[k] = v
	}
	return em
}

func (em *EnvMap) Set(name string, value string) {
	em.m[name] = value
}

func (em *EnvMap) Get(name string) string {
	return em.m[name]
}

func (em *EnvMap) Unset(name string) {
	delete(em.m, name)
}

func (em *EnvMap) Contains(name string) bool {
	_, ok := em.m[name]
	return ok
}

func (em *EnvMap) Environ() []string {
	result := make([]string, 0, len(em.m))
	for k, v := range em.m {
		kv := MergeKeyValue(k, v)
		result = append(result, kv)
	}
	return result
}
