#!/usr/bin/env bash

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

project=$(dirname "${0}")
echo "fixing ${project}"

properties='.spec.versions[0].schema.openAPIV3Schema.properties.spec.properties'

yq --inplace "del(${properties}.podMetricsEndpoints.items.properties.metricRelabelings.items.properties.action.enum[] | select(. == \"replace\"))" \
  "${project}/podmonitors.yaml"
yq --inplace "del(${properties}.podMetricsEndpoints.items.properties.metricRelabelings.items.properties.action.enum[] | select(. == \"keep\"))" \
  "${project}/podmonitors.yaml"
yq --inplace "del(${properties}.podMetricsEndpoints.items.properties.metricRelabelings.items.properties.action.enum[] | select(. == \"drop\"))" \
  "${project}/podmonitors.yaml"
yq --inplace "del(${properties}.podMetricsEndpoints.items.properties.metricRelabelings.items.properties.action.enum[] | select(. == \"hashmod\"))" \
  "${project}/podmonitors.yaml"
yq --inplace "del(${properties}.podMetricsEndpoints.items.properties.metricRelabelings.items.properties.action.enum[] | select(. == \"labelmap\"))" \
  "${project}/podmonitors.yaml"
yq --inplace "del(${properties}.podMetricsEndpoints.items.properties.metricRelabelings.items.properties.action.enum[] | select(. == \"labeldrop\"))" \
  "${project}/podmonitors.yaml"
yq --inplace "del(${properties}.podMetricsEndpoints.items.properties.metricRelabelings.items.properties.action.enum[] | select(. == \"labelkeep\"))" \
  "${project}/podmonitors.yaml"
yq --inplace "del(${properties}.podMetricsEndpoints.items.properties.metricRelabelings.items.properties.action.enum[] | select(. == \"lowercase\"))" \
  "${project}/podmonitors.yaml"
yq --inplace "del(${properties}.podMetricsEndpoints.items.properties.metricRelabelings.items.properties.action.enum[] | select(. == \"uppercase\"))" \
  "${project}/podmonitors.yaml"
yq --inplace "del(${properties}.podMetricsEndpoints.items.properties.metricRelabelings.items.properties.action.enum[] | select(. == \"keepequal\"))" \
  "${project}/podmonitors.yaml"
yq --inplace "del(${properties}.podMetricsEndpoints.items.properties.metricRelabelings.items.properties.action.enum[] | select(. == \"dropequal\"))" \
  "${project}/podmonitors.yaml"

yq --inplace "del(${properties}.podMetricsEndpoints.items.properties.relabelings.items.properties.action.enum[] | select(. == \"replace\"))" \
  "${project}/podmonitors.yaml"
yq --inplace "del(${properties}.podMetricsEndpoints.items.properties.relabelings.items.properties.action.enum[] | select(. == \"keep\"))" \
  "${project}/podmonitors.yaml"
yq --inplace "del(${properties}.podMetricsEndpoints.items.properties.relabelings.items.properties.action.enum[] | select(. == \"drop\"))" \
  "${project}/podmonitors.yaml"
yq --inplace "del(${properties}.podMetricsEndpoints.items.properties.relabelings.items.properties.action.enum[] | select(. == \"hashmod\"))" \
  "${project}/podmonitors.yaml"
yq --inplace "del(${properties}.podMetricsEndpoints.items.properties.relabelings.items.properties.action.enum[] | select(. == \"labelmap\"))" \
  "${project}/podmonitors.yaml"
yq --inplace "del(${properties}.podMetricsEndpoints.items.properties.relabelings.items.properties.action.enum[] | select(. == \"labeldrop\"))" \
  "${project}/podmonitors.yaml"
yq --inplace "del(${properties}.podMetricsEndpoints.items.properties.relabelings.items.properties.action.enum[] | select(. == \"labelkeep\"))" \
  "${project}/podmonitors.yaml"
yq --inplace "del(${properties}.podMetricsEndpoints.items.properties.relabelings.items.properties.action.enum[] | select(. == \"lowercase\"))" \
  "${project}/podmonitors.yaml"
yq --inplace "del(${properties}.podMetricsEndpoints.items.properties.relabelings.items.properties.action.enum[] | select(. == \"uppercase\"))" \
  "${project}/podmonitors.yaml"
yq --inplace "del(${properties}.podMetricsEndpoints.items.properties.relabelings.items.properties.action.enum[] | select(. == \"keepequal\"))" \
  "${project}/podmonitors.yaml"
yq --inplace "del(${properties}.podMetricsEndpoints.items.properties.relabelings.items.properties.action.enum[] | select(. == \"dropequal\"))" \
  "${project}/podmonitors.yaml"

yq --inplace "del(${properties}.endpoints.items.properties.metricRelabelings.items.properties.action.enum[] | select(. == \"replace\"))" \
  "${project}/servicemonitors.yaml"
yq --inplace "del(${properties}.endpoints.items.properties.metricRelabelings.items.properties.action.enum[] | select(. == \"keep\"))" \
  "${project}/servicemonitors.yaml"
