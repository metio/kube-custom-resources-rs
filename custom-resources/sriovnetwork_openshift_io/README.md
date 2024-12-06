<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# sriovnetwork.openshift.io

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `sriovnetwork.openshift.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### sriovnetwork.openshift.io/v1
- `OVSNetwork`
- `SriovIBNetwork`
- `SriovNetworkNodePolicy`
- `SriovNetworkNodeState`
- `SriovNetworkPoolConfig`
- `SriovNetwork`
- `SriovOperatorConfig`
