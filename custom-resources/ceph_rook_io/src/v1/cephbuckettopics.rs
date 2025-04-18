// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/rook/rook/ceph.rook.io/v1/cephbuckettopics.yaml
// kopium version: 0.21.2

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
    /// The kafka password to use for authentication
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "PasswordSecretRef")]
    pub password_secret_ref: Option<CephBucketTopicEndpointKafkaPasswordSecretRef>,
    /// The kafka user name to use for authentication
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "UserSecretRef")]
    pub user_secret_ref: Option<CephBucketTopicEndpointKafkaUserSecretRef>,
    /// The ack level required for this topic (none/broker)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackLevel")]
    pub ack_level: Option<CephBucketTopicEndpointKafkaAckLevel>,
    /// Indicate whether the server certificate is validated by the client or not
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disableVerifySSL")]
    pub disable_verify_ssl: Option<bool>,
    /// The authentication mechanism for this topic (PLAIN/SCRAM-SHA-512/SCRAM-SHA-256/GSSAPI/OAUTHBEARER)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mechanism: Option<CephBucketTopicEndpointKafkaMechanism>,
    /// The URI of the Kafka endpoint to push notification to
    pub uri: String,
    /// Indicate whether to use SSL when communicating with the broker
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useSSL")]
    pub use_ssl: Option<bool>,
}

/// The kafka password to use for authentication
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBucketTopicEndpointKafkaPasswordSecretRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// The kafka user name to use for authentication
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBucketTopicEndpointKafkaUserSecretRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Spec of Kafka endpoint
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephBucketTopicEndpointKafkaAckLevel {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "broker")]
    Broker,
}

/// Spec of Kafka endpoint
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephBucketTopicEndpointKafkaMechanism {
    #[serde(rename = "PLAIN")]
    Plain,
    #[serde(rename = "SCRAM-SHA-512")]
    ScramSha512,
    #[serde(rename = "SCRAM-SHA-256")]
    ScramSha256,
    #[serde(rename = "GSSAPI")]
    Gssapi,
    #[serde(rename = "OAUTHBEARER")]
    Oauthbearer,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<CephBucketTopicStatusSecrets>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBucketTopicStatusSecrets {
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID is a type that holds unique ID values, including UUIDs.  Because we
    /// don't ONLY use UUIDs, this is an alias to string.  Being a type captures
    /// intent and helps make sure that UIDs and names do not get conflated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

