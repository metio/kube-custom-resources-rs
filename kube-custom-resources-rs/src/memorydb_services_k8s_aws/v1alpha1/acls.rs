// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/memorydb-controller/memorydb.services.k8s.aws/v1alpha1/acls.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// ACLSpec defines the desired state of ACL. 
///  An Access Control List. You can authenticate users with Access Contol Lists. ACLs enable you to control cluster access by grouping users. These Access control lists are designed as a way to organize access to clusters.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "memorydb.services.k8s.aws", version = "v1alpha1", kind = "ACL", plural = "acls")]
#[kube(namespaced)]
#[kube(status = "ACLStatus")]
#[kube(schema = "disabled")]
pub struct ACLSpec {
    /// The name of the Access Control List.
    pub name: String,
    /// A list of tags to be added to this resource. A tag is a key-value pair. A tag key must be accompanied by a tag value, although null is accepted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ACLTags>>,
    /// The list of users that belong to the Access Control List.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userNames")]
    pub user_names: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userRefs")]
    pub user_refs: Option<Vec<ACLUserRefs>>,
}

/// A tag that can be added to an MemoryDB resource. Tags are composed of a Key/Value pair. You can use tags to categorize and track all your MemoryDB resources. When you add or remove tags on clusters, those actions will be replicated to all nodes in the cluster. A tag with a null Value is permitted. For more information, see Tagging your MemoryDB resources (https://docs.aws.amazon.com/MemoryDB/latest/devguide/tagging-resources.html)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ACLTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference type to provide more user friendly syntax for references using 'from' field Ex: APIIDRef: 
///  from: name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ACLUserRefs {
    /// AWSResourceReference provides all the values necessary to reference another k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<ACLUserRefsFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ACLUserRefsFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// ACLStatus defines the observed state of ACL
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ACLStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<ACLStatusAckResourceMetadata>,
    /// A list of clusters associated with the ACL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<String>>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that contains a collection of `ackv1alpha1.Condition` objects that describe the various terminal states of the CR and its backend AWS service API resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ACLStatusConditions>>,
    /// A list of events. Each element in the list contains detailed information about one event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<ACLStatusEvents>>,
    /// The minimum engine version supported for the ACL
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minimumEngineVersion")]
    pub minimum_engine_version: Option<String>,
    /// A list of updates being applied to the ACL.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pendingChanges")]
    pub pending_changes: Option<ACLStatusPendingChanges>,
    /// Indicates ACL status. Can be "creating", "active", "modifying", "deleting".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ACLStatusAckResourceMetadata {
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
pub struct ACLStatusConditions {
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

/// Represents a single occurrence of something interesting within the system. Some examples of events are creating a cluster or adding or removing a node.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ACLStatusEvents {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceName")]
    pub source_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceType")]
    pub source_type: Option<String>,
}

/// A list of updates being applied to the ACL.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ACLStatusPendingChanges {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userNamesToAdd")]
    pub user_names_to_add: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userNamesToRemove")]
    pub user_names_to_remove: Option<Vec<String>>,
}
