/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# k6_io

Custom resources in this crate belong to the `k6.io` group. The following versions and custom resources are available:

## k6.io/v1alpha1
- `K6`
- `PrivateLoadZone`
- `TestRun`
*/
pub mod v1alpha1;
