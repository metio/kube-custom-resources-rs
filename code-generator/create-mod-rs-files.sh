#!/usr/bin/env bash

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

### Generate mod.rs files
for mld in ./kube-custom-resources-rs/src/*; do
  if [ "$(basename "${mld}")" == lib.rs ]; then
    continue
  fi
  find "${mld}" -type f -name '*.rs' -not -name 'mod.rs' -print0 | LC_ALL=C sort --zero-terminated | while IFS= read -r -d '' file; do
      crd=$(basename "${file%.*}")
      echo "pub mod ${crd};"
  done > "${mld}/mod.rs"
  if [ ! -s "${mld}/mod.rs" ]; then
    rm --force "${mld}/mod.rs"
  fi
done
