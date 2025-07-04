// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kyverno/kyverno/reports.kyverno.io/v1/clusterephemeralreports.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::api::core::v1::ObjectReference;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "reports.kyverno.io", version = "v1", kind = "ClusterEphemeralReport", plural = "clusterephemeralreports")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ClusterEphemeralReportSpec {
    /// Owner is a reference to the report owner (e.g. a Deployment, Namespace, or Node)
    pub owner: ClusterEphemeralReportOwner,
    /// PolicyReportResult provides result details
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ClusterEphemeralReportResults>>,
    /// PolicyReportSummary provides a summary of results
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<ClusterEphemeralReportSummary>,
}

/// Owner is a reference to the report owner (e.g. a Deployment, Namespace, or Node)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterEphemeralReportOwner {
    /// API version of the referent.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// If true, AND if the owner has the "foregroundDeletion" finalizer, then
    /// the owner cannot be deleted from the key-value store until this
    /// reference is removed.
    /// See https://kubernetes.io/docs/concepts/architecture/garbage-collection/#foreground-deletion
    /// for how the garbage collector interacts with this field and enforces the foreground deletion.
    /// Defaults to false.
    /// To set this field, a user needs "delete" permission of the owner,
    /// otherwise 422 (Unprocessable Entity) will be returned.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "blockOwnerDeletion")]
    pub block_owner_deletion: Option<bool>,
    /// If true, this reference points to the managing controller.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controller: Option<bool>,
    /// Kind of the referent.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub kind: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names#names
    pub name: String,
    /// UID of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names#uids
    pub uid: String,
}

/// ReportResult provides the result for an individual policy
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterEphemeralReportResults {
    /// Category indicates policy category
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Description is a short user friendly message for the policy rule
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Policy is the name or identifier of the policy
    pub policy: String,
    /// Properties provides additional information for the policy rule
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<String, String>>,
    /// ResourceSelector is an optional label selector for checked Kubernetes resources.
    /// For example, a policy result may apply to all pods that match a label.
    /// Either a Subject or a ResourceSelector can be specified. If neither are provided, the
    /// result is assumed to be for the policy report scope.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceSelector")]
    pub resource_selector: Option<ClusterEphemeralReportResultsResourceSelector>,
    /// Subjects is an optional reference to the checked Kubernetes resources
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<ObjectReference>>,
    /// Result indicates the outcome of the policy rule execution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<ClusterEphemeralReportResultsResult>,
    /// Rule is the name or identifier of the rule within the policy
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>,
    /// Scored indicates if this result is scored
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scored: Option<bool>,
    /// Severity indicates policy check result criticality
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<ClusterEphemeralReportResultsSeverity>,
    /// Source is an identifier for the policy engine that manages this report
    /// If the Source is specified at this level, it will override the Source
    /// field set at the Report level
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Timestamp indicates the time the result was found
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<ClusterEphemeralReportResultsTimestamp>,
}

/// ResourceSelector is an optional label selector for checked Kubernetes resources.
/// For example, a policy result may apply to all pods that match a label.
/// Either a Subject or a ResourceSelector can be specified. If neither are provided, the
/// result is assumed to be for the policy report scope.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterEphemeralReportResultsResourceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ClusterEphemeralReportResultsResourceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterEphemeralReportResultsResourceSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// ReportResult provides the result for an individual policy
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterEphemeralReportResultsResult {
    #[serde(rename = "pass")]
    Pass,
    #[serde(rename = "fail")]
    Fail,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "skip")]
    Skip,
}

/// ReportResult provides the result for an individual policy
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterEphemeralReportResultsSeverity {
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "info")]
    Info,
}

/// Timestamp indicates the time the result was found
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterEphemeralReportResultsTimestamp {
    /// Non-negative fractions of a second at nanosecond resolution. Negative
    /// second values with fractions must still have non-negative nanos values
    /// that count forward in time. Must be from 0 to 999,999,999
    /// inclusive. This field may be limited in precision depending on context.
    pub nanos: i32,
    /// Represents seconds of UTC time since Unix epoch
    /// 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
    /// 9999-12-31T23:59:59Z inclusive.
    pub seconds: i64,
}

/// PolicyReportSummary provides a summary of results
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterEphemeralReportSummary {
    /// Error provides the count of policies that could not be evaluated
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<i64>,
    /// Fail provides the count of policies whose requirements were not met
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fail: Option<i64>,
    /// Pass provides the count of policies whose requirements were met
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pass: Option<i64>,
    /// Skip indicates the count of policies that were not selected for evaluation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Warn provides the count of non-scored policies whose requirements were not met
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warn: Option<i64>,
}

