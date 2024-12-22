/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `asdb.aerospike.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## asdb.aerospike.com/v1
- `AerospikeCluster`
## asdb.aerospike.com/v1beta1
- `AerospikeCluster`
*/
#[cfg(feature = "v1")]
pub mod v1;
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
