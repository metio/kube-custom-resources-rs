/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# velero_io

Custom resources in this crate belong to the `velero.io` group. The following versions and custom resources are available:

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
pub mod v1;
pub mod v2alpha1;
