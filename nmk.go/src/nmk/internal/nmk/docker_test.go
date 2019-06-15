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

var dockerCgroup = `
12:cpu,cpuacct:/docker/c6fa62a9938149f6098fd0cdaffc9cdf0f526f25d97b5f6e2a4cc1fccc7f7ce1
11:perf_event:/docker/c6fa62a9938149f6098fd0cdaffc9cdf0f526f25d97b5f6e2a4cc1fccc7f7ce1
10:rdma:/
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

var k8sCgroup = `
12:hugetlb:/kubepods/besteffort/poda00e29fd-7bbd-11e9-8679-fa163ea7e3b8/c4b1403f3d9c7ce261be851df71d9a9773c53419075ccda39ae8fe6a39fd2eb1
11:cpuset:/kubepods/besteffort/poda00e29fd-7bbd-11e9-8679-fa163ea7e3b8/c4b1403f3d9c7ce261be851df71d9a9773c53419075ccda39ae8fe6a39fd2eb1
`

func TestIsDockerCgroup(t *testing.T) {
	Convey("Test IsDockerCgroup", t, func() {
		So(IsDockerCgroup(dockerCgroup), ShouldBeTrue)
		So(IsDockerCgroup(k8sCgroup), ShouldBeTrue)
		So(IsDockerCgroup(initCgroup), ShouldBeFalse)
	})
}
