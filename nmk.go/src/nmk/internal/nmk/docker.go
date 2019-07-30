package nmk

import (
	"fmt"
	log "github.com/sirupsen/logrus"
	"io/ioutil"
	"strings"
)

func IsDockerCgroup(s string) bool {
	docker := false
	for _, line := range strings.Split(s, "\n") {
		if strings.TrimSpace(line) == "" {
			continue
		}
		parts := ParseCgroupLine(line)
		controlGroup := parts[2]
		if strings.HasPrefix(controlGroup, "/docker") ||
			strings.HasPrefix(controlGroup, "/kube") {
			docker = true
			break
		}
	}
	return docker
}

func ParseCgroupLine(s string) []string {
	return strings.SplitN(s, ":", 3)
}

func IsInsideDocker() bool {
	const cgroupFile = "/proc/1/cgroup"
	bytes, err := ioutil.ReadFile(cgroupFile)
	if err != nil {
		log.Fatal(fmt.Sprintf("Can't read %s", cgroupFile))
	}
	isDocker := IsDockerCgroup(string(bytes))
	if isDocker {
		log.Debug("Detect running inside docker container")
	}
	return isDocker
}