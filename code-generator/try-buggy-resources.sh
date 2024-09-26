#!/usr/bin/env sh

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

### Try buggy resources
# read all .ignore files in the catalog
find ./crd-catalog -type f -name '*.ignore' | while IFS= read -r file; do
  # try to generate the associated .yaml and print file name for debugging
  if ! kopium --filename="${file%.*}.yaml" > /dev/null; then
    echo "  error came from file ${file%.*}.yaml"
  fi
done
