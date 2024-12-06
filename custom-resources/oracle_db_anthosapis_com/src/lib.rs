/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `oracle.db.anthosapis.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## oracle.db.anthosapis.com/v1alpha1
- `Backup`
- `BackupSchedule`
- `Config`
- `CronAnything`
- `Database`
- `Export`
- `Import`
- `PITR`
- `Release`
*/
pub mod v1alpha1;
