<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# cilium.io

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `cilium.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### cilium.io/v2
- `CiliumClusterwideNetworkPolicy`
- `CiliumEgressGatewayPolicy`
- `CiliumEndpoint`
- `CiliumExternalWorkload`
- `CiliumIdentity`
- `CiliumLocalRedirectPolicy`
- `CiliumNetworkPolicy`
- `CiliumNode`
### cilium.io/v2alpha1
- `CiliumBGPPeeringPolicy`
- `CiliumCIDRGroup`
- `CiliumEndpointSlice`
- `CiliumL2AnnouncementPolicy`
- `CiliumLoadBalancerIPPool`
- `CiliumNodeConfig`
- `CiliumPodIPPool`
