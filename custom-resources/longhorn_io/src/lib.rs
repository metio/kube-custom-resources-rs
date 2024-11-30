/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# longhorn_io

Custom resources in this crate belong to the `longhorn.io` group. The following versions and custom resources are available:

## longhorn.io/v1beta1
- `BackingImageDataSource`
- `BackingImageManager`
- `BackingImage`
- `Backup`
- `BackupTarget`
- `BackupVolume`
- `EngineImage`
- `Engine`
- `InstanceManager`
- `Node`
- `RecurringJob`
- `Replica`
- `Setting`
- `ShareManager`
- `Volume`
## longhorn.io/v1beta2
- `BackingImageDataSource`
- `BackingImageManager`
- `BackingImage`
- `BackupBackingImage`
- `Backup`
- `BackupTarget`
- `BackupVolume`
- `EngineImage`
- `Engine`
- `InstanceManager`
- `Node`
- `Orphan`
- `RecurringJob`
- `Replica`
- `Setting`
- `ShareManager`
- `Snapshot`
- `SupportBundle`
- `SystemBackup`
- `SystemRestore`
- `VolumeAttachment`
- `Volume`
*/
pub mod v1beta1;
pub mod v1beta2;
