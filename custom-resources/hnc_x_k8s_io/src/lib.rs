/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `hnc.x-k8s.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## hnc.x-k8s.io/v1alpha2
- `HierarchicalResourceQuota`
- `HierarchyConfiguration`
- `HNCConfiguration`
- `SubnamespaceAnchor`
*/
pub mod v1alpha2;
