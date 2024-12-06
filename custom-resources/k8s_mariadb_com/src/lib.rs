/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `k8s.mariadb.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

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
