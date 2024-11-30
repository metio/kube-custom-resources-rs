/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# k8s_mariadb_com

Custom resources in this crate belong to the `k8s.mariadb.com` group. The following versions and custom resources are available:

## k8s.mariadb.com/v1alpha1
- `Backup`
- `Connection`
- `Database`
- `Grant`
- `MariaDB`
- `MaxScale`
- `Restore`
- `SqlJob`
- `User`
*/
pub mod v1alpha1;
