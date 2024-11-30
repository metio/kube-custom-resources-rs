/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# clusters_clusternet_io

Custom resources in this crate belong to the `clusters.clusternet.io` group. The following versions and custom resources are available:

## clusters.clusternet.io/v1beta1
- `ClusterRegistrationRequest`
- `ManagedCluster`
*/
pub mod v1beta1;
