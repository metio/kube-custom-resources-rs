#!/usr/bin/env bash

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

FILTER="${1:-}"

### Fix cargo warnings
for feature in $(cargo read-manifest --manifest-path ./kube-custom-resources-rs/Cargo.toml | yq -p json '.features | keys | .[]'); do
  if [ -n "${FILTER}" ]; then
    if ! echo -n "${feature}" | grep --quiet --word-regexp "${FILTER}"; then
      continue
    fi
  fi

  echo "fixing ${feature}"
  cargo fix --lib --package kube-custom-resources-rs --features "${feature}" --allow-no-vcs
done
