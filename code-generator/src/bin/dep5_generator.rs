// SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
// SPDX-License-Identifier: 0BSD

use code_generator::catalog;
use handlebars::{to_json, Handlebars};
use serde_json::Map;
use std::fs::OpenOptions;
use std::io::Result;

fn main() -> Result<()> {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
    let template_path = format!("{}/code-generator/src/templates/dep5.hbs", root);
    let dep5_file = format!("{}/.reuse/dep5", root);
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(dep5_file)
        .expect("unable to open file");

    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);

    handlebars
        .register_template_file("dep5", &template_path)
        .expect("register template failed");

    let mut data = Map::new();
    data.insert("sources".to_string(), to_json(catalog::CRD_V1_SOURCES));

    handlebars
        .render_to_write("dep5", &data, &file)
        .expect("error rendering template");

    Ok(())
}
