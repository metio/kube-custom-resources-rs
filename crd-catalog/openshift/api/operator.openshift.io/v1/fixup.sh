#!/usr/bin/env bash

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

project=$(dirname "${0}")
echo "fixing ${project}"

brand='.spec.versions[0].schema.openAPIV3Schema.properties.spec.properties.customization.properties.brand'

yq --inplace "del(${brand}.enum[] | select(. == \"online\"))" \
  "${project}/consoles.yaml"
yq --inplace "del(${brand}.enum[] | select(. == \"okd\"))" \
  "${project}/consoles.yaml"
yq --inplace "del(${brand}.enum[] | select(. == \"ocp\"))" \
  "${project}/consoles.yaml"
yq --inplace "del(${brand}.enum[] | select(. == \"dedicated\"))" \
  "${project}/consoles.yaml"
yq --inplace "del(${brand}.enum[] | select(. == \"azure\"))" \
  "${project}/consoles.yaml"
