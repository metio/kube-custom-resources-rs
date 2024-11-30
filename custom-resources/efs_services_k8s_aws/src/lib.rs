/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# efs_services_k8s_aws

Custom resources in this crate belong to the `efs.services.k8s.aws` group. The following versions and custom resources are available:

## efs.services.k8s.aws/v1alpha1
- `AccessPoint`
- `FileSystem`
- `MountTarget`
*/
pub mod v1alpha1;
