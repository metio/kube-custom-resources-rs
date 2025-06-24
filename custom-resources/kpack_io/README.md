<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# kpack.io

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `kpack.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### kpack.io/v1alpha1
- `Builder`
- `Build`
- `ClusterBuilder`
- `ClusterStack`
- `ClusterStore`
- `Image`
- `SourceResolver`
### kpack.io/v1alpha2
- `Builder`
- `Buildpack`
- `Build`
- `ClusterBuilder`
- `ClusterBuildpack`
- `ClusterLifecycle`
- `ClusterStack`
- `ClusterStore`
- `Image`
- `SourceResolver`
