// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubeedge/kubeedge/apps.kubeedge.io/v1alpha1/nodegroups.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// Spec represents the specification of the desired behavior of member nodegroup.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "apps.kubeedge.io", version = "v1alpha1", kind = "NodeGroup", plural = "nodegroups")]
#[kube(status = "NodeGroupStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct NodeGroupSpec {
    /// MatchLabels are used to select nodes that have these labels.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
    /// Nodes contains names of all the nodes in the nodegroup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<String>>,
    /// topologyEnabled indicates whether the topology is enabled for this NodeGroup.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "topologyEnabled")]
    pub topology_enabled: Option<bool>,
}

/// Status represents the status of member nodegroup.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeGroupStatus {
    /// NodeStatuses is a status list of all selected nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeStatuses")]
    pub node_statuses: Option<Vec<NodeGroupStatusNodeStatuses>>,
}

/// NodeStatus contains status of node that selected by this NodeGroup.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeGroupStatusNodeStatuses {
    /// NodeName contains name of this node.
    #[serde(rename = "nodeName")]
    pub node_name: String,
    /// ReadyStatus contains ready status of this node.
    #[serde(rename = "readyStatus")]
    pub ready_status: String,
    /// SelectionStatus contains status of the selection result for this node.
    #[serde(rename = "selectionStatus")]
    pub selection_status: String,
    /// SelectionStatusReason contains human-readable reason for this SelectionStatus.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "selectionStatusReason")]
    pub selection_status_reason: Option<String>,
}

