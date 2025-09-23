/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `planetscale.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## planetscale.com/v2
- `EtcdLockserver`
- `VitessBackup`
- `VitessBackupSchedule`
- `VitessBackupStorage`
- `VitessCell`
- `VitessCluster`
- `VitessKeyspace`
- `VitessShard`
*/
#[cfg(feature = "v2")]
pub mod v2;
