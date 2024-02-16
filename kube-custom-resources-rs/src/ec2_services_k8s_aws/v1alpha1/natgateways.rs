// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/ec2-controller/ec2.services.k8s.aws/v1alpha1/natgateways.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// NatGatewaySpec defines the desired state of NatGateway.
/// 
/// 
/// Describes a NAT gateway.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "ec2.services.k8s.aws", version = "v1alpha1", kind = "NATGateway", plural = "natgateways")]
#[kube(namespaced)]
#[kube(status = "NATGatewayStatus")]
#[kube(schema = "disabled")]
pub struct NATGatewaySpec {
    /// [Public NAT gateways only] The allocation ID of an Elastic IP address to
    /// associate with the NAT gateway. You cannot specify an Elastic IP address
    /// with a private NAT gateway. If the Elastic IP address is associated with
    /// another resource, you must first disassociate it.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allocationID")]
    pub allocation_id: Option<String>,
    /// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
    /// type to provide more user friendly syntax for references using 'from' field
    /// Ex:
    /// APIIDRef:
    /// 
    /// 
    /// 	from:
    /// 	  name: my-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allocationRef")]
    pub allocation_ref: Option<NATGatewayAllocationRef>,
    /// Indicates whether the NAT gateway supports public or private connectivity.
    /// The default is public connectivity.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "connectivityType")]
    pub connectivity_type: Option<String>,
    /// The subnet in which to create the NAT gateway.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetID")]
    pub subnet_id: Option<String>,
    /// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
    /// type to provide more user friendly syntax for references using 'from' field
    /// Ex:
    /// APIIDRef:
    /// 
    /// 
    /// 	from:
    /// 	  name: my-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetRef")]
    pub subnet_ref: Option<NATGatewaySubnetRef>,
    /// The tags. The value parameter is required, but if you don't want the tag
    /// to have a value, specify the parameter with no value, and we set the value
    /// to an empty string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<NATGatewayTags>>,
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
pub struct NATGatewayAllocationRef {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<NATGatewayAllocationRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NATGatewayAllocationRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
pub struct NATGatewaySubnetRef {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<NATGatewaySubnetRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NATGatewaySubnetRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Describes a tag.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NATGatewayTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// NATGatewayStatus defines the observed state of NATGateway
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NATGatewayStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<NATGatewayStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<NATGatewayStatusConditions>>,
    /// The date and time the NAT gateway was created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createTime")]
    pub create_time: Option<String>,
    /// The date and time the NAT gateway was deleted, if applicable.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deleteTime")]
    pub delete_time: Option<String>,
    /// If the NAT gateway could not be created, specifies the error code for the
    /// failure. (InsufficientFreeAddressesInSubnet | Gateway.NotAttached | InvalidAllocationID.NotFound
    /// | Resource.AlreadyAssociated | InternalError | InvalidSubnetID.NotFound)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureCode")]
    pub failure_code: Option<String>,
    /// If the NAT gateway could not be created, specifies the error message for
    /// the failure, that corresponds to the error code.
    /// 
    /// 
    ///    * For InsufficientFreeAddressesInSubnet: "Subnet has insufficient free
    ///    addresses to create this NAT gateway"
    /// 
    /// 
    ///    * For Gateway.NotAttached: "Network vpc-xxxxxxxx has no Internet gateway
    ///    attached"
    /// 
    /// 
    ///    * For InvalidAllocationID.NotFound: "Elastic IP address eipalloc-xxxxxxxx
    ///    could not be associated with this NAT gateway"
    /// 
    /// 
    ///    * For Resource.AlreadyAssociated: "Elastic IP address eipalloc-xxxxxxxx
    ///    is already associated"
    /// 
    /// 
    ///    * For InternalError: "Network interface eni-xxxxxxxx, created and used
    ///    internally by this NAT gateway is in an invalid state. Please try again."
    /// 
    /// 
    ///    * For InvalidSubnetID.NotFound: "The specified subnet subnet-xxxxxxxx
    ///    does not exist or could not be found."
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureMessage")]
    pub failure_message: Option<String>,
    /// Information about the IP addresses and network interface associated with
    /// the NAT gateway.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "natGatewayAddresses")]
    pub nat_gateway_addresses: Option<Vec<NATGatewayStatusNatGatewayAddresses>>,
    /// The ID of the NAT gateway.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "natGatewayID")]
    pub nat_gateway_id: Option<String>,
    /// Reserved. If you need to sustain traffic greater than the documented limits
    /// (https://docs.aws.amazon.com/vpc/latest/userguide/vpc-nat-gateway.html),
    /// contact us through the Support Center (https://console.aws.amazon.com/support/home?).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "provisionedBandwidth")]
    pub provisioned_bandwidth: Option<NATGatewayStatusProvisionedBandwidth>,
    /// The state of the NAT gateway.
    /// 
    /// 
    ///    * pending: The NAT gateway is being created and is not ready to process
    ///    traffic.
    /// 
    /// 
    ///    * failed: The NAT gateway could not be created. Check the failureCode
    ///    and failureMessage fields for the reason.
    /// 
    /// 
    ///    * available: The NAT gateway is able to process traffic. This status remains
    ///    until you delete the NAT gateway, and does not indicate the health of
    ///    the NAT gateway.
    /// 
    /// 
    ///    * deleting: The NAT gateway is in the process of being terminated and
    ///    may still be processing traffic.
    /// 
    /// 
    ///    * deleted: The NAT gateway has been terminated and is no longer processing
    ///    traffic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The ID of the VPC in which the NAT gateway is located.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcID")]
    pub vpc_id: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NATGatewayStatusAckResourceMetadata {
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

/// Condition is the common struct used by all CRDs managed by ACK service
/// controllers to indicate terminal states  of the CR and its backend AWS
/// service API resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NATGatewayStatusConditions {
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

/// Describes the IP addresses and network interface associated with a NAT gateway.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NATGatewayStatusNatGatewayAddresses {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allocationID")]
    pub allocation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "networkInterfaceID")]
    pub network_interface_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "privateIP")]
    pub private_ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "publicIP")]
    pub public_ip: Option<String>,
}

/// Reserved. If you need to sustain traffic greater than the documented limits
/// (https://docs.aws.amazon.com/vpc/latest/userguide/vpc-nat-gateway.html),
/// contact us through the Support Center (https://console.aws.amazon.com/support/home?).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NATGatewayStatusProvisionedBandwidth {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "provisionTime")]
    pub provision_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provisioned: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestTime")]
    pub request_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

