// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/documentdb-controller/documentdb.services.k8s.aws/v1alpha1/dbinstances.yaml --derive=Default --derive=PartialEq
// kopium version: 0.19.0

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// DBInstanceSpec defines the desired state of DBInstance.
/// 
/// 
/// Detailed information about an instance.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "documentdb.services.k8s.aws", version = "v1alpha1", kind = "DBInstance", plural = "dbinstances")]
#[kube(namespaced)]
#[kube(status = "DBInstanceStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DBInstanceSpec {
    /// This parameter does not apply to Amazon DocumentDB. Amazon DocumentDB does
    /// not perform minor version upgrades regardless of the value set.
    /// 
    /// 
    /// Default: false
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoMinorVersionUpgrade")]
    pub auto_minor_version_upgrade: Option<bool>,
    /// The Amazon EC2 Availability Zone that the instance is created in.
    /// 
    /// 
    /// Default: A random, system-chosen Availability Zone in the endpoint's Amazon
    /// Web Services Region.
    /// 
    /// 
    /// Example: us-east-1d
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "availabilityZone")]
    pub availability_zone: Option<String>,
    /// The CA certificate identifier to use for the DB instance's server certificate.
    /// 
    /// 
    /// For more information, see Updating Your Amazon DocumentDB TLS Certificates
    /// (https://docs.aws.amazon.com/documentdb/latest/developerguide/ca_cert_rotation.html)
    /// and Encrypting Data in Transit (https://docs.aws.amazon.com/documentdb/latest/developerguide/security.encryption.ssl.html)
    /// in the Amazon DocumentDB Developer Guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caCertificateIdentifier")]
    pub ca_certificate_identifier: Option<String>,
    /// A value that indicates whether to copy tags from the DB instance to snapshots
    /// of the DB instance. By default, tags are not copied.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "copyTagsToSnapshot")]
    pub copy_tags_to_snapshot: Option<bool>,
    /// The identifier of the cluster that the instance will belong to.
    #[serde(rename = "dbClusterIdentifier")]
    pub db_cluster_identifier: String,
    /// The compute and memory capacity of the instance; for example, db.r5.large.
    #[serde(rename = "dbInstanceClass")]
    pub db_instance_class: String,
    /// The instance identifier. This parameter is stored as a lowercase string.
    /// 
    /// 
    /// Constraints:
    /// 
    /// 
    ///    * Must contain from 1 to 63 letters, numbers, or hyphens.
    /// 
    /// 
    ///    * The first character must be a letter.
    /// 
    /// 
    ///    * Cannot end with a hyphen or contain two consecutive hyphens.
    /// 
    /// 
    /// Example: mydbinstance
    #[serde(rename = "dbInstanceIdentifier")]
    pub db_instance_identifier: String,
    /// The name of the database engine to be used for this instance.
    /// 
    /// 
    /// Valid value: docdb
    pub engine: String,
    /// A value that indicates whether to enable Performance Insights for the DB
    /// Instance. For more information, see Using Amazon Performance Insights (https://docs.aws.amazon.com/documentdb/latest/developerguide/performance-insights.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "performanceInsightsEnabled")]
    pub performance_insights_enabled: Option<bool>,
    /// The KMS key identifier for encryption of Performance Insights data.
    /// 
    /// 
    /// The KMS key identifier is the key ARN, key ID, alias ARN, or alias name for
    /// the KMS key.
    /// 
    /// 
    /// If you do not specify a value for PerformanceInsightsKMSKeyId, then Amazon
    /// DocumentDB uses your default KMS key. There is a default KMS key for your
    /// Amazon Web Services account. Your Amazon Web Services account has a different
    /// default KMS key for each Amazon Web Services region.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "performanceInsightsKMSKeyID")]
    pub performance_insights_kms_key_id: Option<String>,
    /// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
    /// type to provide more user friendly syntax for references using 'from' field
    /// Ex:
    /// APIIDRef:
    /// 
    /// 
    /// 	from:
    /// 	  name: my-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "performanceInsightsKMSKeyRef")]
    pub performance_insights_kms_key_ref: Option<DBInstancePerformanceInsightsKmsKeyRef>,
    /// The time range each week during which system maintenance can occur, in Universal
    /// Coordinated Time (UTC).
    /// 
    /// 
    /// Format: ddd:hh24:mi-ddd:hh24:mi
    /// 
    /// 
    /// The default is a 30-minute window selected at random from an 8-hour block
    /// of time for each Amazon Web Services Region, occurring on a random day of
    /// the week.
    /// 
    /// 
    /// Valid days: Mon, Tue, Wed, Thu, Fri, Sat, Sun
    /// 
    /// 
    /// Constraints: Minimum 30-minute window.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredMaintenanceWindow")]
    pub preferred_maintenance_window: Option<String>,
    /// A value that specifies the order in which an Amazon DocumentDB replica is
    /// promoted to the primary instance after a failure of the existing primary
    /// instance.
    /// 
    /// 
    /// Default: 1
    /// 
    /// 
    /// Valid values: 0-15
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "promotionTier")]
    pub promotion_tier: Option<i64>,
    /// The tags to be assigned to the instance. You can assign up to 10 tags to
    /// an instance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DBInstanceTags>>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
