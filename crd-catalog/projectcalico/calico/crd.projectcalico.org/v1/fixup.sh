#!/usr/bin/env bash

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

project=$(dirname "${0}")
echo "fixing ${project}"

properties='.spec.versions[0].schema.openAPIV3Schema.properties.spec.properties'

yq --inplace "del(${properties}.nat-outgoing)" \
  "${project}/ippools.yaml"
