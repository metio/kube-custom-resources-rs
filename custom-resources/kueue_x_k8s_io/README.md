<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# kueue.x-k8s.io

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `kueue.x-k8s.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### kueue.x-k8s.io/v1alpha1
- `Cohort`
- `MultiKueueCluster`
- `MultiKueueConfig`
### kueue.x-k8s.io/v1beta1
- `AdmissionCheck`
- `ClusterQueue`
- `Cohort`
- `LocalQueue`
- `MultiKueueCluster`
- `MultiKueueConfig`
- `ProvisioningRequestConfig`
- `ResourceFlavor`
- `WorkloadPriorityClass`
- `Workload`
