// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kyverno/kyverno/kyverno.io/v1alpha2/backgroundscanreports.yaml --derive=Default --derive=PartialEq --smart-derive-elision
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
#[kube(group = "kyverno.io", version = "v1alpha2", kind = "BackgroundScanReport", plural = "backgroundscanreports")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct BackgroundScanReportSpec {
    /// PolicyReportResult provides result details
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<BackgroundScanReportResults>>,
    /// PolicyReportSummary provides a summary of results
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<BackgroundScanReportSummary>,
}

/// PolicyReportResult provides the result for an individual policy
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackgroundScanReportResults {
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
    /// SubjectSelector is an optional label selector for checked Kubernetes resources.
    /// For example, a policy result may apply to all pods that match a label.
    /// Either a Subject or a SubjectSelector can be specified.
    /// If neither are provided, the result is assumed to be for the policy report scope.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceSelector")]
    pub resource_selector: Option<BackgroundScanReportResultsResourceSelector>,
    /// Subjects is an optional reference to the checked Kubernetes resources
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<ObjectReference>>,
    /// Result indicates the outcome of the policy rule execution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<BackgroundScanReportResultsResult>,
    /// Rule is the name or identifier of the rule within the policy
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>,
    /// Scored indicates if this result is scored
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scored: Option<bool>,
    /// Severity indicates policy check result criticality
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<BackgroundScanReportResultsSeverity>,
    /// Source is an identifier for the policy engine that manages this report
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Timestamp indicates the time the result was found
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<BackgroundScanReportResultsTimestamp>,
}

/// SubjectSelector is an optional label selector for checked Kubernetes resources.
/// For example, a policy result may apply to all pods that match a label.
/// Either a Subject or a SubjectSelector can be specified.
/// If neither are provided, the result is assumed to be for the policy report scope.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackgroundScanReportResultsResourceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<BackgroundScanReportResultsResourceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackgroundScanReportResultsResourceSelectorMatchExpressions {
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

/// PolicyReportResult provides the result for an individual policy
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BackgroundScanReportResultsResult {
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

/// PolicyReportResult provides the result for an individual policy
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BackgroundScanReportResultsSeverity {
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
pub struct BackgroundScanReportResultsTimestamp {
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
pub struct BackgroundScanReportSummary {
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

