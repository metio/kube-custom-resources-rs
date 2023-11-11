#!/usr/bin/env bash

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

FILTER="${1:-}"

### Test custom resources
for feature in $(cargo read-manifest --manifest-path ./kube-custom-resources-rs/Cargo.toml | yq -p json '.features | keys | .[]'); do
  if [ -n "${FILTER}" ]; then
    if ! echo -n "${feature}" | grep --quiet "${FILTER}"; then
      continue
    fi
  fi

  echo "testing ${feature}"
  cargo check --lib --package kube-custom-resources-rs --features "${feature}" --locked
done
