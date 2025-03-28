// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/sfn-controller/sfn.services.k8s.aws/v1alpha1/activities.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// ActivitySpec defines the desired state of Activity.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "sfn.services.k8s.aws", version = "v1alpha1", kind = "Activity", plural = "activities")]
#[kube(namespaced)]
#[kube(status = "ActivityStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ActivitySpec {
    /// The name of the activity to create. This name must be unique for your Amazon
    /// Web Services account and region for 90 days. For more information, see Limits
    /// Related to State Machine Executions (https://docs.aws.amazon.com/step-functions/latest/dg/limits.html#service-limits-state-machine-executions)
    /// in the Step Functions Developer Guide.
    /// 
    /// A name must not contain:
    /// 
    ///    * white space
    pub name: String,
    /// The list of tags to add to a resource.
    /// 
    /// An array of key-value pairs. For more information, see Using Cost Allocation
    /// Tags (https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html)
    /// in the Amazon Web Services Billing and Cost Management User Guide, and Controlling
    /// Access Using IAM Tags (https://docs.aws.amazon.com/IAM/latest/UserGuide/access_iam-tags.html).
    /// 
    /// Tags may only contain Unicode letters, digits, white space, or these symbols:
    /// _ . : / = + - @.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ActivityTags>>,
}

/// Tags are key-value pairs that can be associated with Step Functions state
/// machines and activities.
/// 
/// An array of key-value pairs. For more information, see Using Cost Allocation
/// Tags (https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html)
/// in the Amazon Web Services Billing and Cost Management User Guide, and Controlling
/// Access Using IAM Tags (https://docs.aws.amazon.com/IAM/latest/UserGuide/access_iam-tags.html).
/// 
/// Tags may only contain Unicode letters, digits, white space, or these symbols:
/// _ . : / = + - @.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActivityTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// ActivityStatus defines the observed state of Activity
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActivityStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<ActivityStatusAckResourceMetadata>,
    /// All CRs managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The date the activity is created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "creationDate")]
    pub creation_date: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActivityStatusAckResourceMetadata {
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

