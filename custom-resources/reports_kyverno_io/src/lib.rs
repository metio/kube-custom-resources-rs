/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# reports_kyverno_io

Custom resources in this crate belong to the `reports.kyverno.io` group. The following versions and custom resources are available:

## reports.kyverno.io/v1
- `ClusterEphemeralReport`
- `EphemeralReport`
*/
pub mod v1;
