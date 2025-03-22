// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws-controllers-k8s/ec2-controller/ec2.services.k8s.aws/v1alpha1/vpcendpoints.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// VpcEndpointSpec defines the desired state of VpcEndpoint.
/// 
/// Describes a VPC endpoint.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "ec2.services.k8s.aws", version = "v1alpha1", kind = "VPCEndpoint", plural = "vpcendpoints")]
#[kube(namespaced)]
#[kube(status = "VPCEndpointStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct VPCEndpointSpec {
    /// The DNS options for the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dnsOptions")]
    pub dns_options: Option<VPCEndpointDnsOptions>,
    /// The IP address type for the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipAddressType")]
    pub ip_address_type: Option<String>,
    /// (Interface and gateway endpoints) A policy to attach to the endpoint that
    /// controls access to the service. The policy must be in valid JSON format.
    /// If this parameter is not specified, we attach a default policy that allows
    /// full access to the service.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "policyDocument")]
    pub policy_document: Option<String>,
    /// (Interface endpoint) Indicates whether to associate a private hosted zone
    /// with the specified VPC. The private hosted zone contains a record set for
    /// the default public DNS name for the service for the Region (for example,
    /// kinesis.us-east-1.amazonaws.com), which resolves to the private IP addresses
    /// of the endpoint network interfaces in the VPC. This enables you to make requests
    /// to the default public DNS name for the service instead of the public DNS
    /// names that are automatically generated by the VPC endpoint service.
    /// 
    /// To use a private hosted zone, you must set the following VPC attributes to
    /// true: enableDnsHostnames and enableDnsSupport. Use ModifyVpcAttribute to
    /// set the VPC attributes.
    /// 
    /// Default: true
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "privateDNSEnabled")]
    pub private_dns_enabled: Option<bool>,
    /// (Gateway endpoint) The route table IDs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routeTableIDs")]
    pub route_table_i_ds: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routeTableRefs")]
    pub route_table_refs: Option<Vec<VPCEndpointRouteTableRefs>>,
    /// (Interface endpoint) The IDs of the security groups to associate with the
    /// endpoint network interfaces. If this parameter is not specified, we use the
    /// default security group for the VPC.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityGroupIDs")]
    pub security_group_i_ds: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityGroupRefs")]
    pub security_group_refs: Option<Vec<VPCEndpointSecurityGroupRefs>>,
    /// The name of the endpoint service.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceName")]
    pub service_name: Option<String>,
    /// (Interface and Gateway Load Balancer endpoints) The IDs of the subnets in
    /// which to create endpoint network interfaces. For a Gateway Load Balancer
    /// endpoint, you can specify only one subnet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetIDs")]
    pub subnet_i_ds: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetRefs")]
    pub subnet_refs: Option<Vec<VPCEndpointSubnetRefs>>,
    /// The tags. The value parameter is required, but if you don't want the tag
    /// to have a value, specify the parameter with no value, and we set the value
    /// to an empty string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<VPCEndpointTags>>,
    /// The type of endpoint.
    /// 
    /// Default: Gateway
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcEndpointType")]
    pub vpc_endpoint_type: Option<String>,
    /// The ID of the VPC.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcID")]
    pub vpc_id: Option<String>,
    /// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
    /// type to provide more user friendly syntax for references using 'from' field
    /// Ex:
    /// APIIDRef:
    /// 
    /// 	from:
    /// 	  name: my-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcRef")]
    pub vpc_ref: Option<VPCEndpointVpcRef>,
}

/// The DNS options for the endpoint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointDnsOptions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dnsRecordIPType")]
    pub dns_record_ip_type: Option<String>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
/// type to provide more user friendly syntax for references using 'from' field
/// Ex:
/// APIIDRef:
/// 
/// 	from:
/// 	  name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointRouteTableRefs {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<VPCEndpointRouteTableRefsFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointRouteTableRefsFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
/// type to provide more user friendly syntax for references using 'from' field
/// Ex:
/// APIIDRef:
/// 
/// 	from:
/// 	  name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointSecurityGroupRefs {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<VPCEndpointSecurityGroupRefsFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointSecurityGroupRefsFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
/// type to provide more user friendly syntax for references using 'from' field
/// Ex:
/// APIIDRef:
/// 
/// 	from:
/// 	  name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointSubnetRefs {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<VPCEndpointSubnetRefsFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointSubnetRefsFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Describes a tag.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
/// type to provide more user friendly syntax for references using 'from' field
/// Ex:
/// APIIDRef:
/// 
/// 	from:
/// 	  name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointVpcRef {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<VPCEndpointVpcRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointVpcRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// VPCEndpointStatus defines the observed state of VPCEndpoint
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<VPCEndpointStatusAckResourceMetadata>,
    /// All CRs managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The date and time that the endpoint was created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "creationTimestamp")]
    pub creation_timestamp: Option<String>,
    /// (Interface endpoint) The DNS entries for the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dnsEntries")]
    pub dns_entries: Option<Vec<VPCEndpointStatusDnsEntries>>,
    /// (Interface endpoint) Information about the security groups that are associated
    /// with the network interface.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<VPCEndpointStatusGroups>>,
    /// The last error that occurred for endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastError")]
    pub last_error: Option<VPCEndpointStatusLastError>,
    /// (Interface endpoint) The network interfaces for the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "networkInterfaceIDs")]
    pub network_interface_i_ds: Option<Vec<String>>,
    /// The ID of the Amazon Web Services account that owns the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    pub owner_id: Option<String>,
    /// Indicates whether the endpoint is being managed by its service.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requesterManaged")]
    pub requester_managed: Option<bool>,
    /// The state of the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The ID of the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcEndpointID")]
    pub vpc_endpoint_id: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointStatusAckResourceMetadata {
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

/// Describes a DNS entry.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointStatusDnsEntries {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dnsName")]
    pub dns_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostedZoneID")]
    pub hosted_zone_id: Option<String>,
}

/// Describes a security group.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointStatusGroups {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "groupID")]
    pub group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "groupName")]
    pub group_name: Option<String>,
}

/// The last error that occurred for endpoint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointStatusLastError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

