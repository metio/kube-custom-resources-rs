#!/usr/bin/env sh

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

### Generate mod.rs files
# iterate over all features
for feature_level_directory in ./kube-custom-resources-rs/src/*; do
  if [ "$(basename "${feature_level_directory}")" = lib.rs ]; then
    continue
  fi

  # iterate over all versions within a feature
  for version_level_directory in "${feature_level_directory}"/*; do
    if [ "$(basename "${version_level_directory}")" = mod.rs ]; then
      continue
    fi

    # each version can have multiple custom resources which we add the the mod.rs file for that feature/version
    find "${version_level_directory}" -type f -name '*.rs' -not -name 'mod.rs' | LC_ALL=C sort | while IFS= read -r file; do
        crd=$(basename "${file%.*}")
        echo "pub mod ${crd};"
    done > "${version_level_directory}/mod.rs"
    if [ ! -s "${version_level_directory}/mod.rs" ]; then
      rm --force "${version_level_directory}/mod.rs"
    fi
  done

  # add each feature/version/mod.rs file to the feature/mod.rs file
  find "${feature_level_directory}" -mindepth 2 -type f -name 'mod.rs' | LC_ALL=C sort | while IFS= read -r file; do
      version=$(basename "$(dirname "${file}")")
      echo "pub mod ${version};"
  done > "${feature_level_directory}/mod.rs"
  if [ ! -s "${feature_level_directory}/mod.rs" ]; then
    rm --force "${feature_level_directory}/mod.rs"
  fi
done
