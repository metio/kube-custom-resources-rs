// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/iam-controller/iam.services.k8s.aws/v1alpha1/policies.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// PolicySpec defines the desired state of Policy. 
///  Contains information about a managed policy. 
///  This data type is used as a response element in the CreatePolicy, GetPolicy, and ListPolicies operations. 
///  For more information about managed policies, refer to Managed policies and inline policies (https://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-vs-inline.html) in the IAM User Guide.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "iam.services.k8s.aws", version = "v1alpha1", kind = "Policy", plural = "policies")]
#[kube(namespaced)]
#[kube(status = "PolicyStatus")]
#[kube(schema = "disabled")]
pub struct PolicySpec {
    /// A friendly description of the policy. 
    ///  Typically used to store information about the permissions defined in the policy. For example, "Grants access to production DynamoDB tables." 
    ///  The policy description is immutable. After a value is assigned, it cannot be changed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The friendly name of the policy. 
    ///  IAM user, group, role, and policy names must be unique within the account. Names are not distinguished by case. For example, you cannot create resources named both "MyResource" and "myresource".
    pub name: String,
    /// The path for the policy. 
    ///  For more information about paths, see IAM identifiers (https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html) in the IAM User Guide. 
    ///  This parameter is optional. If it is not included, it defaults to a slash (/). 
    ///  This parameter allows (through its regex pattern (http://wikipedia.org/wiki/regex)) a string of characters consisting of either a forward slash (/) by itself or a string that must begin and end with forward slashes. In addition, it can contain any ASCII character from the ! (\u0021) through the DEL character (\u007F), including most punctuation characters, digits, and upper and lowercased letters. 
    ///  You cannot use an asterisk (*) in the path name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// The JSON policy document that you want to use as the content for the new policy. 
    ///  You must provide policies in JSON format in IAM. However, for CloudFormation templates formatted in YAML, you can provide the policy in JSON or YAML format. CloudFormation always converts a YAML policy to JSON format before submitting it to IAM. 
    ///  The maximum length of the policy document that you can pass in this operation, including whitespace, is listed below. To view the maximum character counts of a managed policy with no whitespaces, see IAM and STS character quotas (https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-quotas.html#reference_iam-quotas-entity-length). 
    ///  To learn more about JSON policy grammar, see Grammar of the IAM JSON policy language (https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_grammar.html) in the IAM User Guide. 
    ///  The regex pattern (http://wikipedia.org/wiki/regex) used to validate this parameter is a string of characters consisting of the following: 
    ///  * Any printable ASCII character ranging from the space character (\u0020) through the end of the ASCII character range 
    ///  * The printable characters in the Basic Latin and Latin-1 Supplement character set (through \u00FF) 
    ///  * The special characters tab (\u0009), line feed (\u000A), and carriage return (\u000D)
    #[serde(rename = "policyDocument")]
    pub policy_document: String,
    /// A list of tags that you want to attach to the new IAM customer managed policy. Each tag consists of a key name and an associated value. For more information about tagging, see Tagging IAM resources (https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html) in the IAM User Guide. 
    ///  If any one of the tags is invalid or if you exceed the allowed maximum number of tags, then the entire request fails and the resource is not created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<PolicyTags>>,
}

/// A structure that represents user-provided metadata that can be associated with an IAM resource. For more information about tagging, see Tagging IAM resources (https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html) in the IAM User Guide.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// PolicyStatus defines the observed state of Policy
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<PolicyStatusAckResourceMetadata>,
    /// The number of entities (users, groups, and roles) that the policy is attached to.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attachmentCount")]
    pub attachment_count: Option<i64>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that contains a collection of `ackv1alpha1.Condition` objects that describe the various terminal states of the CR and its backend AWS service API resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<PolicyStatusConditions>>,
    /// The date and time, in ISO 8601 date-time format (http://www.iso.org/iso/iso8601), when the policy was created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createDate")]
    pub create_date: Option<String>,
    /// The identifier for the version of the policy that is set as the default version.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultVersionID")]
    pub default_version_id: Option<String>,
    /// Specifies whether the policy can be attached to an IAM user, group, or role.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isAttachable")]
    pub is_attachable: Option<bool>,
    /// The number of entities (users and roles) for which the policy is used to set the permissions boundary. 
    ///  For more information about permissions boundaries, see Permissions boundaries for IAM identities (https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_boundaries.html) in the IAM User Guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "permissionsBoundaryUsageCount")]
    pub permissions_boundary_usage_count: Option<i64>,
    /// The stable and unique string identifying the policy. 
    ///  For more information about IDs, see IAM identifiers (https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html) in the IAM User Guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "policyID")]
    pub policy_id: Option<String>,
    /// The date and time, in ISO 8601 date-time format (http://www.iso.org/iso/iso8601), when the policy was last updated. 
    ///  When a policy has only one version, this field contains the date and time when the policy was created. When a policy has more than one version, this field contains the date and time when the most recent policy version was created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updateDate")]
    pub update_date: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyStatusAckResourceMetadata {
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
pub struct PolicyStatusConditions {
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

