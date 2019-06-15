use std::process::exit;

use crate::platform;

#[allow(dead_code)]
struct CGroup {
    hierarchy_id: String,
    subsystems: String,
    control_group: String,
}

const CGROUP_DELIMITER: &str = ":";

impl CGroup {
    fn parse(s: &str) -> Self {
        const INVALID: &str = "invalid cgroup line";
        let mut sp = s.split(CGROUP_DELIMITER);
        Self {
            hierarchy_id: sp.next().expect(INVALID).to_string(),
            subsystems: sp.next().expect(INVALID).to_string(),
            control_group: sp.next().expect(INVALID).to_string(),
        }
    }

    pub fn is_container(&self) -> bool {
        self.control_group.starts_with("/docker") || self.control_group.starts_with("/kube")
    }
}

fn is_container(s: &str) -> bool {
    let mut container = false;
    for line in s.split("\n") {
        if !line.contains(CGROUP_DELIMITER) {
            continue;
        }
        if CGroup::parse(line).is_container() {
            container = true;
            break;
        };
    }
    return container;
}

pub fn check_container() -> bool {
    if platform::is_mac() {
        return false;
    }
    let contents = std::fs::read_to_string("/proc/1/cgroup").unwrap_or_else(|_| {
        error!("Cannot open cgroup file");
        exit(1);
    });
    return is_container(contents.as_str());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cgroup_parse() {
        let actual = CGroup::parse(&"12:cpu,cpuacct:/".to_string());
        let expect = CGroup {
            hierarchy_id: "12".to_string(),
            subsystems: "cpu,cpuacct".to_string(),
            control_group: "/".to_string(),
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

        let init_cgroup = r#"
12:cpu,cpuacct:/
11:perf_event:/
0::/init.scope"#;

        let k8s_cgroup = r#"
        12:hugetlb:/kubepods/besteffort/poda00e29fd-7bbd-11e9-8679-fa163ea7e3b8/c4b1403f3d9c7ce261be851df71d9a9773c53419075ccda39ae8fe6a39fd2eb1
11:cpuset:/kubepods/besteffort/poda00e29fd-7bbd-11e9-8679-fa163ea7e3b8/c4b1403f3d9c7ce261be851df71d9a9773c53419075ccda39ae8fe6a39fd2eb1"#;

        assert_eq!(is_container(&docker_cgroup), true);
        assert_eq!(is_container(&init_cgroup), false);
        assert_eq!(is_container(&k8s_cgroup), true);
    }
}
