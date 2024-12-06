/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `capsule.clastix.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

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
