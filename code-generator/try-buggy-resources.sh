#!/usr/bin/env bash

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

### Try buggy resources
shopt -s globstar nullglob
for file in ./crd-catalog/**/*.ignore; do
  if ! kopium --filename="${file%.*}.yaml" > /dev/null; then
    echo "  error came from file ${file%.*}.yaml"
  fi
done
