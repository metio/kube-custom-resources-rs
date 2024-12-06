#!/usr/bin/env sh

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

FILTER="${1:-}"

### Test custom resources
for group_level_directory in ./custom-resources/*; do
  group=$(basename "${group_level_directory}")

  if [ -n "${FILTER}" ]; then
    if ! printf '%s' "${group}" | grep --quiet --word-regexp "${FILTER}"; then
      continue
    fi
  fi

  # apply auto-fixable fixes
  echo "testing ${group}"
  if K8S_OPENAPI_ENABLED_VERSION=1.31 cargo check --lib --package "kcr_${group}" --locked; then
    echo "${group} succeeded"
  else
    echo "${group} failed"
    exit 1
  fi
done
