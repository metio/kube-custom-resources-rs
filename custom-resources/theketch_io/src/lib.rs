/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# theketch_io

Custom resources in this crate belong to the `theketch.io` group. The following versions and custom resources are available:

## theketch.io/v1beta1
- `App`
- `Job`
*/
pub mod v1beta1;
