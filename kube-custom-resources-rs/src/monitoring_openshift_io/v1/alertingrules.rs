// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/openshift/api/monitoring.openshift.io/v1/alertingrules.yaml --derive=Default --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// spec describes the desired state of this AlertingRule object.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "monitoring.openshift.io", version = "v1", kind = "AlertingRule", plural = "alertingrules")]
#[kube(namespaced)]
#[kube(status = "AlertingRuleStatus")]
#[kube(schema = "disabled")]
pub struct AlertingRuleSpec {
    /// groups is a list of grouped alerting rules.  Rule groups are the unit at which Prometheus parallelizes rule processing.  All rules in a single group share a configured evaluation interval.  All rules in the group will be processed together on this interval, sequentially, and all rules will be processed. 
    ///  It's common to group related alerting rules into a single AlertingRule resources, and within that resource, closely related alerts, or simply alerts with the same interval, into individual groups.  You are also free to create AlertingRule resources with only a single rule group, but be aware that this can have a performance impact on Prometheus if the group is extremely large or has very complex query expressions to evaluate. Spreading very complex rules across multiple groups to allow them to be processed in parallel is also a common use-case.
    pub groups: Vec<AlertingRuleGroups>,
}

/// RuleGroup is a list of sequentially evaluated alerting rules.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AlertingRuleGroups {
    /// interval is how often rules in the group are evaluated.  If not specified, it defaults to the global.evaluation_interval configured in Prometheus, which itself defaults to 30 seconds.  You can check if this value has been modified from the default on your cluster by inspecting the platform Prometheus configuration: The relevant field in that resource is: spec.evaluationInterval
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// name is the name of the group.
    pub name: String,
    /// rules is a list of sequentially evaluated alerting rules.  Prometheus may process rule groups in parallel, but rules within a single group are always processed sequentially, and all rules are processed.
    pub rules: Vec<AlertingRuleGroupsRules>,
}

/// Rule describes an alerting rule. See Prometheus documentation: - https://www.prometheus.io/docs/prometheus/latest/configuration/alerting_rules
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AlertingRuleGroupsRules {
    /// alert is the name of the alert. Must be a valid label value, i.e. may contain any Unicode character.
    pub alert: String,
    /// annotations to add to each alert.  These are values that can be used to store longer additional information that you won't query on, such as alert descriptions or runbook links.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// expr is the PromQL expression to evaluate. Every evaluation cycle this is evaluated at the current time, and all resultant time series become pending or firing alerts.  This is most often a string representing a PromQL expression, e.g.: mapi_current_pending_csr > mapi_max_pending_csr In rare cases this could be a simple integer, e.g. a simple "1" if the intent is to create an alert that is always firing.  This is sometimes used to create an always-firing "Watchdog" alert in order to ensure the alerting pipeline is functional.
    pub expr: IntOrString,
    /// for is the time period after which alerts are considered firing after first returning results.  Alerts which have not yet fired for long enough are considered pending.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "for")]
    pub r#for: Option<String>,
    /// labels to add or overwrite for each alert.  The results of the PromQL expression for the alert will result in an existing set of labels for the alert, after evaluating the expression, for any label specified here with the same name as a label in that set, the label here wins and overwrites the previous value.  These should typically be short identifying values that may be useful to query against.  A common example is the alert severity, where one sets `severity: warning` under the `labels` key:
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

/// status describes the current state of this AlertOverrides object.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AlertingRuleStatus {
    /// observedGeneration is the last generation change you've dealt with.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// prometheusRule is the generated PrometheusRule for this AlertingRule.  Each AlertingRule instance results in a generated PrometheusRule object in the same namespace, which is always the openshift-monitoring namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "prometheusRule")]
    pub prometheus_rule: Option<AlertingRuleStatusPrometheusRule>,
}

/// prometheusRule is the generated PrometheusRule for this AlertingRule.  Each AlertingRule instance results in a generated PrometheusRule object in the same namespace, which is always the openshift-monitoring namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AlertingRuleStatusPrometheusRule {
    /// name of the referenced PrometheusRule.
    pub name: String,
}

