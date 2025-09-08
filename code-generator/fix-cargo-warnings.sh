#!/usr/bin/env sh

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

FILTER="${1:-}"

### Fix cargo warnings
for group_level_directory in ./custom-resources/*; do
  group=$(basename "${group_level_directory}")

  if [ -n "${FILTER}" ]; then
    if ! printf '%s' "${group}" | grep --quiet --word-regexp "${FILTER}"; then
      continue
    fi
  fi

  # apply auto-fixable fixes
  echo "fixing ${group}"
  K8S_OPENAPI_ENABLED_VERSION=1.34 cargo fix --lib --package "kcr_${group}" --allow-no-vcs
done
