#!/usr/bin/env sh

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

FILTER="${1:-}"

### Test custom resources
# read all features declared in the Cargo manifest
for feature in $(cargo read-manifest --manifest-path ./kube-custom-resources-rs/Cargo.toml | yq -p json '.features | keys | .[]'); do
  if [ -n "${FILTER}" ]; then
    if ! printf '%s' "${feature}" | grep --quiet --word-regexp "${FILTER}"; then
      continue
    fi
  fi

  # run cargo check against each feature
  echo "testing ${feature}"
  if K8S_OPENAPI_ENABLED_VERSION=1.31 cargo check --lib --package kube-custom-resources-rs --features "${feature}" --locked; then
    echo "${feature} succeeded"
  else
    echo "${feature} failed"
    exit 1
  fi
done
