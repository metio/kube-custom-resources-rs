// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/grafana/loki/loki.grafana.com/v1/alertingrules.yaml --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// AlertingRuleSpec defines the desired state of AlertingRule
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "loki.grafana.com", version = "v1", kind = "AlertingRule", plural = "alertingrules")]
#[kube(namespaced)]
#[kube(status = "AlertingRuleStatus")]
#[kube(schema = "disabled")]
pub struct AlertingRuleSpec {
    /// List of groups for alerting rules.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<AlertingRuleGroups>>,
    /// TenantID of tenant where the alerting rules are evaluated in.
    #[serde(rename = "tenantID")]
    pub tenant_id: String,
}

/// AlertingRuleGroup defines a group of Loki alerting rules.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AlertingRuleGroups {
    /// Interval defines the time interval between evaluation of the given alerting rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Limit defines the number of alerts an alerting rule can produce. 0 is no limit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Name of the alerting rule group. Must be unique within all alerting rules.
    pub name: String,
    /// Rules defines a list of alerting rules
    pub rules: Vec<AlertingRuleGroupsRules>,
}

/// AlertingRuleGroupSpec defines the spec for a Loki alerting rule.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AlertingRuleGroupsRules {
    /// The name of the alert. Must be a valid label value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alert: Option<String>,
    /// Annotations to add to each alert.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// The LogQL expression to evaluate. Every evaluation cycle this is evaluated at the current time, and all resultant time series become pending/firing alerts.
    pub expr: String,
    /// Alerts are considered firing once they have been returned for this long. Alerts which have not yet fired for long enough are considered pending.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "for")]
    pub r#for: Option<String>,
    /// Labels to add to each alert.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

/// AlertingRuleStatus defines the observed state of AlertingRule
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AlertingRuleStatus {
    /// Conditions of the AlertingRule generation health.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<AlertingRuleStatusConditions>>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AlertingRuleStatusConditions {
    /// lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// message is a human readable message indicating details about the transition. This may be an empty string.
    pub message: String,
    /// observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
    pub reason: String,
    /// status of the condition, one of True, False, Unknown.
    pub status: AlertingRuleStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AlertingRuleStatusConditionsStatus {
    True,
    False,
    Unknown,
}

