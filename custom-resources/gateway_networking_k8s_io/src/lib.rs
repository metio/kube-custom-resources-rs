/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `gateway.networking.k8s.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## gateway.networking.k8s.io/v1
- `GatewayClass`
- `Gateway`
- `GRPCRoute`
- `HTTPRoute`
## gateway.networking.k8s.io/v1alpha2
- `BackendLBPolicy`
- `GRPCRoute`
- `ReferenceGrant`
- `TCPRoute`
- `TLSRoute`
- `UDPRoute`
## gateway.networking.k8s.io/v1alpha3
- `BackendTLSPolicy`
## gateway.networking.k8s.io/v1beta1
- `GatewayClass`
- `Gateway`
- `HTTPRoute`
- `ReferenceGrant`
*/
pub mod v1;
pub mod v1alpha2;
pub mod v1alpha3;
pub mod v1beta1;
