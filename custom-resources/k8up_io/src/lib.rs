/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `k8up.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

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
