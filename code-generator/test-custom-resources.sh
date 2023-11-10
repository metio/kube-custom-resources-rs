#!/usr/bin/env bash

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

### Test custom resources
for mld in ./kube-custom-resources-rs/src/*; do
  if [ -f "${mld}/mod.rs" ]; then
    module=$(basename "${mld}")
    echo "testing ${module}"
    cargo check --package kube-custom-resources-rs --features "${module}" --locked
  fi
done
