// SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
// SPDX-License-Identifier: 0BSD

use code_generator::{k8s_openapi_kubernetes_version, kube_rs_major_version, last_path_segment};
use handlebars::{to_json, Handlebars};
use serde_json::Map;
use std::fs;
use std::fs::OpenOptions;
use std::io::Result;

fn main() -> Result<()> {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
    let template_path = format!("{}/code-generator/src/templates/Cargo.toml.hbs", root);
    let custom_resources_root =
        fs::canonicalize(format!("{}/custom-resources", root)).expect("canonicalize failed");

    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);

    handlebars
        .register_template_file("Cargo.toml", &template_path)
        .expect("register template failed");

    for group in fs::read_dir(custom_resources_root)? {
        let group_path = group?.path();
        let group_name = last_path_segment(&group_path);

        let cargo_toml_file = group_path.join("Cargo.toml");
        let lib_rs_file = group_path.join("src/lib.rs");

        if lib_rs_file.exists() {
            let mut versions = vec![];
            for version in fs::read_dir(group_path.join("src"))? {
                let entry = version?;
                let version_path = entry.path();
                if entry.metadata()?.is_dir() && version_path.join("mod.rs").exists() {
                    versions.push(last_path_segment(&version_path))
                }
            }

            versions.sort();

            let mut data = Map::new();
            data.insert("group_name_snake_case".to_string(), to_json(group_name));
            data.insert(
                "k8s_openapi_kubernetes_version".to_string(),
                to_json(k8s_openapi_kubernetes_version()),
            );
            data.insert(
                "kube_rs_major_version".to_string(),
                to_json(kube_rs_major_version()),
            );
            data.insert("versions".to_string(), to_json(&versions));

            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(cargo_toml_file)
                .expect("unable to open file");
            handlebars
                .render_to_write("Cargo.toml", &data, &file)
                .expect("error rendering template");
        } else if cargo_toml_file.exists() {
            fs::remove_file(cargo_toml_file)?
        }
    }

    Ok(())
}
