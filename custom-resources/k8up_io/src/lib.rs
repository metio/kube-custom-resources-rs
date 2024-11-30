/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# k8up_io

Custom resources in this crate belong to the `k8up.io` group. The following versions and custom resources are available:

## k8up.io/v1
- `Archive`
- `Backup`
- `Check`
- `PreBackupPod`
- `Prune`
- `Restore`
- `Schedule`
- `Snapshot`
*/
pub mod v1;
