/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# cluster_clusterpedia_io

Custom resources in this crate belong to the `cluster.clusterpedia.io` group. The following versions and custom resources are available:

## cluster.clusterpedia.io/v1alpha2
- `ClusterSyncResources`
- `PediaCluster`
*/
pub mod v1alpha2;
