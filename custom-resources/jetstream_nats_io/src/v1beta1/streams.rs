// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/nats-io/nack/jetstream.nats.io/v1beta1/streams.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "jetstream.nats.io", version = "v1beta1", kind = "Stream", plural = "streams")]
#[kube(namespaced)]
#[kube(status = "StreamStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct StreamSpec {
    /// The description of the stream.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// When a Stream reach it's limits either old messages are deleted or new ones are denied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discard: Option<StreamDiscard>,
    /// The duration window to track duplicate messages for.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "duplicateWindow")]
    pub duplicate_window: Option<String>,
    /// Maximum age of any message in the stream, expressed in Go's time.Duration format. Empty for unlimited.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxAge")]
    pub max_age: Option<String>,
    /// How big the Stream may be, when the combined stream size exceeds this old messages are removed. -1 for unlimited.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxBytes")]
    pub max_bytes: Option<i64>,
    /// How many Consumers can be defined for a given Stream. -1 for unlimited.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxConsumers")]
    pub max_consumers: Option<i64>,
    /// The largest message that will be accepted by the Stream. -1 for unlimited.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxMsgSize")]
    pub max_msg_size: Option<i64>,
    /// How many messages may be in a Stream, oldest messages will be removed if the Stream exceeds this size. -1 for unlimited.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxMsgs")]
    pub max_msgs: Option<i64>,
    /// The maximum number of messages per subject.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxMsgsPerSubject")]
    pub max_msgs_per_subject: Option<i64>,
    /// A stream mirror.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror: Option<StreamMirror>,
    /// A unique name for the Stream.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Disables acknowledging messages that are received by the Stream.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "noAck")]
    pub no_ack: Option<bool>,
    /// A stream's placement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<StreamPlacement>,
    /// How many replicas to keep for each message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i64>,
    /// How messages are retained in the Stream, once this is exceeded old messages are removed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention: Option<StreamRetention>,
    /// A stream's sources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<StreamSources>>,
    /// The storage backend to use for the Stream.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage: Option<StreamStorage>,
    /// A list of subjects to consume, supports wildcards.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subjects: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StreamDiscard {
    #[serde(rename = "old")]
    Old,
    #[serde(rename = "new")]
    New,
}

/// A stream mirror.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamMirror {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalApiPrefix")]
    pub external_api_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalDeliverPrefix")]
    pub external_deliver_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "filterSubject")]
    pub filter_subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "optStartSeq")]
    pub opt_start_seq: Option<i64>,
    /// Time format must be RFC3339.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "optStartTime")]
    pub opt_start_time: Option<String>,
}

/// A stream's placement.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamPlacement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StreamRetention {
    #[serde(rename = "limits")]
    Limits,
    #[serde(rename = "interest")]
    Interest,
    #[serde(rename = "workqueue")]
    Workqueue,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamSources {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalApiPrefix")]
    pub external_api_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalDeliverPrefix")]
    pub external_deliver_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "filterSubject")]
    pub filter_subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "optStartSeq")]
    pub opt_start_seq: Option<i64>,
    /// Time format must be RFC3339.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "optStartTime")]
    pub opt_start_time: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StreamStorage {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "memory")]
    Memory,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
}

