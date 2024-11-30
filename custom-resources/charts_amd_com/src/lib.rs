/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# charts_amd_com

Custom resources in this crate belong to the `charts.amd.com` group. The following versions and custom resources are available:

## charts.amd.com/v1alpha1
- `AMDGPU`
*/
pub mod v1alpha1;
