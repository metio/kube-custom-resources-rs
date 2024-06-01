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
  fixup_file="${path}.fixup"
  fixed_file="${path}.fixed"
  file_to_read="${file}"
  crd=$(basename "${path}")
  version=$(basename "$(dirname "${file}")")
  group=$(basename "$(dirname "$(dirname "${file}")")")
  resource_filename=$(echo "${crd}" | sed -e 's/\./_/g' -e 's/-/_/g')
  cargo_group=$(echo "${group}" | sed -e 's/\./_/g' -e 's/-/_/g')
  cargo_feature="${cargo_group}"
  feature_directory="./kube-custom-resources-rs/src/${cargo_feature}"
  version_directory="${feature_directory}/${version}"

  if [ -f "${ignore_file}" ]; then
    rm --force "${version_directory}/${resource_filename}.rs"
  else
    mkdir --parents "${version_directory}"

    if [ -f "${fixup_file}" ]; then
      "${fixup_file}" > "${fixed_file}"
      file_to_read="${fixed_file}"
    fi

    if ! kopium --docs --filename="${file_to_read}" --derive=Default --derive=PartialEq --smart-derive-elision > "${version_directory}/${resource_filename}.rs"; then
      echo "  error in ${file_to_read}"
    fi
  fi
done
