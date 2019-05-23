package nmk

import (
	"testing"

	. "github.com/smartystreets/goconvey/convey"
)

func TestParseCgroupLine(t *testing.T) {
	Convey("Test ParseCgroupLine", t, func() {
		expect := []string{"12", "cpu,cpuacct", "/"}
		So(ParseCgroupLine("12:cpu,cpuacct:/"), ShouldResemble, expect)
	})
}

var dockerPid1cgroup = `
12:cpu,cpuacct:/docker/c6fa62a9938149f6098fd0cdaffc9cdf0f526f25d97b5f6e2a4cc1fccc7f7ce1
11:perf_event:/docker/c6fa62a9938149f6098fd0cdaffc9cdf0f526f25d97b5f6e2a4cc1fccc7f7ce1
10:rdma:/
9:freezer:/docker/c6fa62a9938149f6098fd0cdaffc9cdf0f526f25d97b5f6e2a4cc1fccc7f7ce1
8:blkio:/docker/c6fa62a9938149f6098fd0cdaffc9cdf0f526f25d97b5f6e2a4cc1fccc7f7ce1
7:cpuset:/docker/c6fa62a9938149f6098fd0cdaffc9cdf0f526f25d97b5f6e2a4cc1fccc7f7ce1
6:devices:/docker/c6fa62a9938149f6098fd0cdaffc9cdf0f526f25d97b5f6e2a4cc1fccc7f7ce1
5:memory:/docker/c6fa62a9938149f6098fd0cdaffc9cdf0f526f25d97b5f6e2a4cc1fccc7f7ce1
4:net_cls,net_prio:/docker/c6fa62a9938149f6098fd0cdaffc9cdf0f526f25d97b5f6e2a4cc1fccc7f7ce1
3:hugetlb:/docker/c6fa62a9938149f6098fd0cdaffc9cdf0f526f25d97b5f6e2a4cc1fccc7f7ce1
2:pids:/docker/c6fa62a9938149f6098fd0cdaffc9cdf0f526f25d97b5f6e2a4cc1fccc7f7ce1
1:name=systemd:/docker/c6fa62a9938149f6098fd0cdaffc9cdf0f526f25d97b5f6e2a4cc1fccc7f7ce1
0::/system.slice/containerd.service
`

var initCgroup = `
12:cpu,cpuacct:/
11:perf_event:/
10:rdma:/
9:freezer:/
8:blkio:/
7:cpuset:/
6:devices:/
5:memory:/
4:net_cls,net_prio:/
3:hugetlb:/
2:pids:/
1:name=systemd:/init.scope
0::/init.scope
`

func TestIsDockerCgroup(t *testing.T) {
	Convey("Test IsDockerCgroup", t, func() {
		So(IsDockerCgroup(dockerPid1cgroup), ShouldBeTrue)
		So(IsDockerCgroup(initCgroup), ShouldBeFalse)
	})
}
