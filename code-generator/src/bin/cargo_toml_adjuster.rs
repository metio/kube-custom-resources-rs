// SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
// SPDX-License-Identifier: 0BSD

use cargo_toml::Manifest;
use std::{fs, path};
use std::io::{Result, Write};

fn main() -> Result<()> {
    let root = path::absolute(concat!(env!("CARGO_MANIFEST_DIR")))?;
    let workspace = root.parent().expect("to have parent");
    let sources = workspace.join("kube-custom-resources-rs/src");
    let cargo_toml = workspace.join("kube-custom-resources-rs/Cargo.toml");

    let mut features: Vec<String> = vec![];

    for entry in fs::read_dir(sources)? {
        let directory = entry?;
        let path = directory.path();
        if path.is_dir() {
            if directory.path().join("mod.rs").try_exists()? {
                if let Some(file_name) = directory.path().file_name() {
                    features.push(String::from(file_name.to_string_lossy()));
                }
            }
        }
    }

    features.sort();

    match Manifest::from_path(&cargo_toml) {
        Ok(mut manifest) => {
            manifest.features.clear();
            for feature in features {
                manifest.features.insert(feature.clone(), vec![]);
            }
            let manifest_content = toml::to_string(&manifest).unwrap();
            let mut cargo_toml_file = fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&cargo_toml)?;
            cargo_toml_file.write_all(manifest_content.as_bytes())?;
        },
        Err(e) => println!("{:?}", e),
    }

    Ok(())
}
