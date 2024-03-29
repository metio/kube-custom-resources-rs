// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/clusterpedia-io/clusterpedia/cluster.clusterpedia.io/v1alpha2/pediaclusters.yaml --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "cluster.clusterpedia.io", version = "v1alpha2", kind = "PediaCluster", plural = "pediaclusters")]
#[kube(status = "PediaClusterStatus")]
#[kube(schema = "disabled")]
pub struct PediaClusterSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apiserver: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caData")]
    pub ca_data: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certData")]
    pub cert_data: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyData")]
    pub key_data: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubeconfig: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "shardingName")]
    pub sharding_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "syncAllCustomResources")]
    pub sync_all_custom_resources: Option<bool>,
    #[serde(rename = "syncResources")]
    pub sync_resources: Vec<PediaClusterSyncResources>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "syncResourcesRefName")]
    pub sync_resources_ref_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenData")]
    pub token_data: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PediaClusterSyncResources {
    pub group: String,
    pub resources: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PediaClusterStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apiserver: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "shardingName")]
    pub sharding_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "syncResources")]
    pub sync_resources: Option<Vec<PediaClusterStatusSyncResources>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PediaClusterStatusSyncResources {
    pub group: String,
    pub resources: Vec<PediaClusterStatusSyncResourcesResources>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PediaClusterStatusSyncResourcesResources {
    pub kind: String,
    pub name: String,
    pub namespaced: bool,
    #[serde(rename = "syncConditions")]
    pub sync_conditions: Vec<PediaClusterStatusSyncResourcesResourcesSyncConditions>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PediaClusterStatusSyncResourcesResourcesSyncConditions {
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// optional
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// optional
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub status: String,
    /// optional
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageResource")]
    pub storage_resource: Option<String>,
    /// optional
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageVersion")]
    pub storage_version: Option<String>,
    /// optional
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "syncResource")]
    pub sync_resource: Option<String>,
    /// optional
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "syncVersion")]
    pub sync_version: Option<String>,
    pub version: String,
}

