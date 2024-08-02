// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/longhorn/longhorn/longhorn.io/v1beta2/recurringjobs.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// RecurringJobSpec defines the desired state of the Longhorn recurring job
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "longhorn.io", version = "v1beta2", kind = "RecurringJob", plural = "recurringjobs")]
#[kube(namespaced)]
#[kube(status = "RecurringJobStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct RecurringJobSpec {
    /// The concurrency of taking the snapshot/backup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<i64>,
    /// The cron setting.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cron: Option<String>,
    /// The recurring job group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    /// The label of the snapshot/backup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// The recurring job name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The parameters of the snapshot/backup.
    /// Support parameters: "full-backup-interval".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<BTreeMap<String, String>>,
    /// The retain count of the snapshot/backup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retain: Option<i64>,
    /// The recurring job task.
    /// Can be "snapshot", "snapshot-force-create", "snapshot-cleanup", "snapshot-delete", "backup", "backup-force-create" or "filesystem-trim"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<RecurringJobTask>,
}

/// RecurringJobSpec defines the desired state of the Longhorn recurring job
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RecurringJobTask {
    #[serde(rename = "snapshot")]
    Snapshot,
    #[serde(rename = "snapshot-force-create")]
    SnapshotForceCreate,
    #[serde(rename = "snapshot-cleanup")]
    SnapshotCleanup,
    #[serde(rename = "snapshot-delete")]
    SnapshotDelete,
    #[serde(rename = "backup")]
    Backup,
    #[serde(rename = "backup-force-create")]
    BackupForceCreate,
    #[serde(rename = "filesystem-trim")]
    FilesystemTrim,
}

/// RecurringJobStatus defines the observed state of the Longhorn recurring job
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RecurringJobStatus {
    /// The number of jobs that have been triggered.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "executionCount")]
    pub execution_count: Option<i64>,
    /// The owner ID which is responsible to reconcile this recurring job CR.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    pub owner_id: Option<String>,
}

