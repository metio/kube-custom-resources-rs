#!/usr/bin/env bash

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

FILTER="${1:-}"

# fix YAMLs
shopt -s globstar nullglob
for file in ./crd-catalog/**/fixup.sh; do
  if [ -n "${FILTER}" ]; then
    if ! echo -n "${file}" | grep --quiet "${FILTER}"; then
      continue
    fi
  fi

  "${file}"
done
