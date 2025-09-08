// SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
// SPDX-License-Identifier: 0BSD

use glob::glob;
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub mod catalog;

const KUBERNETES_MAJOR_VERSION: usize = 34;

pub fn k8s_openapi_kubernetes_version() -> String {
    format!("v1_{}", KUBERNETES_MAJOR_VERSION)
}

pub fn kube_rs_major_version() -> String {
    // Kubernetes was at 1.33 when kube-rs 1.0.0 was released
    format!("{}", KUBERNETES_MAJOR_VERSION - 32)
}

pub fn last_path_segment<P: AsRef<Path>>(path: P) -> String {
    path.as_ref()
        .file_name()
        .expect("path must have a file name")
        .to_str()
        .expect("path must be valid utf-8")
        .to_owned()
}

pub fn path_contains<P: AsRef<Path>>(path: P, pattern: &str) -> bool {
    path.as_ref()
        .to_str()
        .expect("path contains invalid unicode")
        .contains(pattern)
}

pub fn group_name(group: &str) -> String {
    group.replace(".", "_").replace("-", "_").to_lowercase()
}

pub fn kind_name(kind: &str) -> String {
    kind.replace(".", "_").replace("-", "_")
}

pub fn read_all_existing_custom_resources(
    crd_catalog: &str,
    custom_resources_root: &Path,
) -> HashMap<String, HashMap<String, Vec<String>>> {
    let mut entries: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

    let yaml_files = format!("{}/**/*.yaml", crd_catalog);
    for entry in glob(&yaml_files).expect("should be able to read glob pattern") {
        match entry {
            Ok(path) => {
                let content = fs::read_to_string(path).expect("should be able to read file");
                let crd = serde_yaml_ng::from_str::<CustomResourceDefinition>(&content)
                    .expect("should be able to parse YAML");

                let group = &crd.spec.group;
                let group_snake_case = group_name(group);
                let version = &crd.spec.versions[0].name;
                let kind = &crd.spec.names.kind;
                let plural = &crd.spec.names.plural;

                let custom_resource_file = custom_resources_root
                    .join(&group_snake_case)
                    .join("src")
                    .join(version)
                    .join(format!("{}.rs", kind_name(plural)));

                let group_map = entries
                    .entry(group.to_string())
                    .or_insert_with(HashMap::new);

                if custom_resource_file.exists() {
                    group_map
                        .entry(version.to_string())
                        .or_insert_with(Vec::new)
                        .push(kind.clone());
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }

    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_path_segment() {
        assert_eq!(
            last_path_segment(Path::new("/some/path/somewhere")),
            "somewhere"
        );
        assert_eq!(last_path_segment(Path::new("/some/")), "some");
        assert_eq!(last_path_segment(Path::new("/some/path")), "path");
    }

    #[test]
    fn test_path_contains() {
        assert!(path_contains(Path::new("/some/path/somewhere"), "path"));
        assert!(path_contains(
            Path::new("/some/path/somewhere"),
            "path/somewhere"
        ));
        assert!(path_contains(
            Path::new("/some/path/somewhere"),
            "path/some"
        ));
    }

    #[test]
    fn test_group_name() {
        assert_eq!(group_name("some.group.name"), "some_group_name");
        assert_eq!(group_name("some.fancy-group.name"), "some_fancy_group_name");
        assert_eq!(group_name("group"), "group");
    }
}
