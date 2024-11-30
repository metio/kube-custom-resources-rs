// SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
// SPDX-License-Identifier: 0BSD

use handlebars::{to_json, Handlebars};
use serde_json::Map;
use std::fs;
use std::fs::OpenOptions;
use std::io::Result;

fn main() -> Result<()> {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
    let template_path = format!("{}/code-generator/src/templates/Workspace.toml.hbs", root);
    let custom_resources_root =
        fs::canonicalize(format!("{}/custom-resources", root)).expect("canonicalize failed");

    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);

    handlebars
        .register_template_file("Workspace.toml", &template_path)
        .expect("register template failed");

    let mut groups = vec![];
    for group in fs::read_dir(custom_resources_root)? {
        let group_path = group?.path();
        if group_path.join("Cargo.toml").exists() {
            let group_name = group_path
                .file_stem()
                .expect("extract file stem")
                .to_str()
                .expect("convert file stem");
            groups.push(group_name.to_string());
        }
    }

    let mut data = Map::new();
    data.insert("groups".to_string(), to_json(&groups));

    let cargo_toml_file =
        fs::canonicalize(format!("{}/Cargo.toml", root)).expect("absolute failed");
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(cargo_toml_file)
        .expect("unable to open file");
    handlebars
        .render_to_write("Workspace.toml", &data, &file)
        .expect("error rendering template");

    Ok(())
}
