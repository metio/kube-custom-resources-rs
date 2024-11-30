/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# temporal_io

Custom resources in this crate belong to the `temporal.io` group. The following versions and custom resources are available:

## temporal.io/v1beta1
- `TemporalClusterClient`
- `TemporalNamespace`
- `TemporalWorkerProcess`
*/
pub mod v1beta1;
