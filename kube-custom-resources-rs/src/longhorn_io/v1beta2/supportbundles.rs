// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/longhorn/longhorn/longhorn.io/v1beta2/supportbundles.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// SupportBundleSpec defines the desired state of the Longhorn SupportBundle
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "longhorn.io", version = "v1beta2", kind = "SupportBundle", plural = "supportbundles")]
#[kube(namespaced)]
#[kube(status = "SupportBundleStatus")]
#[kube(schema = "disabled")]
pub struct SupportBundleSpec {
    /// A brief description of the issue
    pub description: String,
    /// The issue URL
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "issueURL")]
    pub issue_url: Option<String>,
    /// The preferred responsible controller node ID.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeID")]
    pub node_id: Option<String>,
}

/// SupportBundleStatus defines the observed state of the Longhorn SupportBundle
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SupportBundleStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<SupportBundleStatusConditions>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filesize: Option<i64>,
    /// The support bundle manager image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// The support bundle manager IP
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "managerIP")]
    pub manager_ip: Option<String>,
    /// The current responsible controller node ID
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SupportBundleStatusConditions {
    /// Last time we probed the condition.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastProbeTime")]
    pub last_probe_time: Option<String>,
    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// Human-readable message indicating details about last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Status is the status of the condition. Can be True, False, Unknown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Type is the type of the condition.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

