/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# mariadb_mmontes_io

Custom resources in this crate belong to the `mariadb.mmontes.io` group. The following versions and custom resources are available:

## mariadb.mmontes.io/v1alpha1
- `Backup`
- `Connection`
- `Database`
- `Grant`
- `MariaDB`
- `Restore`
- `SqlJob`
- `User`
*/
pub mod v1alpha1;
