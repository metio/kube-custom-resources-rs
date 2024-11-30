/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# sts_min_io

Custom resources in this crate belong to the `sts.min.io` group. The following versions and custom resources are available:

## sts.min.io/v1alpha1
- `PolicyBinding`
## sts.min.io/v1beta1
- `PolicyBinding`
*/
pub mod v1alpha1;
pub mod v1beta1;
