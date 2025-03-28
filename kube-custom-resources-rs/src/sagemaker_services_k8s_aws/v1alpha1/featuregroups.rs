// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/sagemaker-controller/sagemaker.services.k8s.aws/v1alpha1/featuregroups.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// FeatureGroupSpec defines the desired state of FeatureGroup.
/// 
/// Amazon SageMaker Feature Store stores features in a collection called Feature
/// Group. A Feature Group can be visualized as a table which has rows, with
/// a unique identifier for each row where each column in the table is a feature.
/// In principle, a Feature Group is composed of features and values per features.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "sagemaker.services.k8s.aws", version = "v1alpha1", kind = "FeatureGroup", plural = "featuregroups")]
#[kube(namespaced)]
#[kube(status = "FeatureGroupStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct FeatureGroupSpec {
    /// A free-form description of a FeatureGroup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the feature that stores the EventTime of a Record in a FeatureGroup.
    /// 
    /// An EventTime is a point in time when a new event occurs that corresponds
    /// to the creation or update of a Record in a FeatureGroup. All Records in the
    /// FeatureGroup must have a corresponding EventTime.
    /// 
    /// An EventTime can be a String or Fractional.
    /// 
    ///    * Fractional: EventTime feature values must be a Unix timestamp in seconds.
    /// 
    ///    * String: EventTime feature values must be an ISO-8601 string in the format.
    ///    The following formats are supported yyyy-MM-dd'T'HH:mm:ssZ and yyyy-MM-dd'T'HH:mm:ss.SSSZ
    ///    where yyyy, MM, and dd represent the year, month, and day respectively
    ///    and HH, mm, ss, and if applicable, SSS represent the hour, month, second
    ///    and milliseconds respsectively. 'T' and Z are constants.
    #[serde(rename = "eventTimeFeatureName")]
    pub event_time_feature_name: String,
    /// A list of Feature names and types. Name and Type is compulsory per Feature.
    /// 
    /// Valid feature FeatureTypes are Integral, Fractional and String.
    /// 
    /// FeatureNames cannot be any of the following: is_deleted, write_time, api_invocation_time
    /// 
    /// You can create up to 2,500 FeatureDefinitions per FeatureGroup.
    #[serde(rename = "featureDefinitions")]
    pub feature_definitions: Vec<FeatureGroupFeatureDefinitions>,
    /// The name of the FeatureGroup. The name must be unique within an Amazon Web
    /// Services Region in an Amazon Web Services account.
    /// 
    /// The name:
    /// 
    ///    * Must start with an alphanumeric character.
    /// 
    ///    * Can only include alphanumeric characters, underscores, and hyphens.
    ///    Spaces are not allowed.
    #[serde(rename = "featureGroupName")]
    pub feature_group_name: String,
    /// Use this to configure an OfflineFeatureStore. This parameter allows you to
    /// specify:
    /// 
    ///    * The Amazon Simple Storage Service (Amazon S3) location of an OfflineStore.
    /// 
    ///    * A configuration for an Amazon Web Services Glue or Amazon Web Services
    ///    Hive data catalog.
    /// 
    ///    * An KMS encryption key to encrypt the Amazon S3 location used for OfflineStore.
    ///    If KMS encryption key is not specified, by default we encrypt all data
    ///    at rest using Amazon Web Services KMS key. By defining your bucket-level
    ///    key (https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucket-key.html)
    ///    for SSE, you can reduce Amazon Web Services KMS requests costs by up to
    ///    99 percent.
    /// 
    ///    * Format for the offline store table. Supported formats are Glue (Default)
    ///    and Apache Iceberg (https://iceberg.apache.org/).
    /// 
    /// To learn more about this parameter, see OfflineStoreConfig (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_OfflineStoreConfig.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "offlineStoreConfig")]
    pub offline_store_config: Option<FeatureGroupOfflineStoreConfig>,
    /// You can turn the OnlineStore on or off by specifying True for the EnableOnlineStore
    /// flag in OnlineStoreConfig.
    /// 
    /// You can also include an Amazon Web Services KMS key ID (KMSKeyId) for at-rest
    /// encryption of the OnlineStore.
    /// 
    /// The default value is False.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onlineStoreConfig")]
    pub online_store_config: Option<FeatureGroupOnlineStoreConfig>,
    /// The name of the Feature whose value uniquely identifies a Record defined
    /// in the FeatureStore. Only the latest record per identifier value will be
    /// stored in the OnlineStore. RecordIdentifierFeatureName must be one of feature
    /// definitions' names.
    /// 
    /// You use the RecordIdentifierFeatureName to access data in a FeatureStore.
    /// 
    /// This name:
    /// 
    ///    * Must start with an alphanumeric character.
    /// 
    ///    * Can only contains alphanumeric characters, hyphens, underscores. Spaces
    ///    are not allowed.
    #[serde(rename = "recordIdentifierFeatureName")]
    pub record_identifier_feature_name: String,
    /// The Amazon Resource Name (ARN) of the IAM execution role used to persist
    /// data into the OfflineStore if an OfflineStoreConfig is provided.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "roleARN")]
    pub role_arn: Option<String>,
    /// Tags used to identify Features in each FeatureGroup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<FeatureGroupTags>>,
    /// Used to set feature group throughput configuration. There are two modes:
    /// ON_DEMAND and PROVISIONED. With on-demand mode, you are charged for data
    /// reads and writes that your application performs on your feature group. You
    /// do not need to specify read and write throughput because Feature Store accommodates
    /// your workloads as they ramp up and down. You can switch a feature group to
    /// on-demand only once in a 24 hour period. With provisioned throughput mode,
    /// you specify the read and write capacity per second that you expect your application
    /// to require, and you are billed based on those limits. Exceeding provisioned
    /// throughput will result in your requests being throttled.
    /// 
    /// Note: PROVISIONED throughput mode is supported only for feature groups that
    /// are offline-only, or use the Standard (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_OnlineStoreConfig.html#sagemaker-Type-OnlineStoreConfig-StorageType)
    /// tier online store.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "throughputConfig")]
    pub throughput_config: Option<FeatureGroupThroughputConfig>,
}

