<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# gateway.networking.k8s.io

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `gateway.networking.k8s.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### gateway.networking.k8s.io/v1
- `GatewayClass`
- `Gateway`
- `GRPCRoute`
- `HTTPRoute`
### gateway.networking.k8s.io/v1alpha2
- `BackendLBPolicy`
- `GRPCRoute`
- `ReferenceGrant`
- `TCPRoute`
- `TLSRoute`
- `UDPRoute`
### gateway.networking.k8s.io/v1alpha3
- `BackendTLSPolicy`
- `TLSRoute`
### gateway.networking.k8s.io/v1beta1
- `GatewayClass`
- `Gateway`
- `HTTPRoute`
- `ReferenceGrant`
