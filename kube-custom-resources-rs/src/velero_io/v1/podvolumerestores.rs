// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/vmware-tanzu/velero/velero.io/v1/podvolumerestores.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::api::core::v1::ObjectReference;
}
use self::prelude::*;

/// PodVolumeRestoreSpec is the specification for a PodVolumeRestore.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "velero.io", version = "v1", kind = "PodVolumeRestore", plural = "podvolumerestores")]
#[kube(namespaced)]
#[kube(status = "PodVolumeRestoreStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct PodVolumeRestoreSpec {
    /// BackupStorageLocation is the name of the backup storage location
    /// where the backup repository is stored.
    #[serde(rename = "backupStorageLocation")]
    pub backup_storage_location: String,
    /// Pod is a reference to the pod containing the volume to be restored.
    pub pod: ObjectReference,
    /// RepoIdentifier is the backup repository identifier.
    #[serde(rename = "repoIdentifier")]
    pub repo_identifier: String,
    /// SnapshotID is the ID of the volume snapshot to be restored.
    #[serde(rename = "snapshotID")]
    pub snapshot_id: String,
    /// SourceNamespace is the original namespace for namaspace mapping.
    #[serde(rename = "sourceNamespace")]
    pub source_namespace: String,
    /// UploaderSettings are a map of key-value pairs that should be applied to the
    /// uploader configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "uploaderSettings")]
    pub uploader_settings: Option<BTreeMap<String, String>>,
    /// UploaderType is the type of the uploader to handle the data transfer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "uploaderType")]
    pub uploader_type: Option<PodVolumeRestoreUploaderType>,
    /// Volume is the name of the volume within the Pod to be restored.
    pub volume: String,
}

/// Pod is a reference to the pod containing the volume to be restored.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PodVolumeRestorePod {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string
    /// should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2].
    /// For example, if the object reference is to a container within a pod, this would take on a value like:
    /// "spec.containers{name}" (where "name" refers to the name of the container that triggered
    /// the event) or if no container name is specified "spec.containers[2]" (container with
    /// index 2 in this pod). This syntax is chosen only to have some well-defined way of
    /// referencing a part of an object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// PodVolumeRestoreSpec is the specification for a PodVolumeRestore.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PodVolumeRestoreUploaderType {
    #[serde(rename = "kopia")]
    Kopia,
    #[serde(rename = "restic")]
    Restic,
    #[serde(rename = "")]
    KopiumEmpty,
}

/// PodVolumeRestoreStatus is the current status of a PodVolumeRestore.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PodVolumeRestoreStatus {
    /// CompletionTimestamp records the time a restore was completed.
    /// Completion time is recorded even on failed restores.
    /// The server's time is used for CompletionTimestamps
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "completionTimestamp")]
    pub completion_timestamp: Option<String>,
    /// Message is a message about the pod volume restore's status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Phase is the current state of the PodVolumeRestore.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<PodVolumeRestoreStatusPhase>,
    /// Progress holds the total number of bytes of the snapshot and the current
    /// number of restored bytes. This can be used to display progress information
    /// about the restore operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<PodVolumeRestoreStatusProgress>,
    /// StartTimestamp records the time a restore was started.
    /// The server's time is used for StartTimestamps
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTimestamp")]
    pub start_timestamp: Option<String>,
}

/// PodVolumeRestoreStatus is the current status of a PodVolumeRestore.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PodVolumeRestoreStatusPhase {
    New,
    InProgress,
    Completed,
    Failed,
}

/// Progress holds the total number of bytes of the snapshot and the current
/// number of restored bytes. This can be used to display progress information
/// about the restore operation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PodVolumeRestoreStatusProgress {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bytesDone")]
    pub bytes_done: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "totalBytes")]
    pub total_bytes: Option<i64>,
}

