// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws-controllers-k8s/prometheusservice-controller/prometheusservice.services.k8s.aws/v1alpha1/rulegroupsnamespaces.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// RuleGroupsNamespaceSpec defines the desired state of RuleGroupsNamespace.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "prometheusservice.services.k8s.aws", version = "v1alpha1", kind = "RuleGroupsNamespace", plural = "rulegroupsnamespaces")]
#[kube(namespaced)]
#[kube(status = "RuleGroupsNamespaceStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct RuleGroupsNamespaceSpec {
    pub configuration: String,
    /// The rule groups namespace name.
    pub name: String,
    /// Optional, user-provided tags for this rule groups namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
    /// The ID of the workspace in which to create the rule group namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workspaceID")]
    pub workspace_id: Option<String>,
    /// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
    /// type to provide more user friendly syntax for references using 'from' field
    /// Ex:
    /// APIIDRef:
    /// 
    /// 	from:
    /// 	  name: my-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workspaceRef")]
    pub workspace_ref: Option<RuleGroupsNamespaceWorkspaceRef>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
/// type to provide more user friendly syntax for references using 'from' field
/// Ex:
/// APIIDRef:
/// 
/// 	from:
/// 	  name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupsNamespaceWorkspaceRef {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<RuleGroupsNamespaceWorkspaceRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupsNamespaceWorkspaceRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// RuleGroupsNamespaceStatus defines the observed state of RuleGroupsNamespace
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupsNamespaceStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<RuleGroupsNamespaceStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The status of rule groups namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RuleGroupsNamespaceStatusStatus>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupsNamespaceStatusAckResourceMetadata {
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

/// The status of rule groups namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupsNamespaceStatusStatus {
    /// State of a namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusCode")]
    pub status_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusReason")]
    pub status_reason: Option<String>,
}

