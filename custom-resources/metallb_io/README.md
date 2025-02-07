<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# metallb.io

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `metallb.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### metallb.io/v1beta1
- `BFDProfile`
- `BGPAdvertisement`
- `BGPPeer`
- `Community`
- `IPAddressPool`
- `L2Advertisement`
- `MetalLB`
- `ServiceL2Status`
### metallb.io/v1beta2
- `BGPPeer`