/// type to provide more user friendly syntax for references using 'from' field
/// Ex:
/// APIIDRef:
/// 
/// 
/// 	from:
/// 	  name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBInstancePerformanceInsightsKmsKeyRef {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<DBInstancePerformanceInsightsKmsKeyRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBInstancePerformanceInsightsKmsKeyRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Metadata assigned to an Amazon DocumentDB resource consisting of a key-value
/// pair.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBInstanceTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// DBInstanceStatus defines the observed state of DBInstance
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBInstanceStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<DBInstanceStatusAckResourceMetadata>,
    /// Specifies the number of days for which automatic snapshots are retained.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupRetentionPeriod")]
    pub backup_retention_period: Option<i64>,
    /// The details of the DB instance's server certificate.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certificateDetails")]
    pub certificate_details: Option<DBInstanceStatusCertificateDetails>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Specifies the current state of this database.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dbInstanceStatus")]
    pub db_instance_status: Option<String>,
    /// Specifies information on the subnet group that is associated with the instance,
    /// including the name, description, and subnets in the subnet group.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dbSubnetGroup")]
    pub db_subnet_group: Option<DBInstanceStatusDbSubnetGroup>,
    /// The Amazon Web Services Region-unique, immutable identifier for the instance.
    /// This identifier is found in CloudTrail log entries whenever the KMS key for
    /// the instance is accessed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dbiResourceID")]
    pub dbi_resource_id: Option<String>,
    /// A list of log types that this instance is configured to export to CloudWatch
    /// Logs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enabledCloudwatchLogsExports")]
    pub enabled_cloudwatch_logs_exports: Option<Vec<String>>,
    /// Specifies the connection endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<DBInstanceStatusEndpoint>,
    /// Indicates the database engine version.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "engineVersion")]
    pub engine_version: Option<String>,
    /// Provides the date and time that the instance was created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceCreateTime")]
    pub instance_create_time: Option<String>,
    /// If StorageEncrypted is true, the KMS key identifier for the encrypted instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyID")]
    pub kms_key_id: Option<String>,
    /// Specifies the latest time to which a database can be restored with point-in-time
    /// restore.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "latestRestorableTime")]
    pub latest_restorable_time: Option<String>,
    /// Specifies that changes to the instance are pending. This element is included
    /// only when changes are pending. Specific changes are identified by subelements.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pendingModifiedValues")]
    pub pending_modified_values: Option<DBInstanceStatusPendingModifiedValues>,
    /// Specifies the daily time range during which automated backups are created
    /// if automated backups are enabled, as determined by the BackupRetentionPeriod.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredBackupWindow")]
    pub preferred_backup_window: Option<String>,
    /// Not supported. Amazon DocumentDB does not currently support public endpoints.
    /// The value of PubliclyAccessible is always false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "publiclyAccessible")]
    pub publicly_accessible: Option<bool>,
    /// The status of a read replica. If the instance is not a read replica, this
    /// is blank.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusInfos")]
    pub status_infos: Option<Vec<DBInstanceStatusStatusInfos>>,
    /// Specifies whether or not the instance is encrypted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageEncrypted")]
    pub storage_encrypted: Option<bool>,
    /// Provides a list of VPC security group elements that the instance belongs
    /// to.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcSecurityGroups")]
    pub vpc_security_groups: Option<Vec<DBInstanceStatusVpcSecurityGroups>>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBInstanceStatusAckResourceMetadata {
    /// ARN is the Amazon Resource Name for the resource. This is a
    /// globally-unique identifier and is set only by the ACK service controller
    /// once the controller has orchestrated the creation of the resource OR
    /// when it has verified that an "adopted" resource (a resource where the
    /// ARN annotation was set by the Kubernetes user on the CR) exists and
    /// matches the supplied CR's Spec field values.
    /// TODO(vijat@): Find a better strategy for resources that do not have ARN in CreateOutputResponse
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

/// The details of the DB instance's server certificate.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBInstanceStatusCertificateDetails {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cAIdentifier")]
    pub c_a_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "validTill")]
    pub valid_till: Option<String>,
}

