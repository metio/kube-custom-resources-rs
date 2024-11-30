/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# control_k8ssandra_io

Custom resources in this crate belong to the `control.k8ssandra.io` group. The following versions and custom resources are available:

## control.k8ssandra.io/v1alpha1
- `CassandraTask`
*/
pub mod v1alpha1;
