<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# csi.ceph.io

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `csi.ceph.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### csi.ceph.io/v1
- `CephConnection`
- `ClientProfileMapping`
- `ClientProfile`
- `Driver`
- `OperatorConfig`
### csi.ceph.io/v1alpha1
- `CephConnection`
- `ClientProfileMapping`
- `ClientProfile`
- `Driver`
- `OperatorConfig`
