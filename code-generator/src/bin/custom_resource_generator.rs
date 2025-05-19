// SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
// SPDX-License-Identifier: 0BSD

use code_generator::{group_name, kind_name, last_path_segment, path_contains};
use glob::glob;
use std::process::{Command, Stdio};
use std::{env, fs};
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
    let root_folder = fs::canonicalize(root).expect("canonicalize failed");
    let crd_catalog = root_folder.join("crd-catalog");
    let custom_resources_root = root_folder.join("custom-resources");

    let mut number_of_matched_crds: i64 = 0;

    let yaml_files = format!("{}/**/*.yaml", crd_catalog.display());
    for entry in glob(&yaml_files).expect("should be able to read glob pattern") {
        match entry {
            Ok(path) => {
                if (args.len() == 2 && path_contains(&path, &args[1])) || args.len() < 2 {
                    number_of_matched_crds += 1;
                    let plural = path.file_stem().unwrap().to_str().unwrap();
                    let version = last_path_segment(path.parent().expect("parent should exist"));
                    let group = last_path_segment(
                        path.parent()
                            .expect("parent should exist")
                            .parent()
                            .expect("parent should exist"),
                    );
                    let group_snake_case = group_name(&group);
                    let kind = kind_name(plural);

                    let group_version_directory = custom_resources_root
                        .join(group_snake_case)
                        .join("src")
                        .join(version);
                    let resource_target = group_version_directory.join(format!("{kind}.rs"));

                    let ignore_file = path.with_extension("ignore");
                    if ignore_file.exists() {
                        if resource_target.exists() {
                            fs::remove_file(resource_target).expect("remove failed");
                        }
                    } else {
                        fs::create_dir_all(&group_version_directory).unwrap();
                        let absolute_yaml_path = path.to_string_lossy();
                        let relative_yaml_path = absolute_yaml_path
                            .chars()
                            .skip(&root_folder.to_string_lossy().len() + 1)
                            .collect::<String>();
                        let child = Command::new("kopium")
                            .current_dir(&root_folder)
                            .args(&[
                                "--docs",
                                "--derive=Default",
                                "--derive=PartialEq",
                                "--smart-derive-elision",
                                "--filename",
                                &relative_yaml_path,
                            ])
                            .stdout(Stdio::piped())
                            .spawn()
                            .expect("should be able to run kopium");
                        let output = child
                            .wait_with_output()
                            .expect("should be able to read from stdout");

                        fs::write(&resource_target, output.stdout)
                            .expect("should be able to write custom resource");

                        let code_file = path.with_extension("rs");
                        if code_file.exists() {
                            let additional_code = fs::read(code_file).expect("reading file with additional code failed");
                            let mut generated_file = OpenOptions::new()
                                .append(true)
                                .open(resource_target)
                                .expect("cannot target resource file");
                            generated_file
                                .write(&additional_code)
                                .expect("writing to target resource file failed");
                        }
                    }
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }

    if args.len() == 2 && number_of_matched_crds == 0 {
        std::process::exit(2);
    }
}