yq --inplace "del(${properties}.endpoints.items.properties.metricRelabelings.items.properties.action.enum[] | select(. == \"drop\"))" \
  "${project}/servicemonitors.yaml"
yq --inplace "del(${properties}.endpoints.items.properties.metricRelabelings.items.properties.action.enum[] | select(. == \"hashmod\"))" \
  "${project}/servicemonitors.yaml"
yq --inplace "del(${properties}.endpoints.items.properties.metricRelabelings.items.properties.action.enum[] | select(. == \"labelmap\"))" \
  "${project}/servicemonitors.yaml"
yq --inplace "del(${properties}.endpoints.items.properties.metricRelabelings.items.properties.action.enum[] | select(. == \"labeldrop\"))" \
  "${project}/servicemonitors.yaml"
yq --inplace "del(${properties}.endpoints.items.properties.metricRelabelings.items.properties.action.enum[] | select(. == \"labelkeep\"))" \
  "${project}/servicemonitors.yaml"
yq --inplace "del(${properties}.endpoints.items.properties.metricRelabelings.items.properties.action.enum[] | select(. == \"lowercase\"))" \
  "${project}/servicemonitors.yaml"
yq --inplace "del(${properties}.endpoints.items.properties.metricRelabelings.items.properties.action.enum[] | select(. == \"uppercase\"))" \
  "${project}/servicemonitors.yaml"
yq --inplace "del(${properties}.endpoints.items.properties.metricRelabelings.items.properties.action.enum[] | select(. == \"keepequal\"))" \
  "${project}/servicemonitors.yaml"
yq --inplace "del(${properties}.endpoints.items.properties.metricRelabelings.items.properties.action.enum[] | select(. == \"dropequal\"))" \
  "${project}/servicemonitors.yaml"

yq --inplace "del(${properties}.endpoints.items.properties.relabelings.items.properties.action.enum[] | select(. == \"replace\"))" \
  "${project}/servicemonitors.yaml"
yq --inplace "del(${properties}.endpoints.items.properties.relabelings.items.properties.action.enum[] | select(. == \"keep\"))" \
  "${project}/servicemonitors.yaml"
yq --inplace "del(${properties}.endpoints.items.properties.relabelings.items.properties.action.enum[] | select(. == \"drop\"))" \
  "${project}/servicemonitors.yaml"
yq --inplace "del(${properties}.endpoints.items.properties.relabelings.items.properties.action.enum[] | select(. == \"hashmod\"))" \
  "${project}/servicemonitors.yaml"
yq --inplace "del(${properties}.endpoints.items.properties.relabelings.items.properties.action.enum[] | select(. == \"labelmap\"))" \
  "${project}/servicemonitors.yaml"
yq --inplace "del(${properties}.endpoints.items.properties.relabelings.items.properties.action.enum[] | select(. == \"labeldrop\"))" \
  "${project}/servicemonitors.yaml"
yq --inplace "del(${properties}.endpoints.items.properties.relabelings.items.properties.action.enum[] | select(. == \"labelkeep\"))" \
  "${project}/servicemonitors.yaml"
yq --inplace "del(${properties}.endpoints.items.properties.relabelings.items.properties.action.enum[] | select(. == \"lowercase\"))" \
  "${project}/servicemonitors.yaml"
yq --inplace "del(${properties}.endpoints.items.properties.relabelings.items.properties.action.enum[] | select(. == \"uppercase\"))" \
  "${project}/servicemonitors.yaml"
yq --inplace "del(${properties}.endpoints.items.properties.relabelings.items.properties.action.enum[] | select(. == \"keepequal\"))" \
  "${project}/servicemonitors.yaml"
yq --inplace "del(${properties}.endpoints.items.properties.relabelings.items.properties.action.enum[] | select(. == \"dropequal\"))" \
  "${project}/servicemonitors.yaml"

yq --inplace "del(${properties}.metricRelabelings.items.properties.action.enum[] | select(. == \"replace\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.metricRelabelings.items.properties.action.enum[] | select(. == \"keep\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.metricRelabelings.items.properties.action.enum[] | select(. == \"drop\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.metricRelabelings.items.properties.action.enum[] | select(. == \"hashmod\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.metricRelabelings.items.properties.action.enum[] | select(. == \"labelmap\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.metricRelabelings.items.properties.action.enum[] | select(. == \"labeldrop\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.metricRelabelings.items.properties.action.enum[] | select(. == \"labelkeep\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.metricRelabelings.items.properties.action.enum[] | select(. == \"lowercase\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.metricRelabelings.items.properties.action.enum[] | select(. == \"uppercase\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.metricRelabelings.items.properties.action.enum[] | select(. == \"keepequal\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.metricRelabelings.items.properties.action.enum[] | select(. == \"dropequal\"))" \
  "${project}/probes.yaml"

