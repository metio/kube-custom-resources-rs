#!/usr/bin/env bash

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

### Adjust Cargo.toml
sed -i '/\[features\]/,$d' ./kube-custom-resources-rs/Cargo.toml
echo '[features]' >>./kube-custom-resources-rs/Cargo.toml

for mld in $(find ./kube-custom-resources-rs/src -type d | LC_ALL=C sort --general-numeric-sort); do
  module=$(basename "${mld}")

  if [ -f "${mld}/mod.rs" ]; then
    echo "${module} = []" >>./kube-custom-resources-rs/Cargo.toml
  fi
done

  find ./kube-custom-resources-rs/src -type f -name 'mod.rs' -print0 | LC_ALL=C sort --zero-terminated | while IFS= read -r -d '' file; do
      module=$(dirname "${file}")
      echo "${module} = []"
  done
