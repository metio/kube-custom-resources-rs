// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/grafana-operator/grafana-operator/grafana.integreatly.org/v1beta1/grafanafolders.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "grafana.integreatly.org", version = "v1beta1", kind = "GrafanaFolder", plural = "grafanafolders")]
#[kube(namespaced)]
#[kube(status = "GrafanaFolderStatus")]
#[kube(schema = "disabled")]
pub struct GrafanaFolderSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowCrossNamespaceImport")]
    pub allow_cross_namespace_import: Option<bool>,
    #[serde(rename = "instanceSelector")]
    pub instance_selector: GrafanaFolderInstanceSelector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resyncPeriod")]
    pub resync_period: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaFolderInstanceSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<GrafanaFolderInstanceSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaFolderInstanceSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaFolderStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "NoMatchingInstances")]
    pub no_matching_instances: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastResync")]
    pub last_resync: Option<String>,
}

