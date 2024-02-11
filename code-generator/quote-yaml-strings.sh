#!/usr/bin/env bash

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

FILTER="${1:-}"

### Generate custom resources with kopium
shopt -s globstar nullglob
for file in ./crd-catalog/**/*.yaml; do
  if [ -n "${FILTER}" ]; then
    if ! echo -n "${file}" | grep --quiet "${FILTER}"; then
      continue
    fi
  fi

  yq --inplace '(.. | select(tag == "!!str") ) style="double"' "${file}"
done
