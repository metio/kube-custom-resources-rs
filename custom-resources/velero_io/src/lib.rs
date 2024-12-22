/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `velero.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## velero.io/v1
- `BackupRepository`
- `Backup`
- `BackupStorageLocation`
- `DeleteBackupRequest`
- `DownloadRequest`
- `PodVolumeBackup`
- `PodVolumeRestore`
- `Schedule`
- `ServerStatusRequest`
- `VolumeSnapshotLocation`
## velero.io/v2alpha1
- `DataDownload`
- `DataUpload`
*/
#[cfg(feature = "v1")]
pub mod v1;
#[cfg(feature = "v2alpha1")]
pub mod v2alpha1;
