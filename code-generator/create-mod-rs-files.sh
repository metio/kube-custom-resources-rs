#!/usr/bin/env bash

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

### Generate mod.rs files
for fld in ./kube-custom-resources-rs/src/*; do
  if [ "$(basename "${fld}")" == lib.rs ]; then
    continue
  fi

  for vld in "${fld}"/*; do
    if [ "$(basename "${vld}")" == mod.rs ]; then
      continue
    fi

    find "${vld}" -type f -name '*.rs' -not -name 'mod.rs' -print0 | LC_ALL=C sort --zero-terminated | while IFS= read -r -d '' file; do
        crd=$(basename "${file%.*}")
        echo "pub mod ${crd};"
    done > "${vld}/mod.rs"
    if [ ! -s "${vld}/mod.rs" ]; then
      rm --force "${vld}/mod.rs"
    fi
  done

  find "${fld}" -mindepth 2 -type f -name 'mod.rs' -print0 | LC_ALL=C sort --zero-terminated | while IFS= read -r -d '' file; do
      version=$(basename "$(dirname "${file}")")
      echo "pub mod ${version};"
  done > "${fld}/mod.rs"
  if [ ! -s "${fld}/mod.rs" ]; then
    rm --force "${fld}/mod.rs"
  fi
done
