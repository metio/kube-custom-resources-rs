#!/usr/bin/env bash

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

FILTER="${1:-}"

# fetch CRDs
cargo run --package code-generator --bin crd_v1_fetcher "${FILTER}"

# generate dep5 file
cargo run --package code-generator --bin dep5_generator

# fix YAMLs
shopt -s globstar nullglob
for file in ./crd-catalog/**/fixup.sh; do
  if [ -n "${FILTER}" ]; then
    if ! echo -n "${file}" | grep --quiet "${FILTER}"; then
      continue
    fi
  fi

  "${file}"
done

# generate Rust code
./code-generator/create-custom-resources.sh "${FILTER}"

# generate mod.rs files
./code-generator/create-mod-rs-files.sh

# generate Cargo.toml
./code-generator/adjust-cargo-toml.sh

# generate lib.rs
cargo run --package code-generator --bin lib_rs_generator
