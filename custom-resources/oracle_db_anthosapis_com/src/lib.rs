/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# oracle_db_anthosapis_com

Custom resources in this crate belong to the `oracle.db.anthosapis.com` group. The following versions and custom resources are available:

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
