// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/cilium/cilium/cilium.io/v2alpha1/ciliumendpointslices.yaml --derive=Default --derive=PartialEq
// kopium version: 0.19.0

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// CoreCiliumEndpoint is slim version of status of CiliumEndpoint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CiliumEndpointSliceEndpoints {
    /// EncryptionSpec defines the encryption relevant configuration of a node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<CiliumEndpointSliceEndpointsEncryption>,
    /// IdentityID is the numeric identity of the endpoint
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Name indicate as CiliumEndpoint name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// NamedPorts List of named Layer 4 port and protocol pairs which will be used in Network Policy specs. 
    ///  swagger:model NamedPorts
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "named-ports")]
    pub named_ports: Option<Vec<CiliumEndpointSliceEndpointsNamedPorts>>,
    /// EndpointNetworking is the addressing information of an endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub networking: Option<CiliumEndpointSliceEndpointsNetworking>,
}

/// EncryptionSpec defines the encryption relevant configuration of a node.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CiliumEndpointSliceEndpointsEncryption {
    /// Key is the index to the key to use for encryption or 0 if encryption is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<i64>,
}

/// Port Layer 4 port / protocol pair 
///  swagger:model Port
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CiliumEndpointSliceEndpointsNamedPorts {
    /// Optional layer 4 port name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Layer 4 port number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// Layer 4 protocol Enum: [TCP UDP SCTP ICMP ICMPV6 ANY]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

/// EndpointNetworking is the addressing information of an endpoint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CiliumEndpointSliceEndpointsNetworking {
    /// IP4/6 addresses assigned to this Endpoint
    pub addressing: Vec<CiliumEndpointSliceEndpointsNetworkingAddressing>,
    /// NodeIP is the IP of the node the endpoint is running on. The IP must be reachable between nodes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
}

/// AddressPair is a pair of IPv4 and/or IPv6 address.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CiliumEndpointSliceEndpointsNetworkingAddressing {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<String>,
}

