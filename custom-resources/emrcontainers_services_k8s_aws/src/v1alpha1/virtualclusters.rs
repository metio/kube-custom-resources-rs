// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws-controllers-k8s/emrcontainers-controller/emrcontainers.services.k8s.aws/v1alpha1/virtualclusters.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// VirtualClusterSpec defines the desired state of VirtualCluster.
/// 
/// This entity describes a virtual cluster. A virtual cluster is a Kubernetes
/// namespace that Amazon EMR is registered with. Amazon EMR uses virtual clusters
/// to run jobs and host endpoints. Multiple virtual clusters can be backed by
/// the same physical cluster. However, each virtual cluster maps to one namespace
/// on an EKS cluster. Virtual clusters do not create any active resources that
/// contribute to your bill or that require lifecycle management outside the
/// service.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "emrcontainers.services.k8s.aws", version = "v1alpha1", kind = "VirtualCluster", plural = "virtualclusters")]
#[kube(namespaced)]
#[kube(status = "VirtualClusterStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct VirtualClusterSpec {
    /// The container provider of the virtual cluster.
    #[serde(rename = "containerProvider")]
    pub container_provider: VirtualClusterContainerProvider,
    /// The specified name of the virtual cluster.
    pub name: String,
    /// The tags assigned to the virtual cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

/// The container provider of the virtual cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualClusterContainerProvider {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The information about the container used for a job run or a managed endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<VirtualClusterContainerProviderInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type_")]
    pub r#type: Option<String>,
}

/// The information about the container used for a job run or a managed endpoint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualClusterContainerProviderInfo {
    /// The information about the EKS cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "eksInfo")]
    pub eks_info: Option<VirtualClusterContainerProviderInfoEksInfo>,
}

/// The information about the EKS cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualClusterContainerProviderInfoEksInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// VirtualClusterStatus defines the observed state of VirtualCluster
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualClusterStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<VirtualClusterStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// This output contains the virtual cluster ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualClusterStatusAckResourceMetadata {
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

