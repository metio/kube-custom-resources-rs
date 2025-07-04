// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws-controllers-k8s/kinesis-controller/kinesis.services.k8s.aws/v1alpha1/streams.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// StreamSpec defines the desired state of Stream.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kinesis.services.k8s.aws", version = "v1alpha1", kind = "Stream", plural = "streams")]
#[kube(namespaced)]
#[kube(status = "StreamStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct StreamSpec {
    /// A name to identify the stream. The stream name is scoped to the Amazon Web
    /// Services account used by the application that creates the stream. It is also
    /// scoped by Amazon Web Services Region. That is, two streams in two different
    /// Amazon Web Services accounts can have the same name. Two streams in the same
    /// Amazon Web Services account but in two different Regions can also have the
    /// same name.
    /// 
    /// Regex Pattern: `^[a-zA-Z0-9_.-]+$`
    pub name: String,
    /// The number of shards that the stream will use. The throughput of the stream
    /// is a function of the number of shards; more shards are required for greater
    /// provisioned throughput.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "shardCount")]
    pub shard_count: Option<i64>,
    /// Indicates the capacity mode of the data stream. Currently, in Kinesis Data
    /// Streams, you can choose between an on-demand capacity mode and a provisioned
    /// capacity mode for your data streams.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "streamModeDetails")]
    pub stream_mode_details: Option<StreamStreamModeDetails>,
    /// A set of up to 10 key-value pairs to use to create the tags.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

/// Indicates the capacity mode of the data stream. Currently, in Kinesis Data
/// Streams, you can choose between an on-demand capacity mode and a provisioned
/// capacity mode for your data streams.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamStreamModeDetails {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "streamMode")]
    pub stream_mode: Option<String>,
}

/// StreamStatus defines the observed state of Stream
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<StreamStatusAckResourceMetadata>,
    /// All CRs managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The number of enhanced fan-out consumers registered with the stream.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "consumerCount")]
    pub consumer_count: Option<i64>,
    /// The encryption type used. This value is one of the following:
    /// 
    ///    * KMS
    /// 
    ///    * NONE
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "encryptionType")]
    pub encryption_type: Option<String>,
    /// Represents the current enhanced monitoring settings of the stream.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enhancedMonitoring")]
    pub enhanced_monitoring: Option<Vec<StreamStatusEnhancedMonitoring>>,
    /// The GUID for the customer-managed Amazon Web Services KMS key to use for
    /// encryption. This value can be a globally unique identifier, a fully specified
    /// ARN to either an alias or a key, or an alias name prefixed by "alias/".You
    /// can also use a master key owned by Kinesis Data Streams by specifying the
    /// alias aws/kinesis.
    /// 
    ///    * Key ARN example: arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012
    /// 
    ///    * Alias ARN example: arn:aws:kms:us-east-1:123456789012:alias/MyAliasName
    /// 
    ///    * Globally unique key ID example: 12345678-1234-1234-1234-123456789012
    /// 
    ///    * Alias name example: alias/MyAliasName
    /// 
    ///    * Master key owned by Kinesis Data Streams: alias/aws/kinesis
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyID")]
    pub key_id: Option<String>,
    /// The number of open shards in the stream.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openShardCount")]
    pub open_shard_count: Option<i64>,
    /// The current retention period, in hours.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retentionPeriodHours")]
    pub retention_period_hours: Option<i64>,
    /// The approximate time that the stream was created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "streamCreationTimestamp")]
    pub stream_creation_timestamp: Option<String>,
    /// The current status of the stream being described. The stream status is one
    /// of the following states:
    /// 
    ///    * CREATING - The stream is being created. Kinesis Data Streams immediately
    ///    returns and sets StreamStatus to CREATING.
    /// 
    ///    * DELETING - The stream is being deleted. The specified stream is in the
    ///    DELETING state until Kinesis Data Streams completes the deletion.
    /// 
    ///    * ACTIVE - The stream exists and is ready for read and write operations
    ///    or deletion. You should perform read and write operations only on an ACTIVE
    ///    stream.
    /// 
    ///    * UPDATING - Shards in the stream are being merged or split. Read and
    ///    write operations continue to work while the stream is in the UPDATING
    ///    state.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "streamStatus")]
    pub stream_status: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamStatusAckResourceMetadata {
    /// ARN is the Amazon Resource Name for the resource. This is a
    /// globally-unique identifier and is set only by the ACK service controller
    /// once the controller has orchestrated the creation of the resource OR
    /// when it has verified that an "adopted" resource (a resource where the
    /// ARN annotation was set by the Kubernetes user on the CR) exists and
    /// matches the supplied CR's Spec field values.
    /// https://github.com/aws/aws-controllers-k8s/issues/270
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// OwnerAccountID is the AWS Account ID of the account that owns the
    /// backend AWS service API resource.
    #[serde(rename = "ownerAccountID")]
    pub owner_account_id: String,
    /// Region is the AWS region in which the resource exists or will exist.
    pub region: String,
}

/// Represents enhanced metrics types.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamStatusEnhancedMonitoring {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "shardLevelMetrics")]
    pub shard_level_metrics: Option<Vec<String>>,
}

