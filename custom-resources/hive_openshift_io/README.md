<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# hive.openshift.io

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `hive.openshift.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### hive.openshift.io/v1
- `Checkpoint`
- `ClusterClaim`
- `ClusterDeploymentCustomization`
- `ClusterDeployment`
- `ClusterDeprovision`
- `ClusterImageSet`
- `ClusterPool`
- `ClusterProvision`
- `ClusterRelocate`
- `ClusterState`
- `DNSZone`
- `HiveConfig`
- `MachinePoolNameLease`
- `MachinePool`
- `SelectorSyncIdentityProvider`
- `SyncIdentityProvider`
