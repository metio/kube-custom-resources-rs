/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `longhorn.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

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
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
#[cfg(feature = "v1beta2")]
pub mod v1beta2;
