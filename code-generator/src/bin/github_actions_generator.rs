// SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
// SPDX-License-Identifier: 0BSD

use handlebars::{to_json, Handlebars};
use serde_json::Map;
use std::fs;
use std::fs::OpenOptions;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::Result;

fn main() -> Result<()> {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
    let verify_template = format!("{}/code-generator/src/templates/verify-crd.yml.hbs", root);
    let release_template = format!("{}/code-generator/src/templates/release-crd.yml.hbs", root);
    let custom_resources_root =
        fs::canonicalize(format!("{}/custom-resources", root)).expect("canonicalize failed");
    let github_actions_root =
        fs::canonicalize(format!("{}/.github/workflows", root)).expect("canonicalize failed");

    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);

    handlebars
        .register_template_file("verify-crd", &verify_template)
        .expect("register template failed");
    handlebars
        .register_template_file("release-crd", &release_template)
        .expect("register template failed");

    for group in fs::read_dir(custom_resources_root)? {
        let group_path = group?.path();
        let group_name = group_path.file_name().unwrap().to_str().unwrap();

        let cargo_toml_file = group_path.join("Cargo.toml");
        let verify_action = format!(
            "{}/verify-{}.yml",
            github_actions_root.display(),
            group_name
        );
        let release_action = format!(
            "{}/release-{}.yml",
            github_actions_root.display(),
            group_name
        );

        if cargo_toml_file.exists() {
            let mut hasher = DefaultHasher::new();
            group_name.hash(&mut hasher);
            let hash = hasher.finish();

            let mut data = Map::new();
            data.insert("group_name_snake_case".to_string(), to_json(group_name));
            data.insert(
                "package_name".to_string(),
                to_json(format!("kcr_{group_name}")),
            );
            data.insert(
                "k8s_openapi_kubernetes_version".to_string(),
                to_json("1.31"),
            );
            data.insert("cron_minute".to_string(), to_json(hash.rem_euclid(60)));
            data.insert("cron_hour".to_string(), to_json(hash.rem_euclid(24)));

            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(verify_action)
                .expect("unable to open file");
            handlebars
                .render_to_write("verify-crd", &data, &file)
                .expect("error rendering template");

            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(release_action)
                .expect("unable to open file");
            handlebars
                .render_to_write("release-crd", &data, &file)
                .expect("error rendering template");
        } else {
            if fs::exists(&verify_action).expect("unable to open file") {
                fs::remove_file(&verify_action)?;
            }
            if fs::exists(&release_action).expect("unable to open file") {
                fs::remove_file(&release_action)?;
            }
        }
    }

    Ok(())
}
