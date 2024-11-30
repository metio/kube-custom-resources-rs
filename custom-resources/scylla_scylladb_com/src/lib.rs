/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# scylla_scylladb_com

Custom resources in this crate belong to the `scylla.scylladb.com` group. The following versions and custom resources are available:

## scylla.scylladb.com/v1
- `ScyllaCluster`
## scylla.scylladb.com/v1alpha1
- `NodeConfig`
- `ScyllaOperatorConfig`
*/
pub mod v1;
pub mod v1alpha1;
