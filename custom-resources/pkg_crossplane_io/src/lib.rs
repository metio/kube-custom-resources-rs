/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# pkg_crossplane_io

Custom resources in this crate belong to the `pkg.crossplane.io` group. The following versions and custom resources are available:

## pkg.crossplane.io/v1
- `ConfigurationRevision`
- `Configuration`
- `ProviderRevision`
- `Provider`
## pkg.crossplane.io/v1alpha1
- `ControllerConfig`
## pkg.crossplane.io/v1beta1
- `Lock`
*/
pub mod v1;
pub mod v1alpha1;
pub mod v1beta1;
