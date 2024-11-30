/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# cassandra_datastax_com

Custom resources in this crate belong to the `cassandra.datastax.com` group. The following versions and custom resources are available:

## cassandra.datastax.com/v1beta1
- `CassandraDatacenter`
*/
pub mod v1beta1;
