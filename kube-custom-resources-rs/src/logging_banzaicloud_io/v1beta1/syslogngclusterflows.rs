// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kube-logging/logging-operator/logging.banzaicloud.io/v1beta1/syslogngclusterflows.yaml --derive=Default --derive=PartialEq
// kopium version: 0.19.0

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "logging.banzaicloud.io", version = "v1beta1", kind = "SyslogNGClusterFlow", plural = "syslogngclusterflows")]
#[kube(namespaced)]
#[kube(status = "SyslogNGClusterFlowStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct SyslogNGClusterFlowSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<SyslogNGClusterFlowFilters>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "globalOutputRefs")]
    pub global_output_refs: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "loggingRef")]
    pub logging_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<SyslogNGClusterFlowMatch>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "outputMetrics")]
    pub output_metrics: Option<Vec<SyslogNGClusterFlowOutputMetrics>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowFilters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<SyslogNGClusterFlowFiltersMatch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parser: Option<SyslogNGClusterFlowFiltersParser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rewrite: Option<Vec<SyslogNGClusterFlowFiltersRewrite>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowFiltersMatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub and: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub or: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regexp: Option<SyslogNGClusterFlowFiltersMatchRegexp>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowFiltersMatchRegexp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
    pub pattern: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowFiltersParser {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metrics-probe")]
    pub metrics_probe: Option<SyslogNGClusterFlowFiltersParserMetricsProbe>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regexp: Option<SyslogNGClusterFlowFiltersParserRegexp>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "syslog-parser")]
    pub syslog_parser: Option<SyslogNGClusterFlowFiltersParserSyslogParser>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowFiltersParserMetricsProbe {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowFiltersParserRegexp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
    pub patterns: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowFiltersParserSyslogParser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowFiltersRewrite {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_unset: Option<SyslogNGClusterFlowFiltersRewriteGroupUnset>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rename: Option<SyslogNGClusterFlowFiltersRewriteRename>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<SyslogNGClusterFlowFiltersRewriteSet>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subst: Option<SyslogNGClusterFlowFiltersRewriteSubst>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unset: Option<SyslogNGClusterFlowFiltersRewriteUnset>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowFiltersRewriteGroupUnset {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<SyslogNGClusterFlowFiltersRewriteGroupUnsetCondition>,
    pub pattern: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowFiltersRewriteGroupUnsetCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub and: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub or: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regexp: Option<SyslogNGClusterFlowFiltersRewriteGroupUnsetConditionRegexp>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowFiltersRewriteGroupUnsetConditionRegexp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
    pub pattern: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowFiltersRewriteRename {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<SyslogNGClusterFlowFiltersRewriteRenameCondition>,
    #[serde(rename = "newName")]
    pub new_name: String,
    #[serde(rename = "oldName")]
    pub old_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowFiltersRewriteRenameCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub and: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub or: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regexp: Option<SyslogNGClusterFlowFiltersRewriteRenameConditionRegexp>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowFiltersRewriteRenameConditionRegexp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
    pub pattern: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowFiltersRewriteSet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<SyslogNGClusterFlowFiltersRewriteSetCondition>,
    pub field: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowFiltersRewriteSetCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub and: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub or: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regexp: Option<SyslogNGClusterFlowFiltersRewriteSetConditionRegexp>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowFiltersRewriteSetConditionRegexp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
    pub pattern: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowFiltersRewriteSubst {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<SyslogNGClusterFlowFiltersRewriteSubstCondition>,
    pub field: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
    pub pattern: String,
    pub replace: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowFiltersRewriteSubstCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub and: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub or: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regexp: Option<SyslogNGClusterFlowFiltersRewriteSubstConditionRegexp>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowFiltersRewriteSubstConditionRegexp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
    pub pattern: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowFiltersRewriteUnset {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<SyslogNGClusterFlowFiltersRewriteUnsetCondition>,
    pub field: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowFiltersRewriteUnsetCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub and: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub or: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regexp: Option<SyslogNGClusterFlowFiltersRewriteUnsetConditionRegexp>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowFiltersRewriteUnsetConditionRegexp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
    pub pattern: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowMatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub and: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub or: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regexp: Option<SyslogNGClusterFlowMatchRegexp>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowMatchRegexp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
    pub pattern: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowOutputMetrics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyslogNGClusterFlowStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub problems: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "problemsCount")]
    pub problems_count: Option<i64>,
}

