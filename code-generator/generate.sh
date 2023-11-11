#!/usr/bin/env bash

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

FILTER="${1:-}"

cargo run --package code-generator --bin crd_v1_fetcher "${FILTER}"
cargo run --package code-generator --bin dep5_generator
find ./crd-catalog -name 'fixup.sh' -type f -exec {} \;
./code-generator/create-custom-resources.sh "${FILTER}"
./code-generator/create-mod-rs-files.sh
./code-generator/adjust-cargo-toml.sh
cargo run --package code-generator --bin lib_rs_generator
