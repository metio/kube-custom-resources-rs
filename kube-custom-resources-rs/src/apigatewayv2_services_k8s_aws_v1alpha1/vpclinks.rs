// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/apigatewayv2-controller/apigatewayv2.services.k8s.aws/v1alpha1/vpclinks.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// VpcLinkSpec defines the desired state of VpcLink. 
///  Represents a VPC link.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "apigatewayv2.services.k8s.aws", version = "v1alpha1", kind = "VPCLink", plural = "vpclinks")]
#[kube(namespaced)]
#[kube(status = "VPCLinkStatus")]
#[kube(schema = "disabled")]
pub struct VPCLinkSpec {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityGroupIDs")]
    pub security_group_i_ds: Option<Vec<String>>,
    #[serde(rename = "subnetIDs")]
    pub subnet_i_ds: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

/// VPCLinkStatus defines the observed state of VPCLink
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCLinkStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<VPCLinkStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that contains a collection of `ackv1alpha1.Condition` objects that describe the various terminal states of the CR and its backend AWS service API resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<VPCLinkStatusConditions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createdDate")]
    pub created_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcLinkID")]
    pub vpc_link_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcLinkStatus")]
    pub vpc_link_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcLinkStatusMessage")]
    pub vpc_link_status_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcLinkVersion")]
    pub vpc_link_version: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCLinkStatusAckResourceMetadata {
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
pub struct VPCLinkStatusConditions {
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

