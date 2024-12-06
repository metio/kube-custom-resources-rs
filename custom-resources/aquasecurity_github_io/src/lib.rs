/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `aquasecurity.github.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## aquasecurity.github.io/v1alpha1
- `AquaStarboard`
- `ClusterConfigAuditReport`
- `ConfigAuditReport`
*/
pub mod v1alpha1;
