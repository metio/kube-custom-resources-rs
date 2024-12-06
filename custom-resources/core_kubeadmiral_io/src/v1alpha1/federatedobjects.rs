// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubewharf/kubeadmiral/core.kubeadmiral.io/v1alpha1/federatedobjects.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// Spec defines the desired behavior of the FederatedObject.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "core.kubeadmiral.io", version = "v1alpha1", kind = "FederatedObject", plural = "federatedobjects")]
#[kube(namespaced)]
#[kube(status = "FederatedObjectStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct FederatedObjectSpec {
    /// Follows defines other objects, or "leaders", that the Kubernetes object should follow during propagation, i.e. the Kubernetes object should be propagated to all member clusters that its "leaders" are placed in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follows: Option<Vec<FederatedObjectFollows>>,
    /// Overrides describe the overrides that should be applied to the base template of the Kubernetes object before it is propagated to individual member clusters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<Vec<FederatedObjectOverrides>>,
    /// Placements describe the member clusters that the Kubernetes object will be propagated to, which is a union of all the listed clusters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placements: Option<Vec<FederatedObjectPlacements>>,
    /// Template is the base template of the Kubernetes object to be propagated.
    pub template: serde_json::Value,
}

/// LeaderReference contains the identifying metadata of a "leader" Kubernetes object.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FederatedObjectFollows {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    pub kind: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// OverrideWithController describes the overrides that will be applied to a Kubernetes object before it is propagated to individual member clusters.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FederatedObjectOverrides {
    /// Override is the list of member clusters and their respective override patches.
    pub clusters: Vec<FederatedObjectOverridesClusters>,
    /// Controller identifies the controller responsible for this override.
    pub controller: String,
}

/// ClusterReferenceWithPatches represents a single member cluster and a list of override patches for the cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FederatedObjectOverridesClusters {
    /// Cluster is the name of the member cluster.
    pub cluster: String,
    /// Patches is the list of override patches for the member cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patches: Option<Vec<FederatedObjectOverridesClustersPatches>>,
}

/// OverridePatch defines a JSON patch.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FederatedObjectOverridesClustersPatches {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

/// PlacementWithController describes the member clusters that a Kubernetes object should be propagated to.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FederatedObjectPlacements {
    /// Controller identifies the controller responsible for this placement.
    pub controller: String,
    /// Placement is the list of member clusters that the Kubernetes object should be propagated to.
    pub placement: Vec<FederatedObjectPlacementsPlacement>,
}

/// ClusterReference represents a single member cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FederatedObjectPlacementsPlacement {
    /// Cluster is the name of the member cluster.
    pub cluster: String,
}

/// Status describes the most recently observed status of the FederatedObject.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FederatedObjectStatus {
    /// Clusters contains the propagation status of the Kubernetes object for individual member clusters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<FederatedObjectStatusClusters>>,
    /// Conditions describe the current state of this FederatedObject.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<FederatedObjectStatusConditions>>,
    /// SyncedGeneration is the generation of this FederatedObject when it was last synced to selected member clusters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "syncedGeneration")]
    pub synced_generation: Option<i64>,
}

/// PropagationStatus describes the propagation of a Kubernetes object to a given member cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FederatedObjectStatusClusters {
    /// Cluster is the name of the member cluster.
    pub cluster: String,
    /// LastObservedGeneration is the last observed generation of the Kubernetes object in the member cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastObservedGeneration")]
    pub last_observed_generation: Option<i64>,
    /// Status describes the current status of propagating the Kubernetes object to the member cluster.
    pub status: String,
}

/// GenericFederatedObjectCondition contains the current details about a particular condition of a FederatedObject.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FederatedObjectStatusConditions {
    /// LastTransitionTime is the last time the status of this condition changed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// LastUpdateTime is the last time a reconciliation for this condition occurred.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUpdateTime")]
    pub last_update_time: Option<String>,
    /// Reason is the reason for the last status change of this condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Status is the status of the condition, one of True, False or Unknown.
    pub status: String,
    /// Type is the type of the condition.
    #[serde(rename = "type")]
    pub r#type: String,
}

