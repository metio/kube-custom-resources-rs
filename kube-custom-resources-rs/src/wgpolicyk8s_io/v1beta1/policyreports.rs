// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/wg-policy-prototypes/wgpolicyk8s.io/v1beta1/policyreports.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::api::core::v1::ObjectReference;
}
use self::prelude::*;

/// Configuration is an optional field which can be used to specify a contract between PolicyReport generators and consumers
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyReportConfiguration {
    pub limits: PolicyReportConfigurationLimits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyReportConfigurationLimits {
    /// MaxResults is the maximum number of results contained in the report
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxResults")]
    pub max_results: Option<i64>,
    /// StatusFilter indicates that the PolicyReport contains only those reports with statuses specified in this list
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusFilter")]
    pub status_filter: Option<Vec<String>>,
}

/// PolicyReportResult provides the result for an individual policy
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyReportResults {
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
    /// ResourceSelector is an optional label selector for checked Kubernetes resources. For example, a policy result may apply to all pods that match a label. Either a Subject or a ResourceSelector can be specified. If neither are provided, the result is assumed to be for the policy report scope.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceSelector")]
    pub resource_selector: Option<PolicyReportResultsResourceSelector>,
    /// Subjects is an optional reference to the checked Kubernetes resources
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<ObjectReference>>,
    /// Result indicates the outcome of the policy rule execution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<PolicyReportResultsResult>,
    /// Rule is the name or identifier of the rule within the policy
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>,
    /// Scored indicates if this result is scored
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scored: Option<bool>,
    /// Severity indicates policy check result criticality
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<PolicyReportResultsSeverity>,
    /// Source is an identifier for the policy engine that manages this report If the Source is specified at this level, it will override the Source field set at the PolicyReport level
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Timestamp indicates the time the result was found
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<PolicyReportResultsTimestamp>,
}

/// ResourceSelector is an optional label selector for checked Kubernetes resources. For example, a policy result may apply to all pods that match a label. Either a Subject or a ResourceSelector can be specified. If neither are provided, the result is assumed to be for the policy report scope.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyReportResultsResourceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<PolicyReportResultsResourceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyReportResultsResourceSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// PolicyReportResult provides the result for an individual policy
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PolicyReportResultsResult {
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
pub enum PolicyReportResultsSeverity {
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
pub struct PolicyReportResultsTimestamp {
    /// Non-negative fractions of a second at nanosecond resolution. Negative second values with fractions must still have non-negative nanos values that count forward in time. Must be from 0 to 999,999,999 inclusive. This field may be limited in precision depending on context.
    pub nanos: i32,
    /// Represents seconds of UTC time since Unix epoch 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to 9999-12-31T23:59:59Z inclusive.
    pub seconds: i64,
}

/// Scope is an optional reference to the report scope (e.g. a Deployment, Namespace, or Node)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyReportScope {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2]. For example, if the object reference is to a container within a pod, this would take on a value like: "spec.containers{name}" (where "name" refers to the name of the container that triggered the event) or if no container name is specified "spec.containers[2]" (container with index 2 in this pod). This syntax is chosen only to have some well-defined way of referencing a part of an object. TODO: this design is not final and this field is subject to change in the future.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// ScopeSelector is an optional selector for multiple scopes (e.g. Pods). Either one of, or none of, but not both of, Scope or ScopeSelector should be specified.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyReportScopeSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<PolicyReportScopeSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyReportScopeSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// PolicyReportSummary provides a summary of results
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyReportSummary {
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

