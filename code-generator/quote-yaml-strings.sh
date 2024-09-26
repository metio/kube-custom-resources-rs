#!/usr/bin/env sh

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

FILTER="${1:-}"

### Generate custom resources with kopium
# iterate over all yaml files in catalog
find ./crd-catalog -type f -name '*.yaml' | while IFS= read -r file; do
  if [ -n "${FILTER}" ]; then
    if ! printf '%s'  "${file}" | grep --quiet "${FILTER}"; then
      continue
    fi
  fi

  # ensure that yaml files can be read with yaml 1.1 compliant parsers (like Go's sigs.k8s.io/yaml)
  yq --inplace '(.. | select(tag == "!!str") ) style="double"' "${file}"
done
