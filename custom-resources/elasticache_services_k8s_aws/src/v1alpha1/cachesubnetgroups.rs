// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws-controllers-k8s/elasticache-controller/elasticache.services.k8s.aws/v1alpha1/cachesubnetgroups.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// CacheSubnetGroupSpec defines the desired state of CacheSubnetGroup.
/// 
/// 
/// Represents the output of one of the following operations:
/// 
/// 
///    * CreateCacheSubnetGroup
/// 
/// 
///    * ModifyCacheSubnetGroup
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "elasticache.services.k8s.aws", version = "v1alpha1", kind = "CacheSubnetGroup", plural = "cachesubnetgroups")]
#[kube(namespaced)]
#[kube(status = "CacheSubnetGroupStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct CacheSubnetGroupSpec {
    /// A description for the cache subnet group.
    #[serde(rename = "cacheSubnetGroupDescription")]
    pub cache_subnet_group_description: String,
    /// A name for the cache subnet group. This value is stored as a lowercase string.
    /// 
    /// 
    /// Constraints: Must contain no more than 255 alphanumeric characters or hyphens.
    /// 
    /// 
    /// Example: mysubnetgroup
    #[serde(rename = "cacheSubnetGroupName")]
    pub cache_subnet_group_name: String,
    /// A list of VPC subnet IDs for the cache subnet group.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetIDs")]
    pub subnet_i_ds: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetRefs")]
    pub subnet_refs: Option<Vec<CacheSubnetGroupSubnetRefs>>,
    /// A list of tags to be added to this resource. A tag is a key-value pair. A
    /// tag key must be accompanied by a tag value, although null is accepted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<CacheSubnetGroupTags>>,
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
pub struct CacheSubnetGroupSubnetRefs {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<CacheSubnetGroupSubnetRefsFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CacheSubnetGroupSubnetRefsFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// A tag that can be added to an ElastiCache cluster or replication group. Tags
/// are composed of a Key/Value pair. You can use tags to categorize and track
/// all your ElastiCache resources, with the exception of global replication
/// group. When you add or remove tags on replication groups, those actions will
/// be replicated to all nodes in the replication group. A tag with a null Value
/// is permitted.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CacheSubnetGroupTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// CacheSubnetGroupStatus defines the observed state of CacheSubnetGroup
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CacheSubnetGroupStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<CacheSubnetGroupStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// A list of events. Each element in the list contains detailed information
    /// about one event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<CacheSubnetGroupStatusEvents>>,
    /// A list of subnets associated with the cache subnet group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<CacheSubnetGroupStatusSubnets>>,
    /// The Amazon Virtual Private Cloud identifier (VPC ID) of the cache subnet
    /// group.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcID")]
    pub vpc_id: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CacheSubnetGroupStatusAckResourceMetadata {
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

/// Represents a single occurrence of something interesting within the system.
/// Some examples of events are creating a cluster, adding or removing a cache
/// node, or rebooting a node.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CacheSubnetGroupStatusEvents {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceIdentifier")]
    pub source_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceType")]
    pub source_type: Option<String>,
}

/// Represents the subnet associated with a cluster. This parameter refers to
/// subnets defined in Amazon Virtual Private Cloud (Amazon VPC) and used with
/// ElastiCache.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CacheSubnetGroupStatusSubnets {
    /// Describes an Availability Zone in which the cluster is launched.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetAvailabilityZone")]
    pub subnet_availability_zone: Option<CacheSubnetGroupStatusSubnetsSubnetAvailabilityZone>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetIdentifier")]
    pub subnet_identifier: Option<String>,
    /// The ID of the outpost subnet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetOutpost")]
    pub subnet_outpost: Option<CacheSubnetGroupStatusSubnetsSubnetOutpost>,
}

/// Describes an Availability Zone in which the cluster is launched.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CacheSubnetGroupStatusSubnetsSubnetAvailabilityZone {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// The ID of the outpost subnet.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CacheSubnetGroupStatusSubnetsSubnetOutpost {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetOutpostARN")]
    pub subnet_outpost_arn: Option<String>,
}

