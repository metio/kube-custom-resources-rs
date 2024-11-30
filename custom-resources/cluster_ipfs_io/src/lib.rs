/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# cluster_ipfs_io

Custom resources in this crate belong to the `cluster.ipfs.io` group. The following versions and custom resources are available:

## cluster.ipfs.io/v1alpha1
- `CircuitRelay`
- `IpfsCluster`
*/
pub mod v1alpha1;
