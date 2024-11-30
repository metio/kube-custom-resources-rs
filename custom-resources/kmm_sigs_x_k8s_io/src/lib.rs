/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# kmm_sigs_x_k8s_io

Custom resources in this crate belong to the `kmm.sigs.x-k8s.io` group. The following versions and custom resources are available:

## kmm.sigs.x-k8s.io/v1beta1
- `Module`
- `NodeModulesConfig`
- `PreflightValidation`
## kmm.sigs.x-k8s.io/v1beta2
- `PreflightValidation`
*/
pub mod v1beta1;
pub mod v1beta2;
