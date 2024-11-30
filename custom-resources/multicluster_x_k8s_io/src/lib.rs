/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# multicluster_x_k8s_io

Custom resources in this crate belong to the `multicluster.x-k8s.io` group. The following versions and custom resources are available:

## multicluster.x-k8s.io/v1alpha1
- `ServiceExport`
- `ServiceImport`
- `AppliedWork`
*/
pub mod v1alpha1;
