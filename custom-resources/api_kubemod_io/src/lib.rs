/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# api_kubemod_io

Custom resources in this crate belong to the `api.kubemod.io` group. The following versions and custom resources are available:

## api.kubemod.io/v1beta1
- `ModRule`
*/
pub mod v1beta1;
