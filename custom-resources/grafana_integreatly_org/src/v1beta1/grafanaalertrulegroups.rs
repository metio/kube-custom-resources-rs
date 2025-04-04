// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/grafana/grafana-operator/grafana.integreatly.org/v1beta1/grafanaalertrulegroups.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// GrafanaAlertRuleGroupSpec defines the desired state of GrafanaAlertRuleGroup
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "grafana.integreatly.org", version = "v1beta1", kind = "GrafanaAlertRuleGroup", plural = "grafanaalertrulegroups")]
#[kube(namespaced)]
#[kube(status = "GrafanaAlertRuleGroupStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct GrafanaAlertRuleGroupSpec {
    /// Allow the Operator to match this resource with Grafanas outside the current namespace
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowCrossNamespaceImport")]
    pub allow_cross_namespace_import: Option<bool>,
    /// Whether to enable or disable editing of the alert rule group in Grafana UI
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    /// Match GrafanaFolders CRs to infer the uid
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "folderRef")]
    pub folder_ref: Option<String>,
    /// UID of the folder containing this rule group
    /// Overrides the FolderSelector
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "folderUID")]
    pub folder_uid: Option<String>,
    /// Selects Grafana instances for import
    #[serde(rename = "instanceSelector")]
    pub instance_selector: GrafanaAlertRuleGroupInstanceSelector,
    pub interval: String,
    /// Name of the alert rule group. If not specified, the resource name will be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// How often the resource is synced, defaults to 10m0s if not set
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resyncPeriod")]
    pub resync_period: Option<String>,
    pub rules: Vec<GrafanaAlertRuleGroupRules>,
}

/// Selects Grafana instances for import
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaAlertRuleGroupInstanceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<GrafanaAlertRuleGroupInstanceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaAlertRuleGroupInstanceSelectorMatchExpressions {
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

/// AlertRule defines a specific rule to be evaluated. It is based on the upstream model with some k8s specific type mappings
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GrafanaAlertRuleGroupRules {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    pub condition: String,
    pub data: Vec<GrafanaAlertRuleGroupRulesData>,
    #[serde(rename = "execErrState")]
    pub exec_err_state: GrafanaAlertRuleGroupRulesExecErrState,
    #[serde(rename = "for")]
    pub r#for: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isPaused")]
    pub is_paused: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    #[serde(rename = "noDataState")]
    pub no_data_state: GrafanaAlertRuleGroupRulesNoDataState,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notificationSettings")]
    pub notification_settings: Option<GrafanaAlertRuleGroupRulesNotificationSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record: Option<GrafanaAlertRuleGroupRulesRecord>,
    pub title: String,
    /// UID of the alert rule. Can be any string consisting of alphanumeric characters, - and _ with a maximum length of 40
    pub uid: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaAlertRuleGroupRulesData {
    /// Grafana data source unique identifier; it should be '__expr__' for a Server Side Expression operation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "datasourceUid")]
    pub datasource_uid: Option<String>,
    /// JSON is the raw JSON query and includes the above properties as well as custom properties.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<serde_json::Value>,
    /// QueryType is an optional identifier for the type of query.
    /// It can be used to distinguish different types of queries.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "queryType")]
    pub query_type: Option<String>,
    /// RefID is the unique identifier of the query, set by the frontend call.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refId")]
    pub ref_id: Option<String>,
    /// relative time range
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "relativeTimeRange")]
    pub relative_time_range: Option<GrafanaAlertRuleGroupRulesDataRelativeTimeRange>,
}

/// relative time range
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaAlertRuleGroupRulesDataRelativeTimeRange {
    /// from
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    /// to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

/// AlertRule defines a specific rule to be evaluated. It is based on the upstream model with some k8s specific type mappings
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum GrafanaAlertRuleGroupRulesExecErrState {
    #[serde(rename = "OK")]
    Ok,
    Alerting,
    Error,
    KeepLast,
}

/// AlertRule defines a specific rule to be evaluated. It is based on the upstream model with some k8s specific type mappings
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum GrafanaAlertRuleGroupRulesNoDataState {
    Alerting,
    NoData,
    #[serde(rename = "OK")]
    Ok,
    KeepLast,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaAlertRuleGroupRulesNotificationSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_interval: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_wait: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mute_time_intervals: Option<Vec<String>>,
    pub receiver: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repeat_interval: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaAlertRuleGroupRulesRecord {
    pub from: String,
    pub metric: String,
}

/// The most recent observed state of a Grafana resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaAlertRuleGroupStatus {
    /// Results when synchonizing resource with Grafana instances
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Last time the resource was synchronized with Grafana instances
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastResync")]
    pub last_resync: Option<String>,
}

