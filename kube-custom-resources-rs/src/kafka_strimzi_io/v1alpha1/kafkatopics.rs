// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/strimzi/strimzi-kafka-operator/kafka.strimzi.io/v1alpha1/kafkatopics.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// The specification of the topic.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kafka.strimzi.io", version = "v1alpha1", kind = "KafkaTopic", plural = "kafkatopics")]
#[kube(namespaced)]
#[kube(status = "KafkaTopicStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct KafkaTopicSpec {
    /// The topic configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, serde_json::Value>>,
    /// The number of partitions the topic should have. This cannot be decreased after topic creation. It can be increased after topic creation, but it is important to understand the consequences that has, especially for topics with semantic partitioning. When absent this will default to the broker configuration for `num.partitions`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partitions: Option<i64>,
    /// The number of replicas the topic should have. When absent this will default to the broker configuration for `default.replication.factor`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i64>,
    /// The name of the topic. When absent this will default to the metadata.name of the topic. It is recommended to not set this unless the topic name is not a valid Kubernetes resource name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "topicName")]
    pub topic_name: Option<String>,
}

/// The status of the topic.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KafkaTopicStatus {
    /// List of status conditions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The generation of the CRD that was last reconciled by the operator.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// Replication factor change status.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicasChange")]
    pub replicas_change: Option<KafkaTopicStatusReplicasChange>,
    /// The topic's id. For a KafkaTopic with the ready condition, this will change only if the topic gets deleted and recreated with the same name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "topicId")]
    pub topic_id: Option<String>,
    /// Topic name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "topicName")]
    pub topic_name: Option<String>,
}

/// Replication factor change status.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KafkaTopicStatusReplicasChange {
    /// Message for the user related to the replicas change request. This may contain transient error messages that would disappear on periodic reconciliations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The session identifier for replicas change requests pertaining to this KafkaTopic resource. This is used by the Topic Operator to track the status of `ongoing` replicas change operations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionId")]
    pub session_id: Option<String>,
    /// Current state of the replicas change operation. This can be `pending`, when the change has been requested, or `ongoing`, when the change has been successfully submitted to Cruise Control.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<KafkaTopicStatusReplicasChangeState>,
    /// The target replicas value requested by the user. This may be different from .spec.replicas when a change is ongoing.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetReplicas")]
    pub target_replicas: Option<i64>,
}

/// Replication factor change status.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum KafkaTopicStatusReplicasChangeState {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "ongoing")]
    Ongoing,
}

