// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/openshift/api/config.openshift.io/v1/operatorhubs.yaml --derive=Default --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// OperatorHubSpec defines the desired state of OperatorHub
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "config.openshift.io", version = "v1", kind = "OperatorHub", plural = "operatorhubs")]
#[kube(status = "OperatorHubStatus")]
#[kube(schema = "disabled")]
pub struct OperatorHubSpec {
    /// disableAllDefaultSources allows you to disable all the default hub sources. If this is true, a specific entry in sources can be used to enable a default source. If this is false, a specific entry in sources can be used to disable or enable a default source.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disableAllDefaultSources")]
    pub disable_all_default_sources: Option<bool>,
    /// sources is the list of default hub sources and their configuration. If the list is empty, it implies that the default hub sources are enabled on the cluster unless disableAllDefaultSources is true. If disableAllDefaultSources is true and sources is not empty, the configuration present in sources will take precedence. The list of default hub sources and their current state will always be reflected in the status block.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<OperatorHubSources>>,
}

/// HubSource is used to specify the hub source and its configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OperatorHubSources {
    /// disabled is used to disable a default hub source on cluster
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// name is the name of one of the default hub sources
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// OperatorHubStatus defines the observed state of OperatorHub. The current state of the default hub sources will always be reflected here.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OperatorHubStatus {
    /// sources encapsulates the result of applying the configuration for each hub source
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<OperatorHubStatusSources>>,
}

/// HubSourceStatus is used to reflect the current state of applying the configuration to a default source
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OperatorHubStatusSources {
    /// disabled is used to disable a default hub source on cluster
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// message provides more information regarding failures
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// name is the name of one of the default hub sources
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// status indicates success or failure in applying the configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

