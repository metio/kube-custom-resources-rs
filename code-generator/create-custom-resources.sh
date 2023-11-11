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

  path="${file%.*}"
  ignore_file="${path}.ignore"

  if [ ! -f "${ignore_file}" ]; then
    args="${path}.args"
    crd=$(basename "${path}")
    version=$(basename "$(dirname "${file}")")
    group=$(basename "$(dirname "$(dirname "${file}")")")
    rust_crd=$(echo "${crd}" | sed -e 's/\./_/g' -e 's/-/_/g')
    rust_group=$(echo "${group}" | sed -e 's/\./_/g' -e 's/-/_/g')
    module="${rust_group}_${version}"

    mkdir --parents "./kube-custom-resources-rs/src/${module}"

    if [ -f "${args}" ]; then
      if ! xargs --arg-file="${args}" --delimiter='\n' kopium --docs --filename="${file}" > "./kube-custom-resources-rs/src/${module}/${rust_crd}.rs"; then
        echo "error in ${file}"
      fi
    else
      if ! kopium --docs --filename="${file}" --derive=Default --derive=PartialEq > "./kube-custom-resources-rs/src/${module}/${rust_crd}.rs"; then
        echo "error in ${file}"
      fi
    fi
  fi
done
