// SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
// SPDX-License-Identifier: 0BSD

use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::{Result, Write};
use std::path::Path;

use glob::glob;
use itertools::Itertools;
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;

fn main() -> Result<()> {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
    let crd_catalog = format!("{}/crd-catalog", root);
    let sources = format!("{}/kube-custom-resources-rs/src", root);
    let lib_rs_file = format!("{}/lib.rs", sources);
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(lib_rs_file)
        .expect("unable to open file");
    let mut buffer = BufWriter::new(file);

    writeln!(buffer, "/*!")?;
    writeln!(buffer, "This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium) and updated weekly.")?;
    writeln!(buffer, "")?;
    writeln!(buffer, "# Available Features")?;
    writeln!(buffer, "")?;
    writeln!(buffer, "Every group has its own feature in this crate. The available features are as follows:")?;

    let mut entries: HashMap<String, HashMap<String, Vec<CustomResourceDefinition>>> = HashMap::new();

    let yaml_files = format!("{}/**/*.yaml", crd_catalog);
    for entry in glob(&yaml_files).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let content = fs::read_to_string(path).expect("should be able to read file");
                let crd = serde_yaml::from_str::<CustomResourceDefinition>(&content)
                    .expect("should be able to parse YAML");

                let group = &crd.spec.group;
                let version = &crd.spec.versions[0].name;
                let feature = group.replace(".", "_").replace("-", "_");

                let resource_target = format!(
                    "{}/{}/{}/{}.rs",
                    sources,
                    feature,
                    version,
                    crd.spec.names.plural.replace(".", "_").replace("-", "_")
                );
                if Path::new(&resource_target).exists() {
                    entries.entry(group.to_string())
                        .or_insert_with(HashMap::new)
                        .entry(version.to_string())
                        .or_insert_with(Vec::new)
                        .push(crd);
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }

    for (group, versions) in entries.iter().sorted_by_key(|x| x.0) {
        let feature = group
            .replace(".", "_")
            .replace("-", "_");
        writeln!(buffer, "")?;
        writeln!(buffer, "## {}", feature)?;

        for (version, kinds) in versions.iter().sorted_by_key(|x| x.0) {
            writeln!(buffer, "")?;
            writeln!(buffer, "- apiVersion: `{}/{}`", group, version)?;
            writeln!(buffer, "- kinds:")?;

            for crd in kinds {
                writeln!(buffer, "  - `{}`", crd.spec.names.kind)?;
            }
        }
    }

    writeln!(buffer, " */")?;
    writeln!(buffer, "")?;

    for (group, _) in entries.iter().sorted_by_key(|x| x.0) {
        let feature = group
            .replace(".", "_")
            .replace("-", "_");
        writeln!(buffer, "#[cfg(feature = \"{}\")]", feature)?;
        writeln!(buffer, "pub mod {};", feature)?;
    }

    Ok(())
}
