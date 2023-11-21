#!/usr/bin/env bash

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

### Adjust Cargo.toml
sed -i '/\[features\]/,$d' ./kube-custom-resources-rs/Cargo.toml
echo '[features]' >>./kube-custom-resources-rs/Cargo.toml

find ./kube-custom-resources-rs/src -maxdepth 2 -type f -name 'mod.rs' -print0 | LC_ALL=C sort --zero-terminated | while IFS= read -r -d '' file; do
    feature=$(basename "$(dirname "${file}")")
    echo "${feature} = []"
done >> ./kube-custom-resources-rs/Cargo.toml
