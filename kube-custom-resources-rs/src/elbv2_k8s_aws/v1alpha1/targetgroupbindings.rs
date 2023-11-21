// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/aws-load-balancer-controller/elbv2.k8s.aws/v1alpha1/targetgroupbindings.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// TargetGroupBindingSpec defines the desired state of TargetGroupBinding
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "elbv2.k8s.aws", version = "v1alpha1", kind = "TargetGroupBinding", plural = "targetgroupbindings")]
#[kube(namespaced)]
#[kube(status = "TargetGroupBindingStatus")]
#[kube(schema = "disabled")]
pub struct TargetGroupBindingSpec {
    /// networking provides the networking setup for ELBV2 LoadBalancer to access targets in TargetGroup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub networking: Option<TargetGroupBindingNetworking>,
    /// serviceRef is a reference to a Kubernetes Service and ServicePort.
    #[serde(rename = "serviceRef")]
    pub service_ref: TargetGroupBindingServiceRef,
    /// targetGroupARN is the Amazon Resource Name (ARN) for the TargetGroup.
    #[serde(rename = "targetGroupARN")]
    pub target_group_arn: String,
    /// targetType is the TargetType of TargetGroup. If unspecified, it will be automatically inferred.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetType")]
    pub target_type: Option<TargetGroupBindingTargetType>,
}

/// networking provides the networking setup for ELBV2 LoadBalancer to access targets in TargetGroup.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TargetGroupBindingNetworking {
    /// List of ingress rules to allow ELBV2 LoadBalancer to access targets in TargetGroup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingress: Option<Vec<TargetGroupBindingNetworkingIngress>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TargetGroupBindingNetworkingIngress {
    /// List of peers which should be able to access the targets in TargetGroup. At least one NetworkingPeer should be specified.
    pub from: Vec<TargetGroupBindingNetworkingIngressFrom>,
    /// List of ports which should be made accessible on the targets in TargetGroup. If ports is empty or unspecified, it defaults to all ports with TCP.
    pub ports: Vec<TargetGroupBindingNetworkingIngressPorts>,
}

/// NetworkingPeer defines the source/destination peer for networking rules.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TargetGroupBindingNetworkingIngressFrom {
    /// IPBlock defines an IPBlock peer. If specified, none of the other fields can be set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipBlock")]
    pub ip_block: Option<TargetGroupBindingNetworkingIngressFromIpBlock>,
    /// SecurityGroup defines a SecurityGroup peer. If specified, none of the other fields can be set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityGroup")]
    pub security_group: Option<TargetGroupBindingNetworkingIngressFromSecurityGroup>,
}

/// IPBlock defines an IPBlock peer. If specified, none of the other fields can be set.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TargetGroupBindingNetworkingIngressFromIpBlock {
    /// CIDR is the network CIDR. Both IPV4 or IPV6 CIDR are accepted.
    pub cidr: String,
}

/// SecurityGroup defines a SecurityGroup peer. If specified, none of the other fields can be set.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TargetGroupBindingNetworkingIngressFromSecurityGroup {
    /// GroupID is the EC2 SecurityGroupID.
    #[serde(rename = "groupID")]
    pub group_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TargetGroupBindingNetworkingIngressPorts {
    /// The port which traffic must match. When NodePort endpoints(instance TargetType) is used, this must be a numerical port. When Port endpoints(ip TargetType) is used, this can be either numerical or named port on pods. if port is unspecified, it defaults to all ports.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<IntOrString>,
    /// The protocol which traffic must match. If protocol is unspecified, it defaults to TCP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<TargetGroupBindingNetworkingIngressPortsProtocol>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TargetGroupBindingNetworkingIngressPortsProtocol {
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "UDP")]
    Udp,
}

/// serviceRef is a reference to a Kubernetes Service and ServicePort.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TargetGroupBindingServiceRef {
    /// Name is the name of the Service.
    pub name: String,
    /// Port is the port of the ServicePort.
    pub port: IntOrString,
}

/// TargetGroupBindingSpec defines the desired state of TargetGroupBinding
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TargetGroupBindingTargetType {
    #[serde(rename = "instance")]
    Instance,
    #[serde(rename = "ip")]
    Ip,
}

/// TargetGroupBindingStatus defines the observed state of TargetGroupBinding
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TargetGroupBindingStatus {
    /// The generation observed by the TargetGroupBinding controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
}
