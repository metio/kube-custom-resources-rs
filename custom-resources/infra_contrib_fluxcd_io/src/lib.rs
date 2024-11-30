/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# infra_contrib_fluxcd_io

Custom resources in this crate belong to the `infra.contrib.fluxcd.io` group. The following versions and custom resources are available:

## infra.contrib.fluxcd.io/v1alpha1
- `Terraform`
## infra.contrib.fluxcd.io/v1alpha2
- `Terraform`
*/
pub mod v1alpha1;
pub mod v1alpha2;
