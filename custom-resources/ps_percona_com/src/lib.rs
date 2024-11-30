/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# ps_percona_com

Custom resources in this crate belong to the `ps.percona.com` group. The following versions and custom resources are available:

## ps.percona.com/v1alpha1
- `PerconaServerMySQLBackup`
- `PerconaServerMySQLRestore`
- `PerconaServerMySQL`
*/
pub mod v1alpha1;
