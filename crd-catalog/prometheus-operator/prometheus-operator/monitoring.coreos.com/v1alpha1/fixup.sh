#!/usr/bin/env bash

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

project=$(dirname "${0}")
echo "fixing ${project}"

properties='.spec.versions[0].schema.openAPIV3Schema.properties.spec.properties'

yq --inplace "del(${properties}.remoteWrite.items.properties.writeRelabelConfigs.items.properties.action.enum[] | select(. == \"replace\"))" \
  "${project}/prometheusagents.yaml"
yq --inplace "del(${properties}.remoteWrite.items.properties.writeRelabelConfigs.items.properties.action.enum[] | select(. == \"keep\"))" \
  "${project}/prometheusagents.yaml"
yq --inplace "del(${properties}.remoteWrite.items.properties.writeRelabelConfigs.items.properties.action.enum[] | select(. == \"drop\"))" \
  "${project}/prometheusagents.yaml"
yq --inplace "del(${properties}.remoteWrite.items.properties.writeRelabelConfigs.items.properties.action.enum[] | select(. == \"hashmod\"))" \
  "${project}/prometheusagents.yaml"
yq --inplace "del(${properties}.remoteWrite.items.properties.writeRelabelConfigs.items.properties.action.enum[] | select(. == \"labelmap\"))" \
  "${project}/prometheusagents.yaml"
yq --inplace "del(${properties}.remoteWrite.items.properties.writeRelabelConfigs.items.properties.action.enum[] | select(. == \"labeldrop\"))" \
  "${project}/prometheusagents.yaml"
yq --inplace "del(${properties}.remoteWrite.items.properties.writeRelabelConfigs.items.properties.action.enum[] | select(. == \"labelkeep\"))" \
  "${project}/prometheusagents.yaml"
yq --inplace "del(${properties}.remoteWrite.items.properties.writeRelabelConfigs.items.properties.action.enum[] | select(. == \"lowercase\"))" \
  "${project}/prometheusagents.yaml"
yq --inplace "del(${properties}.remoteWrite.items.properties.writeRelabelConfigs.items.properties.action.enum[] | select(. == \"uppercase\"))" \
  "${project}/prometheusagents.yaml"
yq --inplace "del(${properties}.remoteWrite.items.properties.writeRelabelConfigs.items.properties.action.enum[] | select(. == \"keepequal\"))" \
  "${project}/prometheusagents.yaml"
yq --inplace "del(${properties}.remoteWrite.items.properties.writeRelabelConfigs.items.properties.action.enum[] | select(. == \"dropequal\"))" \
  "${project}/prometheusagents.yaml"

yq --inplace "del(${properties}.kubernetesSDConfigs.items.properties.role.enum[] | select(. == \"node\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.kubernetesSDConfigs.items.properties.role.enum[] | select(. == \"service\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.kubernetesSDConfigs.items.properties.role.enum[] | select(. == \"pod\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.kubernetesSDConfigs.items.properties.role.enum[] | select(. == \"endpoints\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.kubernetesSDConfigs.items.properties.role.enum[] | select(. == \"endpointslice\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.kubernetesSDConfigs.items.properties.role.enum[] | select(. == \"ingress\"))" \
  "${project}/scrapeconfigs.yaml"

yq --inplace "del(${properties}.kubernetesSDConfigs.items.properties.selectors.items.properties.role.enum[] | select(. == \"node\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.kubernetesSDConfigs.items.properties.selectors.items.properties.role.enum[] | select(. == \"service\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.kubernetesSDConfigs.items.properties.selectors.items.properties.role.enum[] | select(. == \"pod\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.kubernetesSDConfigs.items.properties.selectors.items.properties.role.enum[] | select(. == \"endpoints\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.kubernetesSDConfigs.items.properties.selectors.items.properties.role.enum[] | select(. == \"endpointslice\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.kubernetesSDConfigs.items.properties.selectors.items.properties.role.enum[] | select(. == \"ingress\"))" \
  "${project}/scrapeconfigs.yaml"

yq --inplace "del(${properties}.metricRelabelings.items.properties.action.enum[] | select(. == \"replace\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.metricRelabelings.items.properties.action.enum[] | select(. == \"keep\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.metricRelabelings.items.properties.action.enum[] | select(. == \"drop\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.metricRelabelings.items.properties.action.enum[] | select(. == \"hashmod\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.metricRelabelings.items.properties.action.enum[] | select(. == \"labelmap\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.metricRelabelings.items.properties.action.enum[] | select(. == \"labeldrop\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.metricRelabelings.items.properties.action.enum[] | select(. == \"labelkeep\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.metricRelabelings.items.properties.action.enum[] | select(. == \"lowercase\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.metricRelabelings.items.properties.action.enum[] | select(. == \"uppercase\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.metricRelabelings.items.properties.action.enum[] | select(. == \"keepequal\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.metricRelabelings.items.properties.action.enum[] | select(. == \"dropequal\"))" \
  "${project}/scrapeconfigs.yaml"

yq --inplace "del(${properties}.relabelings.items.properties.action.enum[] | select(. == \"replace\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.relabelings.items.properties.action.enum[] | select(. == \"keep\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.relabelings.items.properties.action.enum[] | select(. == \"drop\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.relabelings.items.properties.action.enum[] | select(. == \"hashmod\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.relabelings.items.properties.action.enum[] | select(. == \"labelmap\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.relabelings.items.properties.action.enum[] | select(. == \"labeldrop\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.relabelings.items.properties.action.enum[] | select(. == \"labelkeep\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.relabelings.items.properties.action.enum[] | select(. == \"lowercase\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.relabelings.items.properties.action.enum[] | select(. == \"uppercase\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.relabelings.items.properties.action.enum[] | select(. == \"keepequal\"))" \
  "${project}/scrapeconfigs.yaml"
yq --inplace "del(${properties}.relabelings.items.properties.action.enum[] | select(. == \"dropequal\"))" \
  "${project}/scrapeconfigs.yaml"
