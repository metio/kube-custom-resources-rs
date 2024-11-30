/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# sriovnetwork_openshift_io

Custom resources in this crate belong to the `sriovnetwork.openshift.io` group. The following versions and custom resources are available:

## sriovnetwork.openshift.io/v1
- `OVSNetwork`
- `SriovIBNetwork`
- `SriovNetworkNodePolicy`
- `SriovNetworkNodeState`
- `SriovNetworkPoolConfig`
- `SriovNetwork`
- `SriovOperatorConfig`
*/
pub mod v1;
