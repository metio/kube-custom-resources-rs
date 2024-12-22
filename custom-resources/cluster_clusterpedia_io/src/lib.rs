/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `cluster.clusterpedia.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## cluster.clusterpedia.io/v1alpha2
- `ClusterSyncResources`
- `PediaCluster`
*/
#[cfg(feature = "v1alpha2")]
pub mod v1alpha2;
