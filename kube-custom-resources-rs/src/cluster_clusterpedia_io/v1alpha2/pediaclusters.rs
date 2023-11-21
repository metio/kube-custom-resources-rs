// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/clusterpedia-io/clusterpedia/cluster.clusterpedia.io/v1alpha2/pediaclusters.yaml --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

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
    pub conditions: Option<Vec<PediaClusterStatusConditions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "syncResources")]
    pub sync_resources: Option<Vec<PediaClusterStatusSyncResources>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PediaClusterStatusConditions {
    /// lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// message is a human readable message indicating details about the transition. This may be an empty string.
    pub message: String,
    /// observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
    pub reason: String,
    /// status of the condition, one of True, False, Unknown.
    pub status: PediaClusterStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PediaClusterStatusConditionsStatus {
    True,
    False,
    Unknown,
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
