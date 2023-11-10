// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/VictoriaMetrics/operator/operator.victoriametrics.com/v1beta1/vmrules.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// VMRuleSpec defines the desired state of VMRule
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "operator.victoriametrics.com", version = "v1beta1", kind = "VMRule", plural = "vmrules")]
#[kube(namespaced)]
#[kube(status = "VMRuleStatus")]
#[kube(schema = "disabled")]
pub struct VMRuleSpec {
    /// Groups list of group rules
    pub groups: Vec<VMRuleGroups>,
}

/// RuleGroup is a list of sequentially evaluated recording and alerting rules.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMRuleGroups {
    /// Concurrency defines how many rules execute at once.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<i64>,
    /// ExtraFilterLabels optional list of label filters applied to every rule's request withing a group. Is compatible only with VM datasource. See more details at https://docs.victoriametrics.com#prometheus-querying-api-enhancements Deprecated, use params instead
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_filter_labels: Option<BTreeMap<String, String>>,
    /// Headers contains optional HTTP headers added to each rule request Must be in form `header-name: value` For example: headers: - "CustomHeader: foo" - "CustomHeader2: bar"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<String>>,
    /// evaluation interval for group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Labels optional list of labels added to every rule within a group. It has priority over the external labels. Labels are commonly used for adding environment or tenant-specific tag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// Limit the number of alerts an alerting rule and series a recording rule can produce
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Name of group
    pub name: String,
    /// NotifierHeaders contains optional HTTP headers added to each alert request which will send to notifier Must be in form `header-name: value` For example: headers: - "CustomHeader: foo" - "CustomHeader2: bar"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notifier_headers: Option<Vec<String>>,
    /// Params optional HTTP URL parameters added to each rule request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<BTreeMap<String, String>>,
    /// Rules list of alert rules
    pub rules: Vec<VMRuleGroupsRules>,
    /// Tenant id for group, can be used only with enterprise version of vmalert See more details at https://docs.victoriametrics.com/vmalert.html#multitenancy
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
    /// Type defines datasource type for enterprise version of vmalert possible values - prometheus,graphite
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// Rule describes an alerting or recording rule.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMRuleGroupsRules {
    /// Alert is a name for alert
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alert: Option<String>,
    /// Annotations will be added to rule configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Debug enables logging for rule it useful for tracking
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debug: Option<bool>,
    /// Expr is query, that will be evaluated at dataSource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expr: Option<String>,
    /// For evaluation interval in time.Duration format 30s, 1m, 1h  or nanoseconds
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "for")]
    pub r#for: Option<String>,
    /// KeepFiringFor will make alert continue firing for this long even when the alerting expression no longer has results. Use time.Duration format, 30s, 1m, 1h  or nanoseconds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keep_firing_for: Option<String>,
    /// Labels will be added to rule configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// Record represents a query, that will be recorded to dataSource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record: Option<String>,
    /// UpdateEntriesLimit defines max number of rule's state updates stored in memory. Overrides `-rule.updateEntriesLimit` in vmalert.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_entries_limit: Option<i64>,
}

/// VMRuleStatus defines the observed state of VMRule
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMRuleStatus {
}

