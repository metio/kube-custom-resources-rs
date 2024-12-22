/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `fossul.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## fossul.io/v1
- `BackupConfig`
- `Backup`
- `BackupSchedule`
- `Fossul`
- `Restore`
*/
#[cfg(feature = "v1")]
pub mod v1;
