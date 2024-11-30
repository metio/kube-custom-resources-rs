/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# capsule_clastix_io

Custom resources in this crate belong to the `capsule.clastix.io` group. The following versions and custom resources are available:

## capsule.clastix.io/v1alpha1
- `CapsuleConfiguration`
- `Tenant`
## capsule.clastix.io/v1beta1
- `Tenant`
## capsule.clastix.io/v1beta2
- `CapsuleConfiguration`
- `Tenant`
*/
pub mod v1alpha1;
pub mod v1beta1;
pub mod v1beta2;
