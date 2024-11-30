// SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
// SPDX-License-Identifier: 0BSD

use handlebars::{to_json, Handlebars};
use serde_json::Map;
use std::fs;
use std::fs::OpenOptions;
use std::io::Result;
use code_generator::last_path_segment;

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
            let mut data = Map::new();
            data.insert("group_name_snake_case".to_string(), to_json(group_name));
            data.insert(
                "k8s_openapi_kubernetes_version".to_string(),
                to_json("v1_31"),
            );

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
