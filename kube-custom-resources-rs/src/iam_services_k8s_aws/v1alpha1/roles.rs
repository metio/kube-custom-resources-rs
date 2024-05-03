// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/iam-controller/iam.services.k8s.aws/v1alpha1/roles.yaml --derive=Default --derive=PartialEq
// kopium version: 0.19.0

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// RoleSpec defines the desired state of Role.
/// 
/// 
/// Contains information about an IAM role. This structure is returned as a response
/// element in several API operations that interact with roles.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "iam.services.k8s.aws", version = "v1alpha1", kind = "Role", plural = "roles")]
#[kube(namespaced)]
#[kube(status = "RoleStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct RoleSpec {
    /// The trust relationship policy document that grants an entity permission to
    /// assume the role.
    /// 
    /// 
    /// In IAM, you must provide a JSON policy that has been converted to a string.
    /// However, for CloudFormation templates formatted in YAML, you can provide
    /// the policy in JSON or YAML format. CloudFormation always converts a YAML
    /// policy to JSON format before submitting it to IAM.
    /// 
    /// 
    /// The regex pattern (http://wikipedia.org/wiki/regex) used to validate this
    /// parameter is a string of characters consisting of the following:
    /// 
    /// 
    ///    * Any printable ASCII character ranging from the space character (\u0020)
    ///    through the end of the ASCII character range
    /// 
    /// 
    ///    * The printable characters in the Basic Latin and Latin-1 Supplement character
    ///    set (through \u00FF)
    /// 
    /// 
    ///    * The special characters tab (\u0009), line feed (\u000A), and carriage
    ///    return (\u000D)
    /// 
    /// 
    /// Upon success, the response includes the same trust policy in JSON format.
    #[serde(rename = "assumeRolePolicyDocument")]
    pub assume_role_policy_document: String,
    /// A description of the role.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inlinePolicies")]
    pub inline_policies: Option<BTreeMap<String, String>>,
    /// The maximum session duration (in seconds) that you want to set for the specified
    /// role. If you do not specify a value for this setting, the default value of
    /// one hour is applied. This setting can have a value from 1 hour to 12 hours.
    /// 
    /// 
    /// Anyone who assumes the role from the CLI or API can use the DurationSeconds
    /// API parameter or the duration-seconds CLI parameter to request a longer session.
    /// The MaxSessionDuration setting determines the maximum duration that can be
    /// requested using the DurationSeconds parameter. If users don't specify a value
    /// for the DurationSeconds parameter, their security credentials are valid for
    /// one hour by default. This applies when you use the AssumeRole* API operations
    /// or the assume-role* CLI operations but does not apply when you use those
    /// operations to create a console URL. For more information, see Using IAM roles
    /// (https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use.html) in the
    /// IAM User Guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxSessionDuration")]
    pub max_session_duration: Option<i64>,
    /// The name of the role to create.
    /// 
    /// 
    /// IAM user, group, role, and policy names must be unique within the account.
    /// Names are not distinguished by case. For example, you cannot create resources
    /// named both "MyResource" and "myresource".
    /// 
    /// 
    /// This parameter allows (through its regex pattern (http://wikipedia.org/wiki/regex))
    /// a string of characters consisting of upper and lowercase alphanumeric characters
    /// with no spaces. You can also include any of the following characters: _+=,.@-
    pub name: String,
    /// The path to the role. For more information about paths, see IAM Identifiers
    /// (https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html)
    /// in the IAM User Guide.
    /// 
    /// 
    /// This parameter is optional. If it is not included, it defaults to a slash
    /// (/).
    /// 
    /// 
    /// This parameter allows (through its regex pattern (http://wikipedia.org/wiki/regex))
    /// a string of characters consisting of either a forward slash (/) by itself
    /// or a string that must begin and end with forward slashes. In addition, it
    /// can contain any ASCII character from the ! (\u0021) through the DEL character
    /// (\u007F), including most punctuation characters, digits, and upper and lowercased
    /// letters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// The ARN of the managed policy that is used to set the permissions boundary
    /// for the role.
    /// 
    /// 
    /// A permissions boundary policy defines the maximum permissions that identity-based
    /// policies can grant to an entity, but does not grant permissions. Permissions
    /// boundaries do not define the maximum permissions that a resource-based policy
    /// can grant to an entity. To learn more, see Permissions boundaries for IAM
    /// entities (https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_boundaries.html)
    /// in the IAM User Guide.
    /// 
    /// 
    /// For more information about policy types, see Policy types (https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#access_policy-types)
    /// in the IAM User Guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "permissionsBoundary")]
    pub permissions_boundary: Option<String>,
    /// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
    /// type to provide more user friendly syntax for references using 'from' field
    /// Ex:
    /// APIIDRef:
    /// 
    /// 
    /// 	from:
    /// 	  name: my-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "permissionsBoundaryRef")]
    pub permissions_boundary_ref: Option<RolePermissionsBoundaryRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "policyRefs")]
    pub policy_refs: Option<Vec<RolePolicyRefs>>,
    /// A list of tags that you want to attach to the new role. Each tag consists
    /// of a key name and an associated value. For more information about tagging,
    /// see Tagging IAM resources (https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html)
    /// in the IAM User Guide.
    /// 
    /// 
    /// If any one of the tags is invalid or if you exceed the allowed maximum number
    /// of tags, then the entire request fails and the resource is not created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RoleTags>>,
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
pub struct RolePermissionsBoundaryRef {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<RolePermissionsBoundaryRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RolePermissionsBoundaryRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
pub struct RolePolicyRefs {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<RolePolicyRefsFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RolePolicyRefsFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// A structure that represents user-provided metadata that can be associated
/// with an IAM resource. For more information about tagging, see Tagging IAM
/// resources (https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html)
/// in the IAM User Guide.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RoleTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// RoleStatus defines the observed state of Role
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RoleStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<RoleStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The date and time, in ISO 8601 date-time format (http://www.iso.org/iso/iso8601),
    /// when the role was created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createDate")]
    pub create_date: Option<String>,
    /// The stable and unique string identifying the role. For more information about
    /// IDs, see IAM identifiers (https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html)
    /// in the IAM User Guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "roleID")]
    pub role_id: Option<String>,
    /// Contains information about the last time that an IAM role was used. This
    /// includes the date and time and the Region in which the role was last used.
    /// Activity is only reported for the trailing 400 days. This period can be shorter
    /// if your Region began supporting these features within the last year. The
    /// role might have been used more than 400 days ago. For more information, see
    /// Regions where data is tracked (https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_access-advisor.html#access-advisor_tracking-period)
    /// in the IAM user Guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "roleLastUsed")]
    pub role_last_used: Option<RoleStatusRoleLastUsed>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RoleStatusAckResourceMetadata {
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

/// Contains information about the last time that an IAM role was used. This
/// includes the date and time and the Region in which the role was last used.
/// Activity is only reported for the trailing 400 days. This period can be shorter
/// if your Region began supporting these features within the last year. The
/// role might have been used more than 400 days ago. For more information, see
/// Regions where data is tracked (https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_access-advisor.html#access-advisor_tracking-period)
/// in the IAM user Guide.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RoleStatusRoleLastUsed {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUsedDate")]
    pub last_used_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

