// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/fluid-cloudnative/fluid/data.fluid.io/v1alpha1/databackups.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// DataBackupSpec defines the desired state of DataBackup
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "data.fluid.io", version = "v1alpha1", kind = "DataBackup", plural = "databackups")]
#[kube(namespaced)]
#[kube(status = "DataBackupStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
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
    /// AffinityStrategy specifies the pod affinity strategy with the referent operation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "affinityStrategy")]
    pub affinity_strategy: Option<DataBackupRunAfterAffinityStrategy>,
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

/// AffinityStrategy specifies the pod affinity strategy with the referent operation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataBackupRunAfterAffinityStrategy {
    /// Specifies the dependent preceding operation in a workflow. If not set, use the operation referred to by RunAfter.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dependOn")]
    pub depend_on: Option<DataBackupRunAfterAffinityStrategyDependOn>,
    /// Policy one of: "", "Require", "Prefer"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefers: Option<Vec<DataBackupRunAfterAffinityStrategyPrefers>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requires: Option<Vec<DataBackupRunAfterAffinityStrategyRequires>>,
}

/// Specifies the dependent preceding operation in a workflow. If not set, use the operation referred to by RunAfter.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DataBackupRunAfterAffinityStrategyDependOn {
    /// API version of the referent operation
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Kind specifies the type of the referent operation
    pub kind: DataBackupRunAfterAffinityStrategyDependOnKind,
    /// Name specifies the name of the referent operation
    pub name: String,
    /// Namespace specifies the namespace of the referent operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Specifies the dependent preceding operation in a workflow. If not set, use the operation referred to by RunAfter.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DataBackupRunAfterAffinityStrategyDependOnKind {
    DataLoad,
    DataBackup,
    DataMigrate,
    DataProcess,
}

/// Prefer defines the label key and weight for generating a PreferredSchedulingTerm.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataBackupRunAfterAffinityStrategyPrefers {
    pub name: String,
    pub weight: i32,
}

/// Require defines the label key for generating a NodeSelectorTerm.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataBackupRunAfterAffinityStrategyRequires {
    pub name: String,
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
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
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
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
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
    /// NodeAffinity records the node affinity for operation pods
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeAffinity")]
    pub node_affinity: Option<DataBackupStatusNodeAffinity>,
    /// Phase describes current phase of operation
    pub phase: String,
    /// WaitingStatus stores information about waiting operation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "waitingFor")]
    pub waiting_for: Option<DataBackupStatusWaitingFor>,
}

/// NodeAffinity records the node affinity for operation pods
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataBackupStatusNodeAffinity {
    /// The scheduler will prefer to schedule pods to nodes that satisfy
    /// the affinity expressions specified by this field, but it may choose
    /// a node that violates one or more of the expressions. The node that is
    /// most preferred is the one with the greatest sum of weights, i.e.
    /// for each node that meets all of the scheduling requirements (resource
    /// request, requiredDuringScheduling affinity expressions, etc.),
    /// compute a sum by iterating through the elements of this field and adding
    /// "weight" to the sum if the node matches the corresponding matchExpressions; the
    /// node(s) with the highest sum are the most preferred.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredDuringSchedulingIgnoredDuringExecution")]
    pub preferred_during_scheduling_ignored_during_execution: Option<Vec<DataBackupStatusNodeAffinityPreferredDuringSchedulingIgnoredDuringExecution>>,
    /// If the affinity requirements specified by this field are not met at
    /// scheduling time, the pod will not be scheduled onto the node.
    /// If the affinity requirements specified by this field cease to be met
    /// at some point during pod execution (e.g. due to an update), the system
    /// may or may not try to eventually evict the pod from its node.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requiredDuringSchedulingIgnoredDuringExecution")]
    pub required_during_scheduling_ignored_during_execution: Option<DataBackupStatusNodeAffinityRequiredDuringSchedulingIgnoredDuringExecution>,
}

/// An empty preferred scheduling term matches all objects with implicit weight 0
/// (i.e. it's a no-op). A null preferred scheduling term matches no objects (i.e. is also a no-op).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataBackupStatusNodeAffinityPreferredDuringSchedulingIgnoredDuringExecution {
    /// A node selector term, associated with the corresponding weight.
    pub preference: DataBackupStatusNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreference,
    /// Weight associated with matching the corresponding nodeSelectorTerm, in the range 1-100.
    pub weight: i32,
}

/// A node selector term, associated with the corresponding weight.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataBackupStatusNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreference {
    /// A list of node selector requirements by node's labels.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<DataBackupStatusNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreferenceMatchExpressions>>,
    /// A list of node selector requirements by node's fields.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchFields")]
    pub match_fields: Option<Vec<DataBackupStatusNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreferenceMatchFields>>,
}

/// A node selector requirement is a selector that contains values, a key, and an operator
/// that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataBackupStatusNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreferenceMatchExpressions {
    /// The label key that the selector applies to.
    pub key: String,
    /// Represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    pub operator: String,
    /// An array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. If the operator is Gt or Lt, the values
    /// array must have a single element, which will be interpreted as an integer.
    /// This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// A node selector requirement is a selector that contains values, a key, and an operator
/// that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataBackupStatusNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreferenceMatchFields {
    /// The label key that the selector applies to.
    pub key: String,
    /// Represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    pub operator: String,
    /// An array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. If the operator is Gt or Lt, the values
    /// array must have a single element, which will be interpreted as an integer.
    /// This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// If the affinity requirements specified by this field are not met at
/// scheduling time, the pod will not be scheduled onto the node.
/// If the affinity requirements specified by this field cease to be met
/// at some point during pod execution (e.g. due to an update), the system
/// may or may not try to eventually evict the pod from its node.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataBackupStatusNodeAffinityRequiredDuringSchedulingIgnoredDuringExecution {
    /// Required. A list of node selector terms. The terms are ORed.
    #[serde(rename = "nodeSelectorTerms")]
    pub node_selector_terms: Vec<DataBackupStatusNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTerms>,
}

/// A null or empty node selector term matches no objects. The requirements of
/// them are ANDed.
/// The TopologySelectorTerm type implements a subset of the NodeSelectorTerm.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataBackupStatusNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTerms {
    /// A list of node selector requirements by node's labels.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<DataBackupStatusNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTermsMatchExpressions>>,
    /// A list of node selector requirements by node's fields.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchFields")]
    pub match_fields: Option<Vec<DataBackupStatusNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTermsMatchFields>>,
}

/// A node selector requirement is a selector that contains values, a key, and an operator
/// that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataBackupStatusNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTermsMatchExpressions {
    /// The label key that the selector applies to.
    pub key: String,
    /// Represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    pub operator: String,
    /// An array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. If the operator is Gt or Lt, the values
    /// array must have a single element, which will be interpreted as an integer.
    /// This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// A node selector requirement is a selector that contains values, a key, and an operator
/// that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataBackupStatusNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTermsMatchFields {
    /// The label key that the selector applies to.
    pub key: String,
    /// Represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    pub operator: String,
    /// An array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. If the operator is Gt or Lt, the values
    /// array must have a single element, which will be interpreted as an integer.
    /// This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// WaitingStatus stores information about waiting operation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataBackupStatusWaitingFor {
    /// OperationComplete indicates if the preceding operation is complete
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "operationComplete")]
    pub operation_complete: Option<bool>,
}

