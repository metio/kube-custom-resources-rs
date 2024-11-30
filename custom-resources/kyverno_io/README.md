<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# kyverno.io

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `kyverno.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### kyverno.io/v1
- `ClusterPolicy`
- `Policy`
### kyverno.io/v1alpha2
- `AdmissionReport`
- `BackgroundScanReport`
- `ClusterAdmissionReport`
- `ClusterBackgroundScanReport`
### kyverno.io/v1beta1
- `UpdateRequest`
### kyverno.io/v2
- `AdmissionReport`
- `BackgroundScanReport`
- `CleanupPolicy`
- `ClusterAdmissionReport`
- `ClusterBackgroundScanReport`
- `ClusterCleanupPolicy`
- `PolicyException`
- `UpdateRequest`
### kyverno.io/v2alpha1
- `CleanupPolicy`
- `ClusterCleanupPolicy`
- `GlobalContextEntry`
- `PolicyException`
### kyverno.io/v2beta1
- `CleanupPolicy`
- `ClusterCleanupPolicy`
- `ClusterPolicy`
- `Policy`
- `PolicyException`
