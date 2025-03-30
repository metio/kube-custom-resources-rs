// SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
// SPDX-License-Identifier: 0BSD

use handlebars::{to_json, Handlebars};
use serde_json::Map;
use std::fs;
use std::fs::OpenOptions;
use std::io::Result;

fn main() -> Result<()> {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
    let template_path = format!("{}/code-generator/src/templates/mod.rs.hbs", root);
    let custom_resources_root =
        fs::canonicalize(format!("{}/custom-resources", root)).expect("canonicalize failed");

    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);

    handlebars
        .register_template_file("mod.rs", &template_path)
        .expect("register template failed");

    for group in fs::read_dir(custom_resources_root)? {
        let group_path = group?.path();

        for version in fs::read_dir(&format!("{}/src", group_path.display()))? {
            let version_path = version?.path();
            if version_path.ends_with("lib.rs") {
                continue;
            }

            let mut versioned_resources = vec![];
            for resources in fs::read_dir(&version_path)? {
                let resources_path = resources?.path();
                if resources_path.ends_with("mod.rs") {
                    continue;
                }
                let basename = resources_path
                    .file_stem()
                    .expect("extract file stem")
                    .to_str()
                    .expect("convert file stem");
                versioned_resources.push(basename.to_string());
            }

            let mod_rs_file = version_path.join("mod.rs");

            if versioned_resources.is_empty() {
                if mod_rs_file.exists() {
                    fs::remove_file(&mod_rs_file).expect("failed to remove mod.rs");
                }
            } else {
                let mut data = Map::new();
                versioned_resources.sort();
                data.insert("resources".to_string(), to_json(versioned_resources));

                let file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(mod_rs_file)
                    .expect("unable to open file");
                handlebars
                    .render_to_write("mod.rs", &data, &file)
                    .expect("error rendering template");
            }
        }
    }

    Ok(())
}
