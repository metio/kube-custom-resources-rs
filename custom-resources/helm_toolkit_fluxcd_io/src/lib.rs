/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# helm_toolkit_fluxcd_io

Custom resources in this crate belong to the `helm.toolkit.fluxcd.io` group. The following versions and custom resources are available:

## helm.toolkit.fluxcd.io/v2
- `HelmRelease`
## helm.toolkit.fluxcd.io/v2beta1
- `HelmRelease`
## helm.toolkit.fluxcd.io/v2beta2
- `HelmRelease`
*/
pub mod v2;
pub mod v2beta1;
pub mod v2beta2;
