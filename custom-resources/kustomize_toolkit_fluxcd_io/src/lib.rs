/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# kustomize_toolkit_fluxcd_io

Custom resources in this crate belong to the `kustomize.toolkit.fluxcd.io` group. The following versions and custom resources are available:

## kustomize.toolkit.fluxcd.io/v1
- `Kustomization`
## kustomize.toolkit.fluxcd.io/v1beta1
- `Kustomization`
## kustomize.toolkit.fluxcd.io/v1beta2
- `Kustomization`
*/
pub mod v1;
pub mod v1beta1;
pub mod v1beta2;
