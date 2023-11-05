#!/usr/bin/env bash

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

project=$(dirname "${0}")
echo "fixing ${project}"

properties='.spec.versions[0].schema.openAPIV3Schema.properties.spec.properties'

yq --inplace "del(${properties}.init_containers)" \
  "${project}/postgresqls.yaml"
yq --inplace "del(${properties}.initContainers)" \
  "${project}/postgresqls.yaml"
yq --inplace "del(${properties}.pod_priority_class_name)" \
  "${project}/postgresqls.yaml"
yq --inplace "del(${properties}.env)" \
  "${project}/postgresqls.yaml"
yq --inplace "del(${properties}.sidecars)" \
  "${project}/postgresqls.yaml"
yq --inplace "del(${properties}.patroni.properties.slots)" \
  "${project}/postgresqls.yaml"
