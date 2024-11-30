/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# digitalis_io

Custom resources in this crate belong to the `digitalis.io` group. The following versions and custom resources are available:

## digitalis.io/v1
- `ValsSecret`
## digitalis.io/v1beta1
- `DbSecret`
*/
pub mod v1;
pub mod v1beta1;
