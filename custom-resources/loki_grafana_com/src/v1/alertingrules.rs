// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/grafana/loki/loki.grafana.com/v1/alertingrules.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// AlertingRuleSpec defines the desired state of AlertingRule
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "loki.grafana.com", version = "v1", kind = "AlertingRule", plural = "alertingrules")]
#[kube(namespaced)]
#[kube(status = "AlertingRuleStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct AlertingRuleSpec {
    /// List of groups for alerting rules.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<AlertingRuleGroups>>,
    /// TenantID of tenant where the alerting rules are evaluated in.
    #[serde(rename = "tenantID")]
    pub tenant_id: String,
}

/// AlertingRuleGroup defines a group of Loki alerting rules.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AlertingRuleGroups {
    /// Interval defines the time interval between evaluation of the given
    /// alerting rule.
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
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AlertingRuleGroupsRules {
    /// The name of the alert. Must be a valid label value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alert: Option<String>,
    /// Annotations to add to each alert.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// The LogQL expression to evaluate. Every evaluation cycle this is
    /// evaluated at the current time, and all resultant time series become
    /// pending/firing alerts.
    pub expr: String,
    /// Alerts are considered firing once they have been returned for this long.
    /// Alerts which have not yet fired for long enough are considered pending.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "for")]
    pub r#for: Option<String>,
    /// Labels to add to each alert.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

/// AlertingRuleStatus defines the observed state of AlertingRule
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AlertingRuleStatus {
    /// Conditions of the AlertingRule generation health.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

