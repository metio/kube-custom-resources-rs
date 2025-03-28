// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/ec2-controller/ec2.services.k8s.aws/v1alpha1/elasticipaddresses.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// ElasticIPAddressSpec defines the desired state of ElasticIPAddress.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "ec2.services.k8s.aws", version = "v1alpha1", kind = "ElasticIPAddress", plural = "elasticipaddresses")]
#[kube(namespaced)]
#[kube(status = "ElasticIPAddressStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ElasticIPAddressSpec {
    /// The Elastic IP address to recover or an IPv4 address from an address pool.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The ID of a customer-owned address pool. Use this parameter to let Amazon
    /// EC2 select an address from the address pool. Alternatively, specify a specific
    /// address from the address pool.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customerOwnedIPv4Pool")]
    pub customer_owned_i_pv4_pool: Option<String>,
    /// A unique set of Availability Zones, Local Zones, or Wavelength Zones from
    /// which Amazon Web Services advertises IP addresses. Use this parameter to
    /// limit the IP address to this location. IP addresses cannot move between network
    /// border groups.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "networkBorderGroup")]
    pub network_border_group: Option<String>,
    /// The ID of an address pool that you own. Use this parameter to let Amazon
    /// EC2 select an address from the address pool. To specify a specific address
    /// from the address pool, use the Address parameter instead.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "publicIPv4Pool")]
    pub public_i_pv4_pool: Option<String>,
    /// The tags. The value parameter is required, but if you don't want the tag
    /// to have a value, specify the parameter with no value, and we set the value
    /// to an empty string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ElasticIPAddressTags>>,
}

/// Describes a tag.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ElasticIPAddressTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// ElasticIPAddressStatus defines the observed state of ElasticIPAddress
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ElasticIPAddressStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<ElasticIPAddressStatusAckResourceMetadata>,
    /// The ID that represents the allocation of the Elastic IP address.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allocationID")]
    pub allocation_id: Option<String>,
    /// The carrier IP address. This option is only available for network interfaces
    /// that reside in a subnet in a Wavelength Zone.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "carrierIP")]
    pub carrier_ip: Option<String>,
    /// All CRs managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The customer-owned IP address.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customerOwnedIP")]
    pub customer_owned_ip: Option<String>,
    /// The Elastic IP address.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "publicIP")]
    pub public_ip: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ElasticIPAddressStatusAckResourceMetadata {
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

