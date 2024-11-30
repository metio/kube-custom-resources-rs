/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# secrets_stackable_tech

Custom resources in this crate belong to the `secrets.stackable.tech` group. The following versions and custom resources are available:

## secrets.stackable.tech/v1alpha1
- `SecretClass`
*/
pub mod v1alpha1;
