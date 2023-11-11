#!/usr/bin/env bash

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

### Fix cargo warnings
for mld in ./kube-custom-resources-rs/src/*; do
  if [ -f "${mld}/mod.rs" ]; then
    module=$(basename "${mld}")
    echo "fixing ${module}"
    cargo fix --lib --package kube-custom-resources-rs --features "${module}" --allow-no-vcs
  fi
done
