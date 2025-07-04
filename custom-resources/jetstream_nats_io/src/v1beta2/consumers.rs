// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/nats-io/nack/jetstream.nats.io/v1beta2/consumers.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "jetstream.nats.io", version = "v1beta2", kind = "Consumer", plural = "consumers")]
#[kube(namespaced)]
#[kube(status = "ConsumerStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ConsumerSpec {
    /// Name of the account to which the Consumer belongs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// How messages should be acknowledged.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackPolicy")]
    pub ack_policy: Option<ConsumerAckPolicy>,
    /// How long to allow messages to remain un-acknowledged before attempting redelivery.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackWait")]
    pub ack_wait: Option<String>,
    /// List of durations representing a retry time scale for NaK'd or retried messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backoff: Option<Vec<String>>,
    /// NATS user credentials for connecting to servers. Please make sure your controller has mounted the creds on its path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creds: Option<String>,
    /// The name of a queue group.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deliverGroup")]
    pub deliver_group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deliverPolicy")]
    pub deliver_policy: Option<ConsumerDeliverPolicy>,
    /// The subject to deliver observed messages, when not set, a pull-based Consumer is created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deliverSubject")]
    pub deliver_subject: Option<String>,
    /// The description of the consumer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the Consumer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "durableName")]
    pub durable_name: Option<String>,
    /// Select only a specific incoming subjects, supports wildcards.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "filterSubject")]
    pub filter_subject: Option<String>,
    /// List of incoming subjects, supports wildcards. Available since 2.10.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "filterSubjects")]
    pub filter_subjects: Option<Vec<String>>,
    /// Enables flow control.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "flowControl")]
    pub flow_control: Option<bool>,
    /// When set, only the headers of messages in the stream are delivered, and not the bodies. Additionally, Nats-Msg-Size header is added to indicate the size of the removed payload.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "headersOnly")]
    pub headers_only: Option<bool>,
    /// The interval used to deliver idle heartbeats for push-based consumers, in Go's time.Duration format.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "heartbeatInterval")]
    pub heartbeat_interval: Option<String>,
    /// The idle time an Ephemeral Consumer allows before it is removed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inactiveThreshold")]
    pub inactive_threshold: Option<String>,
    /// The JetStream domain to use for the consumer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jsDomain")]
    pub js_domain: Option<String>,
    /// Maximum pending Acks before consumers are paused.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxAckPending")]
    pub max_ack_pending: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxDeliver")]
    pub max_deliver: Option<i64>,
    /// The largest batch property that may be specified when doing a pull on a Pull Consumer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxRequestBatch")]
    pub max_request_batch: Option<i64>,
    /// The maximum expires duration that may be set when doing a pull on a Pull Consumer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxRequestExpires")]
    pub max_request_expires: Option<String>,
    /// The maximum max_bytes value that maybe set when dong a pull on a Pull Consumer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxRequestMaxBytes")]
    pub max_request_max_bytes: Option<i64>,
    /// The number of pulls that can be outstanding on a pull consumer, pulls received after this is reached are ignored.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxWaiting")]
    pub max_waiting: Option<i64>,
    /// Force the consumer state to be kept in memory rather than inherit the setting from the stream.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memStorage")]
    pub mem_storage: Option<bool>,
    /// Additional Consumer metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, String>>,
    /// NATS user NKey for connecting to servers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nkey: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "optStartSeq")]
    pub opt_start_seq: Option<i64>,
    /// Time format must be RFC3339.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "optStartTime")]
    pub opt_start_time: Option<String>,
    /// When true, the managed Consumer will not be deleted when the resource is deleted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preventDelete")]
    pub prevent_delete: Option<bool>,
    /// When true, the managed Consumer will not be updated when the resource is updated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preventUpdate")]
    pub prevent_update: Option<bool>,
    /// Rate at which messages will be delivered to clients, expressed in bit per second.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rateLimitBps")]
    pub rate_limit_bps: Option<i64>,
    /// How messages are sent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replayPolicy")]
    pub replay_policy: Option<ConsumerReplayPolicy>,
    /// When set do not inherit the replica count from the stream but specifically set it to this amount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i64>,
    /// What percentage of acknowledgements should be samples for observability.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sampleFreq")]
    pub sample_freq: Option<String>,
    /// A list of servers for creating consumer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<String>>,
    /// The name of the Stream to create the Consumer in.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "streamName")]
    pub stream_name: Option<String>,
    /// A client's TLS certs and keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<ConsumerTls>,
    /// When true, the KV Store will initiate TLS before server INFO.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsFirst")]
    pub tls_first: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConsumerAckPolicy {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "all")]
    All,
    #[serde(rename = "explicit")]
    Explicit,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConsumerDeliverPolicy {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "last")]
    Last,
    #[serde(rename = "new")]
    New,
    #[serde(rename = "byStartSequence")]
    ByStartSequence,
    #[serde(rename = "byStartTime")]
    ByStartTime,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConsumerReplayPolicy {
    #[serde(rename = "instant")]
    Instant,
    #[serde(rename = "original")]
    Original,
}

/// A client's TLS certs and keys.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConsumerTls {
    /// A client's cert filepath. Should be mounted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientCert")]
    pub client_cert: Option<String>,
    /// A client's key filepath. Should be mounted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientKey")]
    pub client_key: Option<String>,
    /// A list of filepaths to CAs. Should be mounted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rootCas")]
    pub root_cas: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConsumerStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
}

