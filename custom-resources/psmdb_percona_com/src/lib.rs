/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `psmdb.percona.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## psmdb.percona.com/v1
- `PerconaServerMongoDBBackup`
- `PerconaServerMongoDBRestore`
*/
pub mod v1;
