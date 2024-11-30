/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# asdb_aerospike_com

Custom resources in this crate belong to the `asdb.aerospike.com` group. The following versions and custom resources are available:

## asdb.aerospike.com/v1
- `AerospikeCluster`
## asdb.aerospike.com/v1beta1
- `AerospikeCluster`
*/
pub mod v1;
pub mod v1beta1;
