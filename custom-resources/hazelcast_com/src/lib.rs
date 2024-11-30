/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# hazelcast_com

Custom resources in this crate belong to the `hazelcast.com` group. The following versions and custom resources are available:

## hazelcast.com/v1alpha1
- `CronHotBackup`
- `Hazelcast`
- `HotBackup`
- `ManagementCenter`
- `Map`
- `WanReplication`
*/
pub mod v1alpha1;
