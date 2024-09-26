#!/usr/bin/env sh

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

FILTER="${1:-}"

### Fix cargo warnings
# read all features declared in the Cargo manifest
for feature in $(cargo read-manifest --manifest-path ./kube-custom-resources-rs/Cargo.toml | yq -p json '.features | keys | .[]'); do
  if [ -n "${FILTER}" ]; then
    if ! printf '%s' "${feature}" | grep --quiet --word-regexp "${FILTER}"; then
      continue
    fi
  fi

  # apply auto-fixable fixes
  echo "fixing ${feature}"
  K8S_OPENAPI_ENABLED_VERSION=1.31 cargo fix --lib --package kube-custom-resources-rs --features "${feature}" --allow-no-vcs
done
