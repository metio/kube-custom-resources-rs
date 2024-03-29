// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/fluid-cloudnative/fluid/data.fluid.io/v1alpha1/databackups.yaml --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;

/// DataBackupSpec defines the desired state of DataBackup
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "data.fluid.io", version = "v1alpha1", kind = "DataBackup", plural = "databackups")]
#[kube(namespaced)]
#[kube(status = "DataBackupStatus")]
#[kube(schema = "disabled")]
pub struct DataBackupSpec {
    /// BackupPath defines the target path to save data of the DataBackup
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupPath")]
    pub backup_path: Option<String>,
    /// Dataset defines the target dataset of the DataBackup
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dataset: Option<String>,
    /// Specifies that the preceding operation in a workflow
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runAfter")]
    pub run_after: Option<DataBackupRunAfter>,
    /// Manage the user to run Alluxio DataBackup
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runAs")]
    pub run_as: Option<DataBackupRunAs>,
    /// TTLSecondsAfterFinished is the time second to clean up data operations after finished or failed
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ttlSecondsAfterFinished")]
    pub ttl_seconds_after_finished: Option<i32>,
}

/// Specifies that the preceding operation in a workflow
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DataBackupRunAfter {
    /// API version of the referent operation
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Kind specifies the type of the referent operation
    pub kind: DataBackupRunAfterKind,
    /// Name specifies the name of the referent operation
    pub name: String,
    /// Namespace specifies the namespace of the referent operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Specifies that the preceding operation in a workflow
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DataBackupRunAfterKind {
    DataLoad,
    DataBackup,
    DataMigrate,
    DataProcess,
}

/// Manage the user to run Alluxio DataBackup
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DataBackupRunAs {
    /// The gid to run the alluxio runtime
    pub gid: i64,
    /// The group name to run the alluxio runtime
    pub group: String,
    /// The uid to run the alluxio runtime
    pub uid: i64,
    /// The user name to run the alluxio runtime
    pub user: String,
}

/// OperationStatus defines the observed state of operation
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DataBackupStatus {
    /// Conditions consists of transition information on operation's Phase
    pub conditions: Vec<Condition>,
    /// Duration tell user how much time was spent to operation
    pub duration: String,
    /// Infos operation customized name-value
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub infos: Option<BTreeMap<String, String>>,
    /// LastScheduleTime is the last time the cron operation was scheduled
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastScheduleTime")]
    pub last_schedule_time: Option<String>,
    /// LastSuccessfulTime is the last time the cron operation successfully completed
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastSuccessfulTime")]
    pub last_successful_time: Option<String>,
    /// Phase describes current phase of operation
    pub phase: String,
    /// WaitingStatus stores information about waiting operation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "waitingFor")]
    pub waiting_for: Option<DataBackupStatusWaitingFor>,
}

/// WaitingStatus stores information about waiting operation.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DataBackupStatusWaitingFor {
    /// OperationComplete indicates if the preceding operation is complete
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "operationComplete")]
    pub operation_complete: Option<bool>,
}

