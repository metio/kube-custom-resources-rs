/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# nfd_kubernetes_io

Custom resources in this crate belong to the `nfd.kubernetes.io` group. The following versions and custom resources are available:

## nfd.kubernetes.io/v1
- `NodeFeatureDiscovery`
## nfd.kubernetes.io/v1alpha1
- `NodeFeatureRule`
*/
pub mod v1;
pub mod v1alpha1;
