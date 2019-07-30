use std::process::exit;

use crate::platform;

#[allow(dead_code)]
struct CGroup<'a> {
    hierarchy_id: &'a str,
    subsystems: &'a str,
    control_group: &'a str,
}

impl<'a> CGroup<'a> {
    pub fn parse(line: &'a str) -> Option<Self> {
        let mut iter = line.split(":");
        match (iter.next(), iter.next(), iter.next()) {
            (Some(hierarchy_id), Some(subsystems), Some(control_group)) => Some(Self {
                hierarchy_id,
                subsystems,
                control_group,
            }),
            _ => None
        }
    }

    pub fn is_container(&self) -> bool {
        self.control_group.starts_with("/docker") || self.control_group.starts_with("/kube")
    }
}

fn is_container(s: &str) -> bool {
    s.split("\n").any(|line| {
        CGroup::parse(line)
            .map(|cg| cg.is_container())
            .unwrap_or_default()
    })
}

pub fn detect_container() -> bool {
    if platform::is_mac() {
        return false;
    }
    let contents = std::fs::read_to_string("/proc/1/cgroup").unwrap_or_else(|_| {
        error!("Cannot open cgroup file");
        exit(1);
    });
    return is_container(&contents);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cgroup_parse() {
        let actual = CGroup::parse("12:cpu,cpuacct:/").unwrap();
        let expect = CGroup {
            hierarchy_id: "12",
            subsystems: "cpu,cpuacct",
            control_group: "/",
        };
        assert_eq!(actual.hierarchy_id, expect.hierarchy_id);
        assert_eq!(actual.subsystems, expect.subsystems);
        assert_eq!(actual.control_group, expect.control_group);
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
