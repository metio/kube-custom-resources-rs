/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `dataprotection.kubeblocks.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## dataprotection.kubeblocks.io/v1alpha1
- `ActionSet`
- `BackupPolicy`
- `BackupRepo`
- `Backup`
- `BackupSchedule`
- `Restore`
*/
pub mod v1alpha1;
