// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/dynamodb-controller/dynamodb.services.k8s.aws/v1alpha1/backups.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// BackupSpec defines the desired state of Backup.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "dynamodb.services.k8s.aws", version = "v1alpha1", kind = "Backup", plural = "backups")]
#[kube(namespaced)]
#[kube(status = "BackupStatus")]
#[kube(schema = "disabled")]
pub struct BackupSpec {
    /// Specified name for the backup.
    #[serde(rename = "backupName")]
    pub backup_name: String,
    /// The name of the table.
    #[serde(rename = "tableName")]
    pub table_name: String,
}

/// BackupStatus defines the observed state of Backup
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<BackupStatusAckResourceMetadata>,
    /// Time at which the backup was created. This is the request time of the backup.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupCreationDateTime")]
    pub backup_creation_date_time: Option<String>,
    /// Time at which the automatic on-demand backup created by DynamoDB will expire. This SYSTEM on-demand backup expires automatically 35 days after its creation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupExpiryDateTime")]
    pub backup_expiry_date_time: Option<String>,
    /// Size of the backup in bytes. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupSizeBytes")]
    pub backup_size_bytes: Option<i64>,
    /// Backup can be in one of the following states: CREATING, ACTIVE, DELETED.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupStatus")]
    pub backup_status: Option<String>,
    /// BackupType: 
    ///  * USER - You create and manage these using the on-demand backup feature. 
    ///  * SYSTEM - If you delete a table with point-in-time recovery enabled, a SYSTEM backup is automatically created and is retained for 35 days (at no additional cost). System backups allow you to restore the deleted table to the state it was in just before the point of deletion. 
    ///  * AWS_BACKUP - On-demand backup created by you from Backup service.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupType")]
    pub backup_type: Option<String>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that contains a collection of `ackv1alpha1.Condition` objects that describe the various terminal states of the CR and its backend AWS service API resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<BackupStatusConditions>>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupStatusAckResourceMetadata {
    /// ARN is the Amazon Resource Name for the resource. This is a globally-unique identifier and is set only by the ACK service controller once the controller has orchestrated the creation of the resource OR when it has verified that an "adopted" resource (a resource where the ARN annotation was set by the Kubernetes user on the CR) exists and matches the supplied CR's Spec field values. TODO(vijat@): Find a better strategy for resources that do not have ARN in CreateOutputResponse https://github.com/aws/aws-controllers-k8s/issues/270
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// OwnerAccountID is the AWS Account ID of the account that owns the backend AWS service API resource.
    #[serde(rename = "ownerAccountID")]
    pub owner_account_id: String,
    /// Region is the AWS region in which the resource exists or will exist.
    pub region: String,
}

/// Condition is the common struct used by all CRDs managed by ACK service controllers to indicate terminal states  of the CR and its backend AWS service API resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupStatusConditions {
    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// A human readable message indicating details about the transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Status of the condition, one of True, False, Unknown.
    pub status: String,
    /// Type is the type of the Condition
    #[serde(rename = "type")]
    pub r#type: String,
}

