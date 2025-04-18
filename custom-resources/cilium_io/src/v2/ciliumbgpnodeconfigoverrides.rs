// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/cilium/cilium/cilium.io/v2/ciliumbgpnodeconfigoverrides.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// Spec is the specification of the desired behavior of the CiliumBGPNodeConfigOverride.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "cilium.io", version = "v2", kind = "CiliumBGPNodeConfigOverride", plural = "ciliumbgpnodeconfigoverrides")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct CiliumBGPNodeConfigOverrideSpec {
    /// BGPInstances is a list of BGP instances to override.
    #[serde(rename = "bgpInstances")]
    pub bgp_instances: Vec<CiliumBGPNodeConfigOverrideBgpInstances>,
}

/// CiliumBGPNodeConfigInstanceOverride defines configuration options which can be overridden for a specific BGP instance.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CiliumBGPNodeConfigOverrideBgpInstances {
    /// LocalASN is the ASN to use for this BGP instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localASN")]
    pub local_asn: Option<i64>,
    /// LocalPort is port to use for this BGP instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localPort")]
    pub local_port: Option<i32>,
    /// Name is the name of the BGP instance for which the configuration is overridden.
    pub name: String,
    /// Peers is a list of peer configurations to override.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peers: Option<Vec<CiliumBGPNodeConfigOverrideBgpInstancesPeers>>,
    /// RouterID is BGP router id to use for this instance. It must be unique across all BGP instances.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routerID")]
    pub router_id: Option<String>,
}

/// CiliumBGPNodeConfigPeerOverride defines configuration options which can be overridden for a specific peer.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CiliumBGPNodeConfigOverrideBgpInstancesPeers {
    /// LocalAddress is the IP address to use for connecting to this peer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localAddress")]
    pub local_address: Option<String>,
    /// LocalPort is source port to use for connecting to this peer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localPort")]
    pub local_port: Option<i32>,
    /// Name is the name of the peer for which the configuration is overridden.
    pub name: String,
}

