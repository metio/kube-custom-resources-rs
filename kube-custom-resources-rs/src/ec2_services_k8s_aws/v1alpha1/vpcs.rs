// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/ec2-controller/ec2.services.k8s.aws/v1alpha1/vpcs.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// VpcSpec defines the desired state of Vpc.
/// 
/// Describes a VPC.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "ec2.services.k8s.aws", version = "v1alpha1", kind = "VPC", plural = "vpcs")]
#[kube(namespaced)]
#[kube(status = "VPCStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct VPCSpec {
    /// Requests an Amazon-provided IPv6 CIDR block with a /56 prefix length for
    /// the VPC. You cannot specify the range of IP addresses, or the size of the
    /// CIDR block.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amazonProvidedIPv6CIDRBlock")]
    pub amazon_provided_i_pv6_cidr_block: Option<bool>,
    #[serde(rename = "cidrBlocks")]
    pub cidr_blocks: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disallowSecurityGroupDefaultRules")]
    pub disallow_security_group_default_rules: Option<bool>,
    /// The attribute value. The valid values are true or false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableDNSHostnames")]
    pub enable_dns_hostnames: Option<bool>,
    /// The attribute value. The valid values are true or false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableDNSSupport")]
    pub enable_dns_support: Option<bool>,
    /// The tenancy options for instances launched into the VPC. For default, instances
    /// are launched with shared tenancy by default. You can launch instances with
    /// any tenancy into a shared tenancy VPC. For dedicated, instances are launched
    /// as dedicated tenancy instances by default. You can only launch instances
    /// with a tenancy of dedicated or host into a dedicated tenancy VPC.
    /// 
    /// Important: The host value cannot be used with this parameter. Use the default
    /// or dedicated values only.
    /// 
    /// Default: default
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceTenancy")]
    pub instance_tenancy: Option<String>,
    /// The ID of an IPv4 IPAM pool you want to use for allocating this VPC's CIDR.
    /// For more information, see What is IPAM? (https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html)
    /// in the Amazon VPC IPAM User Guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipv4IPAMPoolID")]
    pub ipv4_ipam_pool_id: Option<String>,
    /// The netmask length of the IPv4 CIDR you want to allocate to this VPC from
    /// an Amazon VPC IP Address Manager (IPAM) pool. For more information about
    /// IPAM, see What is IPAM? (https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html)
    /// in the Amazon VPC IPAM User Guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipv4NetmaskLength")]
    pub ipv4_netmask_length: Option<i64>,
    /// The IPv6 CIDR block from the IPv6 address pool. You must also specify Ipv6Pool
    /// in the request.
    /// 
    /// To let Amazon choose the IPv6 CIDR block for you, omit this parameter.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipv6CIDRBlock")]
    pub ipv6_cidr_block: Option<String>,
    /// The name of the location from which we advertise the IPV6 CIDR block. Use
    /// this parameter to limit the address to this location.
    /// 
    /// You must set AmazonProvidedIpv6CidrBlock to true to use this parameter.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipv6CIDRBlockNetworkBorderGroup")]
    pub ipv6_cidr_block_network_border_group: Option<String>,
    /// The ID of an IPv6 IPAM pool which will be used to allocate this VPC an IPv6
    /// CIDR. IPAM is a VPC feature that you can use to automate your IP address
    /// management workflows including assigning, tracking, troubleshooting, and
    /// auditing IP addresses across Amazon Web Services Regions and accounts throughout
    /// your Amazon Web Services Organization. For more information, see What is
    /// IPAM? (https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html)
    /// in the Amazon VPC IPAM User Guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipv6IPAMPoolID")]
    pub ipv6_ipam_pool_id: Option<String>,
    /// The netmask length of the IPv6 CIDR you want to allocate to this VPC from
    /// an Amazon VPC IP Address Manager (IPAM) pool. For more information about
    /// IPAM, see What is IPAM? (https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html)
    /// in the Amazon VPC IPAM User Guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipv6NetmaskLength")]
    pub ipv6_netmask_length: Option<i64>,
    /// The ID of an IPv6 address pool from which to allocate the IPv6 CIDR block.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipv6Pool")]
    pub ipv6_pool: Option<String>,
    /// The tags. The value parameter is required, but if you don't want the tag
    /// to have a value, specify the parameter with no value, and we set the value
    /// to an empty string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<VPCTags>>,
}

/// Describes a tag.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// VPCStatus defines the observed state of VPC
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<VPCStatusAckResourceMetadata>,
    /// Information about the IPv4 CIDR blocks associated with the VPC.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cidrBlockAssociationSet")]
    pub cidr_block_association_set: Option<Vec<VPCStatusCidrBlockAssociationSet>>,
    /// All CRs managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The ID of the set of DHCP options you've associated with the VPC.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dhcpOptionsID")]
    pub dhcp_options_id: Option<String>,
    /// Information about the IPv6 CIDR blocks associated with the VPC.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipv6CIDRBlockAssociationSet")]
    pub ipv6_cidr_block_association_set: Option<Vec<VPCStatusIpv6CidrBlockAssociationSet>>,
    /// Indicates whether the VPC is the default VPC.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isDefault")]
    pub is_default: Option<bool>,
    /// The ID of the Amazon Web Services account that owns the VPC.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityGroupDefaultRulesExist")]
    pub security_group_default_rules_exist: Option<bool>,
    /// The current state of the VPC.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The ID of the VPC.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcID")]
    pub vpc_id: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCStatusAckResourceMetadata {
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

/// Describes an IPv4 CIDR block associated with a VPC.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCStatusCidrBlockAssociationSet {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "associationID")]
    pub association_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cidrBlock")]
    pub cidr_block: Option<String>,
    /// Describes the state of a CIDR block.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cidrBlockState")]
    pub cidr_block_state: Option<VPCStatusCidrBlockAssociationSetCidrBlockState>,
}

/// Describes the state of a CIDR block.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCStatusCidrBlockAssociationSetCidrBlockState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusMessage")]
    pub status_message: Option<String>,
}

/// Describes an IPv6 CIDR block associated with a VPC.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCStatusIpv6CidrBlockAssociationSet {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "associationID")]
    pub association_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipv6CIDRBlock")]
    pub ipv6_cidr_block: Option<String>,
    /// Describes the state of a CIDR block.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipv6CIDRBlockState")]
    pub ipv6_cidr_block_state: Option<VPCStatusIpv6CidrBlockAssociationSetIpv6CidrBlockState>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipv6Pool")]
    pub ipv6_pool: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "networkBorderGroup")]
    pub network_border_group: Option<String>,
}

/// Describes the state of a CIDR block.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCStatusIpv6CidrBlockAssociationSetIpv6CidrBlockState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusMessage")]
    pub status_message: Option<String>,
}

