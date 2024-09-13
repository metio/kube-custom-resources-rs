// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/medik8s/node-healthcheck-operator/remediation.medik8s.io/v1alpha1/nodehealthchecks.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use k8s_openapi::api::core::v1::ObjectReference;
}
use self::prelude::*;

/// NodeHealthCheckSpec defines the desired state of NodeHealthCheck
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "remediation.medik8s.io", version = "v1alpha1", kind = "NodeHealthCheck", plural = "nodehealthchecks")]
#[kube(status = "NodeHealthCheckStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct NodeHealthCheckSpec {
    /// EscalatingRemediations contain a list of ordered remediation templates with a timeout.
    /// The remediation templates will be used one after another, until the unhealthy node
    /// gets healthy within the timeout of the currently processed remediation. The order of
    /// remediation is defined by the "order" field of each "escalatingRemediation".
    /// 
    /// 
    /// Mutually exclusive with RemediationTemplate
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "escalatingRemediations")]
    pub escalating_remediations: Option<Vec<NodeHealthCheckEscalatingRemediations>>,
    /// Remediation is allowed if at least "MinHealthy" nodes selected by "selector" are healthy.
    /// Expects either a positive integer value or a percentage value.
    /// Percentage values must be positive whole numbers and are capped at 100%.
    /// 100% is valid and will block all remediation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minHealthy")]
    pub min_healthy: Option<IntOrString>,
    /// PauseRequests will prevent any new remediation to start, while in-flight remediations
    /// keep running. Each entry is free form, and ideally represents the requested party reason
    /// for this pausing - i.e:
    ///     "imaginary-cluster-upgrade-manager-operator"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pauseRequests")]
    pub pause_requests: Option<Vec<String>>,
    /// RemediationTemplate is a reference to a remediation template
    /// provided by an infrastructure provider.
    /// 
    /// 
    /// If a node needs remediation the controller will create an object from this template
    /// and then it should be picked up by a remediation provider.
    /// 
    /// 
    /// Mutually exclusive with EscalatingRemediations
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "remediationTemplate")]
    pub remediation_template: Option<ObjectReference>,
    /// Label selector to match nodes whose health will be exercised.
    /// 
    /// 
    /// Selecting both control-plane and worker nodes in one NHC CR is
    /// highly discouraged and can result in undesired behaviour.
    /// 
    /// 
    /// Note: mandatory now for above reason, but for backwards compatibility existing
    /// CRs will continue to work with an empty selector, which matches all nodes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<NodeHealthCheckSelector>,
    /// UnhealthyConditions contains a list of the conditions that determine
    /// whether a node is considered unhealthy.  The conditions are combined in a
    /// logical OR, i.e. if any of the conditions is met, the node is unhealthy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unhealthyConditions")]
    pub unhealthy_conditions: Option<Vec<NodeHealthCheckUnhealthyConditions>>,
}

/// EscalatingRemediation defines a remediation template with order and timeout
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeHealthCheckEscalatingRemediations {
    /// Order defines the order for this remediation.
    /// Remediations with lower order will be used before remediations with higher order.
    /// Remediations must not have the same order.
    pub order: i64,
    /// RemediationTemplate is a reference to a remediation template
    /// provided by a remediation provider.
    /// 
    /// 
    /// If a node needs remediation the controller will create an object from this template
    /// and then it should be picked up by a remediation provider.
    #[serde(rename = "remediationTemplate")]
    pub remediation_template: ObjectReference,
    /// Timeout defines how long NHC will wait for the node getting healthy
    /// before the next remediation (if any) will be used. When the last remediation times out,
    /// the overall remediation is considered as failed.
    /// As a safeguard for preventing parallel remediations, a minimum of 60s is enforced.
    /// 
    /// 
    /// Expects a string of decimal numbers each with optional
    /// fraction and a unit suffix, eg "300ms", "1.5h" or "2h45m".
    /// Valid time units are "ns", "us" (or "µs"), "ms", "s", "m", "h".
    pub timeout: String,
}

/// RemediationTemplate is a reference to a remediation template
/// provided by a remediation provider.
/// 
/// 
/// If a node needs remediation the controller will create an object from this template
/// and then it should be picked up by a remediation provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeHealthCheckEscalatingRemediationsRemediationTemplate {
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
    /// TODO: this design is not final and this field is subject to change in the future.
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

