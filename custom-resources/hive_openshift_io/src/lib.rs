/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `hive.openshift.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

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
- `SelectorSyncSet`
- `SyncIdentityProvider`
- `SyncSet`
*/
#[cfg(feature = "v1")]
pub mod v1;
