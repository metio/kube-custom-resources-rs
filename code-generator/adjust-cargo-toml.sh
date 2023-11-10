#!/usr/bin/env bash

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

### Adjust Cargo.toml
sed -i '/\[features\]/,$d' ./kube-custom-resources-rs/Cargo.toml
echo '[features]' >>./kube-custom-resources-rs/Cargo.toml

find ./kube-custom-resources-rs/src -type f -name 'mod.rs' -print0 | LC_ALL=C sort --zero-terminated | while IFS= read -r -d '' file; do
    module=$(basename "$(dirname "${file}")")
    echo "${module} = []"
done >> ./kube-custom-resources-rs/Cargo.toml
