/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# psmdb_percona_com

Custom resources in this crate belong to the `psmdb.percona.com` group. The following versions and custom resources are available:

## psmdb.percona.com/v1
- `PerconaServerMongoDBBackup`
- `PerconaServerMongoDBRestore`
*/
pub mod v1;
