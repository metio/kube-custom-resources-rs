// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/sagemaker-controller/sagemaker.services.k8s.aws/v1alpha1/modelpackagegroups.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// ModelPackageGroupSpec defines the desired state of ModelPackageGroup. 
///  A group of versioned models in the model registry.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "sagemaker.services.k8s.aws", version = "v1alpha1", kind = "ModelPackageGroup", plural = "modelpackagegroups")]
#[kube(namespaced)]
#[kube(status = "ModelPackageGroupStatus")]
#[kube(schema = "disabled")]
pub struct ModelPackageGroupSpec {
    /// A description for the model group.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "modelPackageGroupDescription")]
    pub model_package_group_description: Option<String>,
    /// The name of the model group.
    #[serde(rename = "modelPackageGroupName")]
    pub model_package_group_name: String,
    /// A list of key value pairs associated with the model group. For more information, see Tagging Amazon Web Services resources (https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html) in the Amazon Web Services General Reference Guide.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ModelPackageGroupTags>>,
}

/// A tag object that consists of a key and an optional value, used to manage metadata for SageMaker Amazon Web Services resources. 
///  You can add tags to notebook instances, training jobs, hyperparameter tuning jobs, batch transform jobs, models, labeling jobs, work teams, endpoint configurations, and endpoints. For more information on adding tags to SageMaker resources, see AddTags. 
///  For more information on adding metadata to your Amazon Web Services resources with tagging, see Tagging Amazon Web Services resources (https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html). For advice on best practices for managing Amazon Web Services resources with tagging, see Tagging Best Practices: Implement an Effective Amazon Web Services Resource Tagging Strategy (https://d1.awsstatic.com/whitepapers/aws-tagging-best-practices.pdf).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ModelPackageGroupTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// ModelPackageGroupStatus defines the observed state of ModelPackageGroup
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ModelPackageGroupStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<ModelPackageGroupStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that contains a collection of `ackv1alpha1.Condition` objects that describe the various terminal states of the CR and its backend AWS service API resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ModelPackageGroupStatusConditions>>,
    /// The status of the model group.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "modelPackageGroupStatus")]
    pub model_package_group_status: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ModelPackageGroupStatusAckResourceMetadata {
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
pub struct ModelPackageGroupStatusConditions {
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
