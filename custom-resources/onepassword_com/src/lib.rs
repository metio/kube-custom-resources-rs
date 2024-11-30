/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# onepassword_com

Custom resources in this crate belong to the `onepassword.com` group. The following versions and custom resources are available:

## onepassword.com/v1
- `OnePasswordItem`
*/
pub mod v1;
