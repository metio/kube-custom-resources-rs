<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# networking.istio.io

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `networking.istio.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### networking.istio.io/v1
- `DestinationRule`
- `Gateway`
- `ServiceEntry`
- `Sidecar`
- `VirtualService`
- `WorkloadEntry`
- `WorkloadGroup`
### networking.istio.io/v1alpha3
- `DestinationRule`
- `EnvoyFilter`
- `Gateway`
- `ServiceEntry`
- `Sidecar`
- `VirtualService`
- `WorkloadEntry`
- `WorkloadGroup`
### networking.istio.io/v1beta1
- `DestinationRule`
- `Gateway`
- `ProxyConfig`
- `ServiceEntry`
- `Sidecar`
- `VirtualService`
- `WorkloadEntry`
- `WorkloadGroup`
