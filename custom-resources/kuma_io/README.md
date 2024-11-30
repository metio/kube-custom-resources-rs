<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# kuma.io

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `kuma.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### kuma.io/v1alpha1
- `CircuitBreaker`
- `ContainerPatch`
- `DataplaneInsight`
- `Dataplane`
- `ExternalService`
- `FaultInjection`
- `HealthCheck`
- `MeshAccessLog`
- `MeshCircuitBreaker`
- `Mesh`
- `MeshFaultInjection`
- `MeshGatewayConfig`
- `MeshGatewayInstance`
- `MeshGatewayRoute`
- `MeshGateway`
- `MeshHealthCheck`
- `MeshHTTPRoute`
- `MeshInsight`
- `MeshLoadBalancingStrategy`
- `MeshProxyPatch`
- `MeshRateLimit`
- `MeshRetry`
- `MeshTCPRoute`
- `MeshTimeout`
- `MeshTrace`
- `MeshTrafficPermission`
- `ProxyTemplate`
- `RateLimit`
- `Retry`
- `ServiceInsight`
- `Timeout`
- `TrafficLog`
- `TrafficPermission`
- `TrafficRoute`
- `TrafficTrace`
- `VirtualOutbound`
- `ZoneEgress`
- `ZoneEgressInsight`
- `ZoneIngress`
- `ZoneIngressInsight`
- `ZoneInsight`
- `Zone`