/// Specifies information on the subnet group that is associated with the instance,
/// including the name, description, and subnets in the subnet group.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBInstanceStatusDbSubnetGroup {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dbSubnetGroupARN")]
    pub db_subnet_group_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dbSubnetGroupDescription")]
    pub db_subnet_group_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dbSubnetGroupName")]
    pub db_subnet_group_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetGroupStatus")]
    pub subnet_group_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<DBInstanceStatusDbSubnetGroupSubnets>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcID")]
    pub vpc_id: Option<String>,
}

/// Detailed information about a subnet.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBInstanceStatusDbSubnetGroupSubnets {
    /// Information about an Availability Zone.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetAvailabilityZone")]
    pub subnet_availability_zone: Option<DBInstanceStatusDbSubnetGroupSubnetsSubnetAvailabilityZone>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetIdentifier")]
    pub subnet_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetStatus")]
    pub subnet_status: Option<String>,
}

/// Information about an Availability Zone.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBInstanceStatusDbSubnetGroupSubnetsSubnetAvailabilityZone {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Specifies the connection endpoint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBInstanceStatusEndpoint {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostedZoneID")]
    pub hosted_zone_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}

/// Specifies that changes to the instance are pending. This element is included
/// only when changes are pending. Specific changes are identified by subelements.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBInstanceStatusPendingModifiedValues {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allocatedStorage")]
    pub allocated_storage: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupRetentionPeriod")]
    pub backup_retention_period: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caCertificateIdentifier")]
    pub ca_certificate_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dbInstanceClass")]
    pub db_instance_class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dbInstanceIdentifier")]
    pub db_instance_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dbSubnetGroupName")]
    pub db_subnet_group_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "engineVersion")]
    pub engine_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iops: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "licenseModel")]
    pub license_model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "masterUserPassword")]
    pub master_user_password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "multiAZ")]
    pub multi_az: Option<bool>,
    /// A list of the log types whose configuration is still pending. These log types
    /// are in the process of being activated or deactivated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pendingCloudwatchLogsExports")]
    pub pending_cloudwatch_logs_exports: Option<DBInstanceStatusPendingModifiedValuesPendingCloudwatchLogsExports>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageType")]
    pub storage_type: Option<String>,
}

/// A list of the log types whose configuration is still pending. These log types
/// are in the process of being activated or deactivated.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBInstanceStatusPendingModifiedValuesPendingCloudwatchLogsExports {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logTypesToDisable")]
    pub log_types_to_disable: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logTypesToEnable")]
    pub log_types_to_enable: Option<Vec<String>>,
}

/// Provides a list of status information for an instance.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBInstanceStatusStatusInfos {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normal: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusType")]
    pub status_type: Option<String>,
}

/// Used as a response element for queries on virtual private cloud (VPC) security
/// group membership.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBInstanceStatusVpcSecurityGroups {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcSecurityGroupID")]
    pub vpc_security_group_id: Option<String>,
}

