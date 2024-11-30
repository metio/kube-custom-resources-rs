/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# hnc_x_k8s_io

Custom resources in this crate belong to the `hnc.x-k8s.io` group. The following versions and custom resources are available:

## hnc.x-k8s.io/v1alpha2
- `HierarchicalResourceQuota`
- `HierarchyConfiguration`
- `HNCConfiguration`
- `SubnamespaceAnchor`
*/
pub mod v1alpha2;
