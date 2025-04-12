<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# apps.kubeblocks.io

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `apps.kubeblocks.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### apps.kubeblocks.io/v1
- `ClusterDefinition`
- `Cluster`
- `ComponentDefinition`
- `Component`
- `ComponentVersion`
- `ServiceDescriptor`
- `ShardingDefinition`
- `SidecarDefinition`
### apps.kubeblocks.io/v1alpha1
- `BackupPolicyTemplate`
- `ClusterDefinition`
- `Cluster`
- `ClusterVersion`
- `ComponentClassDefinition`
- `ComponentDefinition`
- `Component`
- `ComponentVersion`
- `ConfigConstraint`
- `Configuration`
- `OpsDefinition`
- `OpsRequest`
- `ServiceDescriptor`
### apps.kubeblocks.io/v1beta1
- `ConfigConstraint`
