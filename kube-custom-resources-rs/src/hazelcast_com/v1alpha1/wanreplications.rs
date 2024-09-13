// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/hazelcast/hazelcast-platform-operator/hazelcast.com/v1alpha1/wanreplications.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// WanReplicationSpec defines the desired state of WanReplication
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "hazelcast.com", version = "v1alpha1", kind = "WanReplication", plural = "wanreplications")]
#[kube(namespaced)]
#[kube(status = "WanReplicationStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct WanReplicationSpec {
    /// Acknowledgement is the configuration for the condition when the next batch of WAN events are sent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acknowledgement: Option<WanReplicationAcknowledgement>,
    /// Batch is the configuration for WAN events batch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch: Option<WanReplicationBatch>,
    /// Endpoints is the target cluster comma separated endpoint list .
    pub endpoints: String,
    /// Queue is the configuration for WAN events queue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue: Option<WanReplicationQueue>,
    /// Resources is the list of custom resources to which WAN replication applies.
    pub resources: Vec<WanReplicationResources>,
    /// SyncConsistencyCheckStrategy is the strategy for checking the consistency of data between replicas.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "syncConsistencyCheckStrategy")]
    pub sync_consistency_check_strategy: Option<String>,
    /// ClusterName is the clusterName field of the target Hazelcast resource.
    #[serde(rename = "targetClusterName")]
    pub target_cluster_name: String,
}

/// Acknowledgement is the configuration for the condition when the next batch of WAN events are sent.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WanReplicationAcknowledgement {
    /// Timeout represents the time in milliseconds the source cluster waits for the acknowledgement. After timeout, the events will be resent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    /// Type represents how a batch of replication events is considered successfully replicated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<WanReplicationAcknowledgementType>,
}

/// Acknowledgement is the configuration for the condition when the next batch of WAN events are sent.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum WanReplicationAcknowledgementType {
    #[serde(rename = "ACK_ON_OPERATION_COMPLETE")]
    AckOnOperationComplete,
    #[serde(rename = "ACK_ON_RECEIPT")]
    AckOnReceipt,
}

/// Batch is the configuration for WAN events batch.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WanReplicationBatch {
    /// MaximumDelay represents the maximum delay in milliseconds. If the batch size is not reached, the events will be sent after the maximum delay.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maximumDelay")]
    pub maximum_delay: Option<i32>,
    /// Size represents the maximum batch size.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

/// Queue is the configuration for WAN events queue.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WanReplicationQueue {
    /// Capacity is the total capacity of WAN queue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    /// FullBehavior represents the behavior of the new arrival when the queue is full.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fullBehavior")]
    pub full_behavior: Option<WanReplicationQueueFullBehavior>,
}

/// Queue is the configuration for WAN events queue.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum WanReplicationQueueFullBehavior {
    #[serde(rename = "DISCARD_AFTER_MUTATION")]
    DiscardAfterMutation,
    #[serde(rename = "THROW_EXCEPTION")]
    ThrowException,
    #[serde(rename = "THROW_EXCEPTION_ONLY_IF_REPLICATION_ACTIVE")]
    ThrowExceptionOnlyIfReplicationActive,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WanReplicationResources {
    /// Kind is the kind of custom resource to which WAN replication applies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<WanReplicationResourcesKind>,
    /// Name is the name of custom resource to which WAN replication applies.
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum WanReplicationResourcesKind {
    Map,
    Hazelcast,
}

/// WanReplicationStatus defines the observed state of WanReplication
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WanReplicationStatus {
    /// Message is the field to show detail information or error
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Status is the status of WAN replication
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// WanReplicationMapsStatus is the WAN Replication status of the Maps given in the spec directly or indirectly by Hazelcast resource.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "wanReplicationMapsStatus")]
    pub wan_replication_maps_status: Option<BTreeMap<String, WanReplicationStatusWanReplicationMapsStatus>>,
}

/// WanReplicationMapsStatus is the WAN Replication status of the Maps given in the spec directly or indirectly by Hazelcast resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WanReplicationStatusWanReplicationMapsStatus {
    /// Message is the field to show detail information or error
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// PublisherId is the ID used for WAN publisher ID
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "publisherId")]
    pub publisher_id: Option<String>,
    /// ResourceName is the name of the Map Custom Resource.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceName")]
    pub resource_name: Option<String>,
    /// Status is the status of WAN replication
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

