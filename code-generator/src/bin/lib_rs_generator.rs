// SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
// SPDX-License-Identifier: 0BSD

use std::fs;
use std::fs::OpenOptions;
use std::io::Result;

use code_generator::{group_name, read_all_existing_custom_resources};
use handlebars::{to_json, Handlebars};
use serde_json::Map;

fn main() -> Result<()> {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
    let crd_catalog = format!("{}/crd-catalog", root);
    let custom_resources_root =
        fs::canonicalize(format!("{}/custom-resources", root)).expect("canonicalize failed");
    let template_path = format!("{}/code-generator/src/templates/lib.rs.hbs", root);

    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);

    handlebars
        .register_template_file("lib.rs", &template_path)
        .expect("register template failed");

    let entries = read_all_existing_custom_resources(&crd_catalog, &custom_resources_root);

    for (group, versions) in &entries {
        let group_snake_case = group_name(group);

        let lib_rs_file = custom_resources_root
            .join(&group_snake_case)
            .join("src/lib.rs");

        if versions.is_empty() {
            if lib_rs_file.exists() {
                fs::remove_file(lib_rs_file)?;
            }
        } else {
            let mut data = Map::new();
            data.insert("group_name".to_string(), to_json(group));
            data.insert(
                "group_name_snake_case".to_string(),
                to_json(&group_snake_case),
            );
            data.insert("versions".to_string(), to_json(versions));

            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(lib_rs_file)
                .expect("unable to open file");

            handlebars
                .render_to_write("lib.rs", &data, &file)
                .expect("error rendering template");
        }
    }

    Ok(())
}
