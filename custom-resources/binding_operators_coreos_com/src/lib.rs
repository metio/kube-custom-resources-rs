/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# binding_operators_coreos_com

Custom resources in this crate belong to the `binding.operators.coreos.com` group. The following versions and custom resources are available:

## binding.operators.coreos.com/v1alpha1
- `BindableKinds`
- `ServiceBinding`
*/
pub mod v1alpha1;