/// A list of features. You must include FeatureName and FeatureType. Valid feature
/// FeatureTypes are Integral, Fractional and String.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FeatureGroupFeatureDefinitions {
    /// Configuration for your collection.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "collectionConfig")]
    pub collection_config: Option<FeatureGroupFeatureDefinitionsCollectionConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "collectionType")]
    pub collection_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "featureName")]
    pub feature_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "featureType")]
    pub feature_type: Option<String>,
}

/// Configuration for your collection.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FeatureGroupFeatureDefinitionsCollectionConfig {
    /// Configuration for your vector collection type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vectorConfig")]
    pub vector_config: Option<FeatureGroupFeatureDefinitionsCollectionConfigVectorConfig>,
}

/// Configuration for your vector collection type.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FeatureGroupFeatureDefinitionsCollectionConfigVectorConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension: Option<i64>,
}

/// Use this to configure an OfflineFeatureStore. This parameter allows you to
/// specify:
/// 
///    * The Amazon Simple Storage Service (Amazon S3) location of an OfflineStore.
/// 
///    * A configuration for an Amazon Web Services Glue or Amazon Web Services
///    Hive data catalog.
/// 
///    * An KMS encryption key to encrypt the Amazon S3 location used for OfflineStore.
///    If KMS encryption key is not specified, by default we encrypt all data
///    at rest using Amazon Web Services KMS key. By defining your bucket-level
///    key (https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucket-key.html)
///    for SSE, you can reduce Amazon Web Services KMS requests costs by up to
///    99 percent.
/// 
///    * Format for the offline store table. Supported formats are Glue (Default)
///    and Apache Iceberg (https://iceberg.apache.org/).
/// 
/// To learn more about this parameter, see OfflineStoreConfig (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_OfflineStoreConfig.html).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FeatureGroupOfflineStoreConfig {
    /// The meta data of the Glue table which serves as data catalog for the OfflineStore.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataCatalogConfig")]
    pub data_catalog_config: Option<FeatureGroupOfflineStoreConfigDataCatalogConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disableGlueTableCreation")]
    pub disable_glue_table_creation: Option<bool>,
    /// The Amazon Simple Storage (Amazon S3) location and security configuration
    /// for OfflineStore.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3StorageConfig")]
    pub s3_storage_config: Option<FeatureGroupOfflineStoreConfigS3StorageConfig>,
}

/// The meta data of the Glue table which serves as data catalog for the OfflineStore.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FeatureGroupOfflineStoreConfigDataCatalogConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tableName")]
    pub table_name: Option<String>,
}

