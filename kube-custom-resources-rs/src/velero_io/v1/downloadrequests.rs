// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/vmware-tanzu/velero/velero.io/v1/downloadrequests.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// DownloadRequestSpec is the specification for a download request.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "velero.io", version = "v1", kind = "DownloadRequest", plural = "downloadrequests")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct DownloadRequestSpec {
    /// Target is what to download (e.g. logs for a backup).
    pub target: DownloadRequestTarget,
}

/// Target is what to download (e.g. logs for a backup).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DownloadRequestTarget {
    /// Kind is the type of file to download.
    pub kind: DownloadRequestTargetKind,
    /// Name is the name of the Kubernetes resource with which the file is associated.
    pub name: String,
}

/// Target is what to download (e.g. logs for a backup).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DownloadRequestTargetKind {
    BackupLog,
    BackupContents,
    BackupVolumeSnapshots,
    BackupItemOperations,
    BackupResourceList,
    BackupResults,
    RestoreLog,
    RestoreResults,
    RestoreResourceList,
    RestoreItemOperations,
    #[serde(rename = "CSIBackupVolumeSnapshots")]
    CsiBackupVolumeSnapshots,
    #[serde(rename = "CSIBackupVolumeSnapshotContents")]
    CsiBackupVolumeSnapshotContents,
    BackupVolumeInfos,
    RestoreVolumeInfo,
}

/// DownloadRequestStatus is the current status of a DownloadRequest.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DownloadRequestStatus {
    /// DownloadURL contains the pre-signed URL for the target file.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "downloadURL")]
    pub download_url: Option<String>,
    /// Expiration is when this DownloadRequest expires and can be deleted by the system.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    /// Phase is the current state of the DownloadRequest.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<DownloadRequestStatusPhase>,
}

/// DownloadRequestStatus is the current status of a DownloadRequest.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DownloadRequestStatusPhase {
    New,
    Processed,
}

