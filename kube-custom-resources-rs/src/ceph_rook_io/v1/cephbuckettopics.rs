// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/rook/rook/ceph.rook.io/v1/cephbuckettopics.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// BucketTopicSpec represent the spec of a Bucket Topic
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "ceph.rook.io", version = "v1", kind = "CephBucketTopic", plural = "cephbuckettopics")]
#[kube(namespaced)]
#[kube(status = "CephBucketTopicStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct CephBucketTopicSpec {
    /// Contains the endpoint spec of the topic
    pub endpoint: CephBucketTopicEndpoint,
    /// The name of the object store on which to define the topic
    #[serde(rename = "objectStoreName")]
    pub object_store_name: String,
    /// The namespace of the object store on which to define the topic
    #[serde(rename = "objectStoreNamespace")]
    pub object_store_namespace: String,
    /// Data which is sent in each event
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "opaqueData")]
    pub opaque_data: Option<String>,
    /// Indication whether notifications to this endpoint are persistent or not
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persistent: Option<bool>,
}

/// Contains the endpoint spec of the topic
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBucketTopicEndpoint {
    /// Spec of AMQP endpoint
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amqp: Option<CephBucketTopicEndpointAmqp>,
    /// Spec of HTTP endpoint
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<CephBucketTopicEndpointHttp>,
    /// Spec of Kafka endpoint
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kafka: Option<CephBucketTopicEndpointKafka>,
}

/// Spec of AMQP endpoint
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBucketTopicEndpointAmqp {
    /// The ack level required for this topic (none/broker/routeable)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackLevel")]
    pub ack_level: Option<CephBucketTopicEndpointAmqpAckLevel>,
    /// Indicate whether the server certificate is validated by the client or not
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disableVerifySSL")]
    pub disable_verify_ssl: Option<bool>,
    /// Name of the exchange that is used to route messages based on topics
    pub exchange: String,
    /// The URI of the AMQP endpoint to push notification to
    pub uri: String,
}

/// Spec of AMQP endpoint
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephBucketTopicEndpointAmqpAckLevel {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "broker")]
    Broker,
    #[serde(rename = "routeable")]
    Routeable,
}

/// Spec of HTTP endpoint
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBucketTopicEndpointHttp {
    /// Indicate whether the server certificate is validated by the client or not
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disableVerifySSL")]
    pub disable_verify_ssl: Option<bool>,
    /// Send the notifications with the CloudEvents header: https://github.com/cloudevents/spec/blob/main/cloudevents/adapters/aws-s3.md
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sendCloudEvents")]
    pub send_cloud_events: Option<bool>,
    /// The URI of the HTTP endpoint to push notification to
    pub uri: String,
}

/// Spec of Kafka endpoint
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBucketTopicEndpointKafka {
    /// The ack level required for this topic (none/broker)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackLevel")]
    pub ack_level: Option<CephBucketTopicEndpointKafkaAckLevel>,
    /// Indicate whether the server certificate is validated by the client or not
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disableVerifySSL")]
    pub disable_verify_ssl: Option<bool>,
    /// The URI of the Kafka endpoint to push notification to
    pub uri: String,
    /// Indicate whether to use SSL when communicating with the broker
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useSSL")]
    pub use_ssl: Option<bool>,
}

/// Spec of Kafka endpoint
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephBucketTopicEndpointKafkaAckLevel {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "broker")]
    Broker,
}

/// BucketTopicStatus represents the Status of a CephBucketTopic
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBucketTopicStatus {
    /// The ARN of the topic generated by the RGW
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ARN")]
    pub arn: Option<String>,
    /// ObservedGeneration is the latest generation observed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}

