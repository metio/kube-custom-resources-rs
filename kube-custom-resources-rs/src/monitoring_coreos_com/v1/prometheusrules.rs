// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/prometheus-operator/prometheus-operator/monitoring.coreos.com/v1/prometheusrules.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
}
use self::prelude::*;

/// Specification of desired alerting rule definitions for Prometheus.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "monitoring.coreos.com", version = "v1", kind = "PrometheusRule", plural = "prometheusrules")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct PrometheusRuleSpec {
    /// Content of Prometheus rule file
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<PrometheusRuleGroups>>,
}

/// RuleGroup is a list of sequentially evaluated recording and alerting rules.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PrometheusRuleGroups {
    /// Interval determines how often rules in the group are evaluated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Labels to add or overwrite before storing the result for its rules.
    /// The labels defined at the rule level take precedence.
    /// 
    /// It requires Prometheus >= 3.0.0.
    /// The field is ignored for Thanos Ruler.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// Limit the number of alerts an alerting rule and series a recording
    /// rule can produce.
    /// Limit is supported starting with Prometheus >= 2.31 and Thanos Ruler >= 0.24.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Name of the rule group.
    pub name: String,
    /// PartialResponseStrategy is only used by ThanosRuler and will
    /// be ignored by Prometheus instances.
    /// More info: https://github.com/thanos-io/thanos/blob/main/docs/components/rule.md#partial-response
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partial_response_strategy: Option<String>,
    /// Defines the offset the rule evaluation timestamp of this particular group by the specified duration into the past.
    /// 
    /// It requires Prometheus >= v2.53.0.
    /// It is not supported for ThanosRuler.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_offset: Option<String>,
    /// List of alerting and recording rules.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<PrometheusRuleGroupsRules>>,
}

/// Rule describes an alerting or recording rule
/// See Prometheus documentation: [alerting](https://www.prometheus.io/docs/prometheus/latest/configuration/alerting_rules/) or [recording](https://www.prometheus.io/docs/prometheus/latest/configuration/recording_rules/#recording-rules) rule
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PrometheusRuleGroupsRules {
    /// Name of the alert. Must be a valid label value.
    /// Only one of `record` and `alert` must be set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alert: Option<String>,
    /// Annotations to add to each alert.
    /// Only valid for alerting rules.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// PromQL expression to evaluate.
    pub expr: IntOrString,
    /// Alerts are considered firing once they have been returned for this long.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "for")]
    pub r#for: Option<String>,
    /// KeepFiringFor defines how long an alert will continue firing after the condition that triggered it has cleared.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keep_firing_for: Option<String>,
    /// Labels to add or overwrite.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// Name of the time series to output to. Must be a valid metric name.
    /// Only one of `record` and `alert` must be set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record: Option<String>,
}

