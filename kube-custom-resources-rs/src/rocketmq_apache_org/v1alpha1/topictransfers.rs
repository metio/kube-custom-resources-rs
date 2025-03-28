// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/apache/rocketmq-operator/rocketmq.apache.org/v1alpha1/topictransfers.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// TopicTransferSpec defines the desired state of TopicTransfer
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "rocketmq.apache.org", version = "v1alpha1", kind = "TopicTransfer", plural = "topictransfers")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct TopicTransferSpec {
    /// The cluster where the transferred topic from
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceCluster")]
    pub source_cluster: Option<String>,
    /// The cluster where the topic will be transferred to
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetCluster")]
    pub target_cluster: Option<String>,
    /// Topic name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
}

/// TopicTransferStatus defines the observed state of TopicTransfer
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopicTransferStatus {
}

