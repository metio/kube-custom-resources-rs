// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/elasticache-controller/elasticache.services.k8s.aws/v1alpha1/users.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "elasticache.services.k8s.aws", version = "v1alpha1", kind = "User", plural = "users")]
#[kube(namespaced)]
#[kube(status = "UserStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct UserSpec {
    /// Access permissions string used for this user.
    #[serde(rename = "accessString")]
    pub access_string: String,
    /// The current supported value is Redis.
    pub engine: String,
    /// Indicates a password is not required for this user.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "noPasswordRequired")]
    pub no_password_required: Option<bool>,
    /// Passwords used for this user. You can create up to two passwords for each
    /// user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub passwords: Option<Vec<UserPasswords>>,
    /// A list of tags to be added to this resource. A tag is a key-value pair. A
    /// tag key must be accompanied by a tag value, although null is accepted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<UserTags>>,
    /// The ID of the user.
    #[serde(rename = "userID")]
    pub user_id: String,
    /// The username of the user.
    #[serde(rename = "userName")]
    pub user_name: String,
}

/// SecretKeyReference combines a k8s corev1.SecretReference with a
/// specific key within the referred-to Secret
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserPasswords {
    /// Key is the key within the secret
    pub key: String,
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// A tag that can be added to an ElastiCache cluster or replication group. Tags
/// are composed of a Key/Value pair. You can use tags to categorize and track
/// all your ElastiCache resources, with the exception of global replication
/// group. When you add or remove tags on replication groups, those actions will
/// be replicated to all nodes in the replication group. A tag with a null Value
/// is permitted.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// UserStatus defines the observed state of User
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<UserStatusAckResourceMetadata>,
    /// Denotes whether the user requires a password to authenticate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<UserStatusAuthentication>,
    /// All CRs managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Access permissions string used for this user.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "expandedAccessString")]
    pub expanded_access_string: Option<String>,
    /// Access permissions string used for this user.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastRequestedAccessString")]
    pub last_requested_access_string: Option<String>,
    /// The minimum engine version required, which is Redis OSS 6.0
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minimumEngineVersion")]
    pub minimum_engine_version: Option<String>,
    /// Indicates the user status. Can be "active", "modifying" or "deleting".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Returns a list of the user group IDs the user belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userGroupIDs")]
    pub user_group_i_ds: Option<Vec<String>>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserStatusAckResourceMetadata {
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

/// Denotes whether the user requires a password to authenticate.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserStatusAuthentication {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "passwordCount")]
    pub password_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type_")]
    pub r#type: Option<String>,
}

