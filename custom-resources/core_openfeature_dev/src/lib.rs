/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# core_openfeature_dev

Custom resources in this crate belong to the `core.openfeature.dev` group. The following versions and custom resources are available:

## core.openfeature.dev/v1alpha1
- `FeatureFlagConfiguration`
## core.openfeature.dev/v1alpha2
- `FeatureFlagConfiguration`
*/
pub mod v1alpha1;
pub mod v1alpha2;
