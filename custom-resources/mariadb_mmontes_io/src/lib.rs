/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `mariadb.mmontes.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

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
