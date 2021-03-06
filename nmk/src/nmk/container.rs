use std::fs;

use crate::platform;

struct CGroup<'a> {
    control_group: &'a str,
    #[allow(dead_code)]
    hierarchy_id: &'a str,
    #[allow(dead_code)]
    subsystems: &'a str,
}

impl<'a> CGroup<'a> {
    pub const SEPARATOR: char = ':';

    pub fn parse(line: &'a str) -> Option<Self> {
        let mut iter = line.split(Self::SEPARATOR);
        let (hierarchy_id, subsystems, control_group) = (iter.next()?, iter.next()?, iter.next()?);
        Some(Self {
            control_group,
            hierarchy_id,
            subsystems,
        })
    }

    pub fn is_container(&self) -> bool {
        let cgroup = self.control_group;
        cgroup.starts_with("/docker") || cgroup.starts_with("/kube")
    }
}

fn is_container(s: &str) -> bool {
    s.lines()
        .flat_map(CGroup::parse)
        .any(|cg| cg.is_container())
}

pub fn is_containerized() -> bool {
    if platform::is_mac() {
        return false;
    }
    let self_cgroup = format!("/proc/{}/cgroup", std::process::id());
    let contents = fs::read_to_string(self_cgroup).expect("cannot read self cgroup");
    is_container(&contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cgroup_parse() {
        let actual = CGroup::parse("12:cpu,cpuacct:/").unwrap();
        let expect = CGroup {
            control_group: "/",
            hierarchy_id: "12",
            subsystems: "cpu,cpuacct",
        };
        assert_eq!(actual.control_group, expect.control_group);
        assert_eq!(actual.hierarchy_id, expect.hierarchy_id);
        assert_eq!(actual.subsystems, expect.subsystems);
    }

    #[test]
    fn test_is_container() {
        let docker_cgroup = r#"
12:cpu,cpuacct:/docker/c6fa62a9938149f6098fd0cdaffc9cdf0f526f25d97b5f6e2a4cc1fccc7f7ce1
11:perf_event:/docker/c6fa62a9938149f6098fd0cdaffc9cdf0f526f25d97b5f6e2a4cc1fccc7f7ce1
10:rdma:/"#;
        assert!(is_container(docker_cgroup));
        let init_cgroup = r#"
12:cpu,cpuacct:/
11:perf_event:/
0::/init.scope"#;
        assert!(!is_container(init_cgroup));
        let k8s_cgroup = r#"
12:hugetlb:/kubepods/besteffort/poda00e29fd-7bbd-11e9-8679-fa163ea7e3b8/c4b1403f3d9c7ce261be851df71d9a9773c53419075ccda39ae8fe6a39fd2eb1
11:cpuset:/kubepods/besteffort/poda00e29fd-7bbd-11e9-8679-fa163ea7e3b8/c4b1403f3d9c7ce261be851df71d9a9773c53419075ccda39ae8fe6a39fd2eb1"#;
        assert!(is_container(k8s_cgroup));
    }
}
