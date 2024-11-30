<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# multicluster.crd.antrea.io

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `multicluster.crd.antrea.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### multicluster.crd.antrea.io/v1alpha1
- `ClusterInfoImport`
- `ClusterSet`
- `Gateway`
- `LabelIdentity`
- `MemberClusterAnnounce`
- `MultiClusterConfig`
- `ResourceExport`
- `ResourceImport`
### multicluster.crd.antrea.io/v1alpha2
- `ClusterClaim`
- `ClusterSet`