yq --inplace "del(${properties}.targets.properties.ingress.properties.relabelingConfigs.items.properties.action.enum[] | select(. == \"replace\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.targets.properties.ingress.properties.relabelingConfigs.items.properties.action.enum[] | select(. == \"keep\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.targets.properties.ingress.properties.relabelingConfigs.items.properties.action.enum[] | select(. == \"drop\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.targets.properties.ingress.properties.relabelingConfigs.items.properties.action.enum[] | select(. == \"hashmod\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.targets.properties.ingress.properties.relabelingConfigs.items.properties.action.enum[] | select(. == \"labelmap\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.targets.properties.ingress.properties.relabelingConfigs.items.properties.action.enum[] | select(. == \"labeldrop\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.targets.properties.ingress.properties.relabelingConfigs.items.properties.action.enum[] | select(. == \"labelkeep\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.targets.properties.ingress.properties.relabelingConfigs.items.properties.action.enum[] | select(. == \"lowercase\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.targets.properties.ingress.properties.relabelingConfigs.items.properties.action.enum[] | select(. == \"uppercase\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.targets.properties.ingress.properties.relabelingConfigs.items.properties.action.enum[] | select(. == \"keepequal\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.targets.properties.ingress.properties.relabelingConfigs.items.properties.action.enum[] | select(. == \"dropequal\"))" \
  "${project}/probes.yaml"

yq --inplace "del(${properties}.targets.properties.staticConfig.properties.relabelingConfigs.items.properties.action.enum[] | select(. == \"replace\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.targets.properties.staticConfig.properties.relabelingConfigs.items.properties.action.enum[] | select(. == \"keep\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.targets.properties.staticConfig.properties.relabelingConfigs.items.properties.action.enum[] | select(. == \"drop\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.targets.properties.staticConfig.properties.relabelingConfigs.items.properties.action.enum[] | select(. == \"hashmod\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.targets.properties.staticConfig.properties.relabelingConfigs.items.properties.action.enum[] | select(. == \"labelmap\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.targets.properties.staticConfig.properties.relabelingConfigs.items.properties.action.enum[] | select(. == \"labeldrop\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.targets.properties.staticConfig.properties.relabelingConfigs.items.properties.action.enum[] | select(. == \"labelkeep\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.targets.properties.staticConfig.properties.relabelingConfigs.items.properties.action.enum[] | select(. == \"lowercase\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.targets.properties.staticConfig.properties.relabelingConfigs.items.properties.action.enum[] | select(. == \"uppercase\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.targets.properties.staticConfig.properties.relabelingConfigs.items.properties.action.enum[] | select(. == \"keepequal\"))" \
  "${project}/probes.yaml"
yq --inplace "del(${properties}.targets.properties.staticConfig.properties.relabelingConfigs.items.properties.action.enum[] | select(. == \"dropequal\"))" \
  "${project}/probes.yaml"

yq --inplace "del(${properties}.remoteWrite.items.properties.writeRelabelConfigs.items.properties.action.enum[] | select(. == \"replace\"))" \
  "${project}/prometheuses.yaml"
yq --inplace "del(${properties}.remoteWrite.items.properties.writeRelabelConfigs.items.properties.action.enum[] | select(. == \"keep\"))" \
  "${project}/prometheuses.yaml"
yq --inplace "del(${properties}.remoteWrite.items.properties.writeRelabelConfigs.items.properties.action.enum[] | select(. == \"drop\"))" \
  "${project}/prometheuses.yaml"
yq --inplace "del(${properties}.remoteWrite.items.properties.writeRelabelConfigs.items.properties.action.enum[] | select(. == \"hashmod\"))" \
  "${project}/prometheuses.yaml"
yq --inplace "del(${properties}.remoteWrite.items.properties.writeRelabelConfigs.items.properties.action.enum[] | select(. == \"labelmap\"))" \
  "${project}/prometheuses.yaml"
yq --inplace "del(${properties}.remoteWrite.items.properties.writeRelabelConfigs.items.properties.action.enum[] | select(. == \"labeldrop\"))" \
  "${project}/prometheuses.yaml"
yq --inplace "del(${properties}.remoteWrite.items.properties.writeRelabelConfigs.items.properties.action.enum[] | select(. == \"labelkeep\"))" \
  "${project}/prometheuses.yaml"
yq --inplace "del(${properties}.remoteWrite.items.properties.writeRelabelConfigs.items.properties.action.enum[] | select(. == \"lowercase\"))" \
  "${project}/prometheuses.yaml"
yq --inplace "del(${properties}.remoteWrite.items.properties.writeRelabelConfigs.items.properties.action.enum[] | select(. == \"uppercase\"))" \
  "${project}/prometheuses.yaml"
yq --inplace "del(${properties}.remoteWrite.items.properties.writeRelabelConfigs.items.properties.action.enum[] | select(. == \"keepequal\"))" \
  "${project}/prometheuses.yaml"
yq --inplace "del(${properties}.remoteWrite.items.properties.writeRelabelConfigs.items.properties.action.enum[] | select(. == \"dropequal\"))" \
  "${project}/prometheuses.yaml"

