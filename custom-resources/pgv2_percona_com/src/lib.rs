/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# pgv2_percona_com

Custom resources in this crate belong to the `pgv2.percona.com` group. The following versions and custom resources are available:

## pgv2.percona.com/v2
- `PerconaPGBackup`
- `PerconaPGCluster`
- `PerconaPGRestore`
- `PerconaPGUpgrade`
*/
pub mod v2;
