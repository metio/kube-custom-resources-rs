// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/emqx/emqx-operator/apps.emqx.io/v1beta3/emqxplugins.yaml --derive=Default --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "apps.emqx.io", version = "v1beta3", kind = "EmqxPlugin", plural = "emqxplugins")]
#[kube(namespaced)]
#[kube(status = "EmqxPluginStatus")]
#[kube(schema = "disabled")]
pub struct EmqxPluginSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pluginName")]
    pub plugin_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EmqxPluginStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}

