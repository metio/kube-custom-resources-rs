#!/usr/bin/env sh

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

FILTER="${1:-}"

cd code-generator || exit 1

# fetch CRDs
if ! cargo run --bin crd_v1_fetcher "${FILTER}"; then
  echo 'could not fetch CRDs - make sure that the given filter matches entries in the catalog and that you have a working internet connection'
  exit 1
fi

# generate dep5 file
if ! cargo run --bin dep5_generator; then
  echo 'could not generate dep5 file'
  exit 1
fi

# generate Rust code
if ! cargo run --bin custom_resource_generator "${FILTER}"; then
  echo 'could not generate resources'
  exit 1
fi

# generate mod.rs files
if ! cargo run --bin mod_rs_generator; then
  echo 'could not generate mod.rs files'
  exit 1
fi

# generate lib.rs files
if ! cargo run --bin lib_rs_generator; then
  echo 'could not generate lib.rs files'
  exit 1
fi

# generate Cargo.toml files
if ! cargo run --bin cargo_toml_generator; then
  echo 'could not generate Cargo.toml files'
  exit 1
fi

# generate workspace Cargo.toml file
if ! cargo run --bin workspace_toml_generator; then
  echo 'could not generate workspace Cargo.toml file'
  exit 1
fi

# generate project README.md
if ! cargo run --bin project_readme_generator; then
  echo 'could not generate project README.md file'
  exit 1
fi

# generate crate README.md files
if ! cargo run --bin crate_readme_generator; then
  echo 'could not generate crate README.md files'
  exit 1
fi

# generate GitHub actions
if ! cargo run --bin github_actions_generator; then
  echo 'could not generate workspace Cargo.toml'
  exit 1
fi
