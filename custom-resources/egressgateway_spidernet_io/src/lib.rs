/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `egressgateway.spidernet.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## egressgateway.spidernet.io/v1beta1
- `EgressClusterEndpointSlice`
- `EgressClusterInfo`
- `EgressClusterPolicy`
- `EgressEndpointSlice`
- `EgressGateway`
- `EgressPolicy`
- `EgressTunnel`
*/
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
