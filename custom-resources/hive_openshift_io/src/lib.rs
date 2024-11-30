/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# hive_openshift_io

Custom resources in this crate belong to the `hive.openshift.io` group. The following versions and custom resources are available:

## hive.openshift.io/v1
- `Checkpoint`
- `ClusterClaim`
- `ClusterDeploymentCustomization`
- `ClusterDeployment`
- `ClusterDeprovision`
- `ClusterImageSet`
- `ClusterPool`
- `ClusterProvision`
- `ClusterRelocate`
- `ClusterState`
- `DNSZone`
- `HiveConfig`
- `MachinePoolNameLease`
- `MachinePool`
- `SelectorSyncIdentityProvider`
- `SyncIdentityProvider`
*/
pub mod v1;
