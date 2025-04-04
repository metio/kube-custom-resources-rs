// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubewharf/kubeadmiral/core.kubeadmiral.io/v1alpha1/clustercollectedstatuses.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// CollectedFieldsWithCluster stores the collected fields of a Kubernetes object in a member cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterCollectedStatusClusters {
    /// Cluster is the name of the member cluster.
    pub cluster: String,
    /// CollectedFields is the the set of fields collected for the Kubernetes object.
    #[serde(rename = "collectedFields")]
    pub collected_fields: serde_json::Value,
    /// Error records any errors encountered while collecting fields from the cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