/// The Amazon Simple Storage (Amazon S3) location and security configuration
/// for OfflineStore.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FeatureGroupOfflineStoreConfigS3StorageConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyID")]
    pub kms_key_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resolvedOutputS3URI")]
    pub resolved_output_s3uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3URI")]
    pub s3_uri: Option<String>,
}

/// You can turn the OnlineStore on or off by specifying True for the EnableOnlineStore
/// flag in OnlineStoreConfig.
/// 
/// You can also include an Amazon Web Services KMS key ID (KMSKeyId) for at-rest
/// encryption of the OnlineStore.
/// 
/// The default value is False.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FeatureGroupOnlineStoreConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableOnlineStore")]
    pub enable_online_store: Option<bool>,
    /// The security configuration for OnlineStore.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityConfig")]
    pub security_config: Option<FeatureGroupOnlineStoreConfigSecurityConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageType")]
    pub storage_type: Option<String>,
    /// Time to live duration, where the record is hard deleted after the expiration
    /// time is reached; ExpiresAt = EventTime + TtlDuration. For information on
    /// HardDelete, see the DeleteRecord (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_feature_store_DeleteRecord.html)
    /// API in the Amazon SageMaker API Reference guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ttlDuration")]
    pub ttl_duration: Option<FeatureGroupOnlineStoreConfigTtlDuration>,
}

/// The security configuration for OnlineStore.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FeatureGroupOnlineStoreConfigSecurityConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyID")]
    pub kms_key_id: Option<String>,
}

/// Time to live duration, where the record is hard deleted after the expiration
/// time is reached; ExpiresAt = EventTime + TtlDuration. For information on
/// HardDelete, see the DeleteRecord (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_feature_store_DeleteRecord.html)
/// API in the Amazon SageMaker API Reference guide.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FeatureGroupOnlineStoreConfigTtlDuration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

/// A tag object that consists of a key and an optional value, used to manage
/// metadata for SageMaker Amazon Web Services resources.
/// 
/// You can add tags to notebook instances, training jobs, hyperparameter tuning
/// jobs, batch transform jobs, models, labeling jobs, work teams, endpoint configurations,
/// and endpoints. For more information on adding tags to SageMaker resources,
/// see AddTags (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_AddTags.html).
/// 
/// For more information on adding metadata to your Amazon Web Services resources
/// with tagging, see Tagging Amazon Web Services resources (https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html).
/// For advice on best practices for managing Amazon Web Services resources with
/// tagging, see Tagging Best Practices: Implement an Effective Amazon Web Services
/// Resource Tagging Strategy (https://d1.awsstatic.com/whitepapers/aws-tagging-best-practices.pdf).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FeatureGroupTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Used to set feature group throughput configuration. There are two modes:
/// ON_DEMAND and PROVISIONED. With on-demand mode, you are charged for data
/// reads and writes that your application performs on your feature group. You
/// do not need to specify read and write throughput because Feature Store accommodates
/// your workloads as they ramp up and down. You can switch a feature group to
/// on-demand only once in a 24 hour period. With provisioned throughput mode,
/// you specify the read and write capacity per second that you expect your application
/// to require, and you are billed based on those limits. Exceeding provisioned
/// throughput will result in your requests being throttled.
/// 
/// Note: PROVISIONED throughput mode is supported only for feature groups that
/// are offline-only, or use the Standard (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_OnlineStoreConfig.html#sagemaker-Type-OnlineStoreConfig-StorageType)
/// tier online store.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FeatureGroupThroughputConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "provisionedReadCapacityUnits")]
    pub provisioned_read_capacity_units: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "provisionedWriteCapacityUnits")]
    pub provisioned_write_capacity_units: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "throughputMode")]
    pub throughput_mode: Option<String>,
}

/// FeatureGroupStatus defines the observed state of FeatureGroup
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FeatureGroupStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<FeatureGroupStatusAckResourceMetadata>,
    /// All CRs managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The reason that the FeatureGroup failed to be replicated in the OfflineStore.
    /// This is failure can occur because:
    /// 
    ///    * The FeatureGroup could not be created in the OfflineStore.
    /// 
    ///    * The FeatureGroup could not be deleted from the OfflineStore.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureReason")]
    pub failure_reason: Option<String>,
    /// The status of the feature group.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "featureGroupStatus")]
    pub feature_group_status: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FeatureGroupStatusAckResourceMetadata {
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

