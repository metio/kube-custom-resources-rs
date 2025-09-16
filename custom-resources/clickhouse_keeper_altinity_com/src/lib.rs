/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `clickhouse-keeper.altinity.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## clickhouse-keeper.altinity.com/v1
- `ClickHouseKeeperInstallation`
*/
#[cfg(feature = "v1")]
pub mod v1;
