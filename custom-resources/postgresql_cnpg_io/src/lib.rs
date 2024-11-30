/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# postgresql_cnpg_io

Custom resources in this crate belong to the `postgresql.cnpg.io` group. The following versions and custom resources are available:

## postgresql.cnpg.io/v1
- `Backup`
- `Pooler`
- `ScheduledBackup`
*/
pub mod v1;
