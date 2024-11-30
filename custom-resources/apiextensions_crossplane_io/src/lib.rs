/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# apiextensions_crossplane_io

Custom resources in this crate belong to the `apiextensions.crossplane.io` group. The following versions and custom resources are available:

## apiextensions.crossplane.io/v1
- `CompositeResourceDefinition`
*/
pub mod v1;
