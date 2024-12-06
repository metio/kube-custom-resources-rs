// SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
// SPDX-License-Identifier: 0BSD

use code_generator::{group_name, read_all_existing_custom_resources};
use handlebars::{to_json, Handlebars};
use itertools::Itertools;
use serde_json::Map;
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::Result;

fn main() -> Result<()> {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
    let crd_catalog = format!("{}/crd-catalog", root);
    let template_path = format!(
        "{}/code-generator/src/templates/README-project.md.hbs",
        root
    );
    let custom_resources_root =
        fs::canonicalize(format!("{}/custom-resources", root)).expect("canonicalize failed");

    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);

    handlebars
        .register_template_file("README-project", &template_path)
        .expect("register template failed");

    let entries = read_all_existing_custom_resources(&crd_catalog, &custom_resources_root);
    let mut groups_crates: HashMap<String, String> = HashMap::new();
    for (group, versions) in &entries {
        let group_snake_case = group_name(group);
        if !versions.is_empty() {
            groups_crates
                .entry(group.to_string())
                .or_insert_with(|| format!("kcr_{}", group_snake_case));
        }
    }

    let sorted: HashMap<String, String> = groups_crates
        .into_iter()
        .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        .collect();

    let mut data = Map::new();
    data.insert("crates".to_string(), to_json(&sorted));

    let readme_global_file =
        fs::canonicalize(format!("{}/README.md", root)).expect("absolute failed");
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(readme_global_file)
        .expect("unable to open file");
    handlebars
        .render_to_write("README-project", &data, &file)
        .expect("error rendering template");

    Ok(())
}
