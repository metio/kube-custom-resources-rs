// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/ec2-controller/ec2.services.k8s.aws/v1alpha1/vpcendpoints.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// VpcEndpointSpec defines the desired state of VpcEndpoint. 
///  Describes a VPC endpoint.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "ec2.services.k8s.aws", version = "v1alpha1", kind = "VPCEndpoint", plural = "vpcendpoints")]
#[kube(namespaced)]
#[kube(status = "VPCEndpointStatus")]
#[kube(schema = "disabled")]
pub struct VPCEndpointSpec {
    /// The DNS options for the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dnsOptions")]
    pub dns_options: Option<VPCEndpointDnsOptions>,
    /// The IP address type for the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipAddressType")]
    pub ip_address_type: Option<String>,
    /// (Interface and gateway endpoints) A policy to attach to the endpoint that controls access to the service. The policy must be in valid JSON format. If this parameter is not specified, we attach a default policy that allows full access to the service.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "policyDocument")]
    pub policy_document: Option<String>,
    /// (Interface endpoint) Indicates whether to associate a private hosted zone with the specified VPC. The private hosted zone contains a record set for the default public DNS name for the service for the Region (for example, kinesis.us-east-1.amazonaws.com), which resolves to the private IP addresses of the endpoint network interfaces in the VPC. This enables you to make requests to the default public DNS name for the service instead of the public DNS names that are automatically generated by the VPC endpoint service. 
    ///  To use a private hosted zone, you must set the following VPC attributes to true: enableDnsHostnames and enableDnsSupport. Use ModifyVpcAttribute to set the VPC attributes. 
    ///  Default: true
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "privateDNSEnabled")]
    pub private_dns_enabled: Option<bool>,
    /// (Gateway endpoint) One or more route table IDs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routeTableIDs")]
    pub route_table_i_ds: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routeTableRefs")]
    pub route_table_refs: Option<Vec<VPCEndpointRouteTableRefs>>,
    /// (Interface endpoint) The ID of one or more security groups to associate with the endpoint network interface.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityGroupIDs")]
    pub security_group_i_ds: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityGroupRefs")]
    pub security_group_refs: Option<Vec<VPCEndpointSecurityGroupRefs>>,
    /// The service name. To get a list of available services, use the DescribeVpcEndpointServices request, or get the name from the service provider.
    #[serde(rename = "serviceName")]
    pub service_name: String,
    /// (Interface and Gateway Load Balancer endpoints) The ID of one or more subnets in which to create an endpoint network interface. For a Gateway Load Balancer endpoint, you can specify one subnet only.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetIDs")]
    pub subnet_i_ds: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetRefs")]
    pub subnet_refs: Option<Vec<VPCEndpointSubnetRefs>>,
    /// The tags. The value parameter is required, but if you don't want the tag to have a value, specify the parameter with no value, and we set the value to an empty string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<VPCEndpointTags>>,
    /// The type of endpoint. 
    ///  Default: Gateway
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcEndpointType")]
    pub vpc_endpoint_type: Option<String>,
    /// The ID of the VPC in which the endpoint will be used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcID")]
    pub vpc_id: Option<String>,
    /// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference type to provide more user friendly syntax for references using 'from' field Ex: APIIDRef: 
    ///  from: name: my-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcRef")]
    pub vpc_ref: Option<VPCEndpointVpcRef>,
}

/// The DNS options for the endpoint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointDnsOptions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dnsRecordIPType")]
    pub dns_record_ip_type: Option<String>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference type to provide more user friendly syntax for references using 'from' field Ex: APIIDRef: 
///  from: name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointRouteTableRefs {
    /// AWSResourceReference provides all the values necessary to reference another k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<VPCEndpointRouteTableRefsFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointRouteTableRefsFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference type to provide more user friendly syntax for references using 'from' field Ex: APIIDRef: 
///  from: name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointSecurityGroupRefs {
    /// AWSResourceReference provides all the values necessary to reference another k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<VPCEndpointSecurityGroupRefsFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointSecurityGroupRefsFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference type to provide more user friendly syntax for references using 'from' field Ex: APIIDRef: 
///  from: name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointSubnetRefs {
    /// AWSResourceReference provides all the values necessary to reference another k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<VPCEndpointSubnetRefsFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointSubnetRefsFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Describes a tag.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference type to provide more user friendly syntax for references using 'from' field Ex: APIIDRef: 
///  from: name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointVpcRef {
    /// AWSResourceReference provides all the values necessary to reference another k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<VPCEndpointVpcRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointVpcRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// VPCEndpointStatus defines the observed state of VPCEndpoint
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<VPCEndpointStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that contains a collection of `ackv1alpha1.Condition` objects that describe the various terminal states of the CR and its backend AWS service API resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<VPCEndpointStatusConditions>>,
    /// The date and time that the endpoint was created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "creationTimestamp")]
    pub creation_timestamp: Option<String>,
    /// (Interface endpoint) The DNS entries for the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dnsEntries")]
    pub dns_entries: Option<Vec<VPCEndpointStatusDnsEntries>>,
    /// (Interface endpoint) Information about the security groups that are associated with the network interface.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<VPCEndpointStatusGroups>>,
    /// The last error that occurred for endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastError")]
    pub last_error: Option<VPCEndpointStatusLastError>,
    /// (Interface endpoint) One or more network interfaces for the endpoint.
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

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VPCEndpointStatusAckResourceMetadata {
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
pub struct VPCEndpointStatusConditions {
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

