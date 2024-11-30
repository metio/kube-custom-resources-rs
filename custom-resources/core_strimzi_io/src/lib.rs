/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# core_strimzi_io

Custom resources in this crate belong to the `core.strimzi.io` group. The following versions and custom resources are available:

## core.strimzi.io/v1beta2
- `StrimziPodSet`
*/
pub mod v1beta2;
