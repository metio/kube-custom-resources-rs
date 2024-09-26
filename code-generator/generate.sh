#!/usr/bin/env sh

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

FILTER="${1:-}"

# fetch CRDs
if ! cargo run --package code-generator --bin crd_v1_fetcher "${FILTER}"; then
  echo 'could not fetch CRDs - make sure that the given filter matches entries in the catalog and that you have a working internet connection'
  exit 1
fi

# generate dep5 file
if ! cargo run --package code-generator --bin dep5_generator; then
  echo 'could not generate dep5 file'
  exit 1
fi

# fix YAML quoting
if ! ./code-generator/quote-yaml-strings.sh "${FILTER}"; then
  echo 'could not quote strings'
  exit 1
fi

# generate Rust code
if ! ./code-generator/create-custom-resources.sh "${FILTER}"; then
  echo 'could not generate resources'
  exit 1
fi

# generate mod.rs files
if ! ./code-generator/create-mod-rs-files.sh; then
  echo 'could not generate mod.rs files'
  exit 1
fi

# adjust [features] in Cargo.toml
if ! cargo run --package code-generator --bin cargo_toml_adjuster; then
  echo 'could not adjust Cargo.toml'
  exit 1
fi

# generate lib.rs
if ! cargo run --package code-generator --bin lib_rs_generator; then
  echo 'could not generate lib.rs'
  exit 1
fi
