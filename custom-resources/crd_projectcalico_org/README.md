<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# crd.projectcalico.org

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `crd.projectcalico.org` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### crd.projectcalico.org/v1
- `BGPConfiguration`
- `BGPFilter`
- `BGPPeer`
- `BlockAffinity`
- `CalicoNodeStatus`
- `ClusterInformation`
- `GlobalNetworkSet`
- `HostEndpoint`
- `IPAMBlock`
- `IPAMConfig`
- `IPAMHandle`
- `IPPool`
- `IPReservation`
- `KubeControllersConfiguration`
- `NetworkSet`
- `Tier`
