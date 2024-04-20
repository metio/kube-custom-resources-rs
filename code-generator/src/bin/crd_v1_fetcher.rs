// SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
// SPDX-License-Identifier: 0BSD

use std::{env, fs};

use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use k8s_openapi::serde::Deserialize;
use reqwest::blocking::Client;
use serde_yaml::Value;

use code_generator::catalog;

fn main() {
    let args: Vec<String> = env::args().collect();
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
    let client = Client::new();

    for source in catalog::CRD_V1_SOURCES {
        for url in source.urls {
            if (args.len() == 2 && url.contains(&args[1])) || args.len() < 2 {
                let raw_url = gitlab_url(github_url(url));
                println!("Downloading {}", raw_url);
                if let Ok(response) = client.get(raw_url).send() {
                    if response.status().is_success() {
                        if let Ok(content) = response.text() {
                            for crd in parse_crds(content) {
                                let directory = format!(
                                    "{}/crd-catalog/{}/{}/{}",
                                    root,
                                    source.project_name,
                                    crd.spec.group,
                                    crd.spec.versions[0].name
                                );
                                let file = format!("{}/{}.yaml", directory, crd.spec.names.plural);

                                fs::create_dir_all(directory).unwrap_or_else(|why| {
                                    println!("! {:?}", why);
                                });

                                if let Ok(data) = serde_yaml::to_string(&crd) {
                                    fs::write(file, data).unwrap_or_else(|why| {
                                        println!("! {:?}", why);
                                    });
                                }
                            }
                        }
                    } else {
                        println!("  Failed with status code {}", response.status().as_str());
                    }
                }
            }
        }
    }
}

fn parse_crds(content: String) -> Vec<CustomResourceDefinition> {
    let mut crds: Vec<CustomResourceDefinition> = vec![];

    for document in serde_yaml::Deserializer::from_str(&content) {
        if let Ok(yaml) = Value::deserialize(document) {
            if let Ok(crd) = serde_yaml::from_value::<CustomResourceDefinition>(yaml) {
                for version in &crd.spec.versions {
                    let mut cloned = crd.clone();
                    cloned.spec.versions = vec![version.to_owned()];
                    crds.push(cloned);
                }
            }
        }
    }

    crds
}

fn github_url(url: &str) -> String {
    if !url.starts_with("https://github.com")
        || url.starts_with("https://raw.githubusercontent.com")
        || url.starts_with("https://github.com") && url.contains("releases/latest/download")
    {
        url.to_owned()
    } else {
        let mut raw: String = String::from(url);
        if url.starts_with("https://github.com") {
            raw = url.replacen("github.com", "raw.githubusercontent.com", 1);
        } else if url.starts_with("https://www.github.com") {
            raw = url.replacen("www.github.com", "raw.githubusercontent.com", 1);
        }

        raw.replacen("/blob", "", 1)
    }
}

fn gitlab_url(url: String) -> String {
    if !url.starts_with("https://gitlab.com") {
        url
    } else {
        url.replacen("/blob/", "/raw/", 1)
    }
}