/// RemediationTemplate is a reference to a remediation template
/// provided by an infrastructure provider.
/// 
/// 
/// If a node needs remediation the controller will create an object from this template
/// and then it should be picked up by a remediation provider.
/// 
/// 
/// Mutually exclusive with EscalatingRemediations
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeHealthCheckRemediationTemplate {
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
    /// TODO: this design is not final and this field is subject to change in the future.
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

/// Label selector to match nodes whose health will be exercised.
/// 
/// 
/// Selecting both control-plane and worker nodes in one NHC CR is
/// highly discouraged and can result in undesired behaviour.
/// 
/// 
/// Note: mandatory now for above reason, but for backwards compatibility existing
/// CRs will continue to work with an empty selector, which matches all nodes.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeHealthCheckSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<NodeHealthCheckSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeHealthCheckSelectorMatchExpressions {
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

/// UnhealthyCondition represents a Node condition type and value with a
/// specified duration. When the named condition has been in the given
/// status for at least the duration value a node is considered unhealthy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeHealthCheckUnhealthyConditions {
    /// Duration of the condition specified when a node is considered unhealthy.
    /// 
    /// 
    /// Expects a string of decimal numbers each with optional
    /// fraction and a unit suffix, eg "300ms", "1.5h" or "2h45m".
    /// Valid time units are "ns", "us" (or "µs"), "ms", "s", "m", "h".
    pub duration: String,
    /// The condition status in the node's status to watch for.
    /// Typically False, True or Unknown.
    pub status: String,
    /// The condition type in the node's status to watch for.
    #[serde(rename = "type")]
    pub r#type: String,
}

/// NodeHealthCheckStatus defines the observed state of NodeHealthCheck
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeHealthCheckStatus {
    /// Represents the observations of a NodeHealthCheck's current state.
    /// Known .status.conditions.type are: "Disabled"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// HealthyNodes specified the number of healthy nodes observed
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "healthyNodes")]
    pub healthy_nodes: Option<i64>,
    /// InFlightRemediations records the timestamp when remediation triggered per node.
    /// Deprecated in favour of UnhealthyNodes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inFlightRemediations")]
    pub in_flight_remediations: Option<BTreeMap<String, String>>,
    /// LastUpdateTime is the last time the status was updated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUpdateTime")]
    pub last_update_time: Option<String>,
    /// ObservedNodes specified the number of nodes observed by using the NHC spec.selector
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedNodes")]
    pub observed_nodes: Option<i64>,
    /// Phase represents the current phase of this Config.
    /// Known phases are Disabled, Paused, Remediating and Enabled, based on:\n
    /// - the status of the Disabled condition\n
    /// - the value of PauseRequests\n
    /// - the value of InFlightRemediations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// Reason explains the current phase in more detail.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// UnhealthyNodes tracks currently unhealthy nodes and their remediations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unhealthyNodes")]
    pub unhealthy_nodes: Option<Vec<NodeHealthCheckStatusUnhealthyNodes>>,
}

/// UnhealthyNode defines an unhealthy node and its remediations
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeHealthCheckStatusUnhealthyNodes {
    /// ConditionsHealthyTimestamp is RFC 3339 date and time at which the unhealthy conditions didn't match anymore.
    /// The remediation CR will be deleted at that time, but the node will still be tracked as unhealthy until all
    /// remediation CRs are actually deleted, when remediators finished cleanup and removed their finalizers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "conditionsHealthyTimestamp")]
    pub conditions_healthy_timestamp: Option<String>,
    /// Name is the name of the unhealthy node
    pub name: String,
    /// Remediations tracks the remediations created for this node
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remediations: Option<Vec<NodeHealthCheckStatusUnhealthyNodesRemediations>>,
}

/// Remediation defines a remediation which was created for a node
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeHealthCheckStatusUnhealthyNodesRemediations {
    /// Resource is the reference to the remediation CR which was created
    pub resource: ObjectReference,
    /// Started is the creation time of the remediation CR
    pub started: String,
    /// TemplateName is required when using several templates of the same kind
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "templateName")]
    pub template_name: Option<String>,
    /// TimedOut is the time when the remediation timed out.
    /// Applicable for escalating remediations only.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timedOut")]
    pub timed_out: Option<String>,
}

/// Resource is the reference to the remediation CR which was created
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeHealthCheckStatusUnhealthyNodesRemediationsResource {
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
    /// TODO: this design is not final and this field is subject to change in the future.
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

