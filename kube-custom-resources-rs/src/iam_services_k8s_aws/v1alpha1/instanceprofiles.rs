// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/iam-controller/iam.services.k8s.aws/v1alpha1/instanceprofiles.yaml --derive=Default --derive=PartialEq
// kopium version: 0.19.0

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// InstanceProfileSpec defines the desired state of InstanceProfile.
/// 
/// 
/// Contains information about an instance profile.
/// 
/// 
/// This data type is used as a response element in the following operations:
/// 
/// 
///    * CreateInstanceProfile
/// 
/// 
///    * GetInstanceProfile
/// 
/// 
///    * ListInstanceProfiles
/// 
/// 
///    * ListInstanceProfilesForRole
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "iam.services.k8s.aws", version = "v1alpha1", kind = "InstanceProfile", plural = "instanceprofiles")]
#[kube(namespaced)]
#[kube(status = "InstanceProfileStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct InstanceProfileSpec {
    /// The name of the instance profile to create.
    /// 
    /// 
    /// This parameter allows (through its regex pattern (http://wikipedia.org/wiki/regex))
    /// a string of characters consisting of upper and lowercase alphanumeric characters
    /// with no spaces. You can also include any of the following characters: _+=,.@-
    pub name: String,
    /// The path to the instance profile. For more information about paths, see IAM
    /// Identifiers (https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html)
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
    /// type to provide more user friendly syntax for references using 'from' field
    /// Ex:
    /// APIIDRef:
    /// 
    /// 
    /// 	from:
    /// 	  name: my-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "roleRef")]
    pub role_ref: Option<InstanceProfileRoleRef>,
    /// A list of tags that you want to attach to the newly created IAM instance
    /// profile. Each tag consists of a key name and an associated value. For more
    /// information about tagging, see Tagging IAM resources (https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html)
    /// in the IAM User Guide.
    /// 
    /// 
    /// If any one of the tags is invalid or if you exceed the allowed maximum number
    /// of tags, then the entire request fails and the resource is not created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<InstanceProfileTags>>,
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
pub struct InstanceProfileRoleRef {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<InstanceProfileRoleRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstanceProfileRoleRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// A structure that represents user-provided metadata that can be associated
/// with an IAM resource. For more information about tagging, see Tagging IAM
/// resources (https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html)
/// in the IAM User Guide.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstanceProfileTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// InstanceProfileStatus defines the observed state of InstanceProfile
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstanceProfileStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<InstanceProfileStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The date when the instance profile was created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createDate")]
    pub create_date: Option<String>,
    /// The stable and unique string identifying the instance profile. For more information
    /// about IDs, see IAM identifiers (https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html)
    /// in the IAM User Guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceProfileID")]
    pub instance_profile_id: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstanceProfileStatusAckResourceMetadata {
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

