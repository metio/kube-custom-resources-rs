/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# egressgateway_spidernet_io

Custom resources in this crate belong to the `egressgateway.spidernet.io` group. The following versions and custom resources are available:

## egressgateway.spidernet.io/v1beta1
- `EgressClusterEndpointSlice`
- `EgressClusterInfo`
- `EgressClusterPolicy`
- `EgressEndpointSlice`
- `EgressGateway`
- `EgressPolicy`
- `EgressTunnel`
*/
pub mod v1beta1;
