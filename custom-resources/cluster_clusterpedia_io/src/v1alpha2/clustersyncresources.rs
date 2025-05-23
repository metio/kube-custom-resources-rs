// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/clusterpedia-io/clusterpedia/cluster.clusterpedia.io/v1alpha2/clustersyncresources.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "cluster.clusterpedia.io", version = "v1alpha2", kind = "ClusterSyncResources", plural = "clustersyncresources")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ClusterSyncResourcesSpec {
    #[serde(rename = "syncResources")]
    pub sync_resources: Vec<ClusterSyncResourcesSyncResources>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterSyncResourcesSyncResources {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "eventsInvolvedResources")]
    pub events_involved_resources: Option<Vec<String>>,
    pub group: String,
    pub resources: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<String>>,
}

