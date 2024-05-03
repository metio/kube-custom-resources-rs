// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/projectcalico/calico/crd.projectcalico.org/v1/bgppeers.yaml --derive=Default --derive=PartialEq
// kopium version: 0.19.0

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// BGPPeerSpec contains the specification for a BGPPeer resource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "crd.projectcalico.org", version = "v1", kind = "BGPPeer", plural = "bgppeers")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct BGPPeerSpec {
    /// The AS Number of the peer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "asNumber")]
    pub as_number: Option<i32>,
    /// The ordered set of BGPFilters applied on this BGP peer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<String>>,
    /// Option to keep the original nexthop field when routes are sent to a BGP Peer. Setting "true" configures the selected BGP Peers node to use the "next hop keep;" instead of "next hop self;"(default) in the specific branch of the Node on "bird.cfg".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keepOriginalNextHop")]
    pub keep_original_next_hop: Option<bool>,
    /// Time to allow for software restart.  When specified, this is configured as the graceful restart timeout.  When not specified, the BIRD default of 120s is used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxRestartTime")]
    pub max_restart_time: Option<String>,
    /// The node name identifying the Calico node instance that is targeted by this peer. If this is not set, and no nodeSelector is specified, then this BGP peer selects all nodes in the cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
    /// Selector for the nodes that should have this peering.  When this is set, the Node field must be empty.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<String>,
    /// Maximum number of local AS numbers that are allowed in the AS path for received routes. This removes BGP loop prevention and should only be used if absolutely necessary.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "numAllowedLocalASNumbers")]
    pub num_allowed_local_as_numbers: Option<i32>,
    /// Optional BGP password for the peerings generated by this BGPPeer resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<BGPPeerPassword>,
    /// The IP address of the peer followed by an optional port number to peer with. If port number is given, format should be `[<IPv6>]:port` or `<IPv4>:<port>` for IPv4. If optional port number is not set, and this peer IP and ASNumber belongs to a calico/node with ListenPort set in BGPConfiguration, then we use that port to peer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "peerIP")]
    pub peer_ip: Option<String>,
    /// Selector for the remote nodes to peer with.  When this is set, the PeerIP and ASNumber fields must be empty.  For each peering between the local node and selected remote nodes, we configure an IPv4 peering if both ends have NodeBGPSpec.IPv4Address specified, and an IPv6 peering if both ends have NodeBGPSpec.IPv6Address specified.  The remote AS number comes from the remote node's NodeBGPSpec.ASNumber, or the global default if that is not set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "peerSelector")]
    pub peer_selector: Option<String>,
    /// Add an exact, i.e. /32, static route toward peer IP in order to prevent route flapping. ReachableBy contains the address of the gateway which peer can be reached by.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reachableBy")]
    pub reachable_by: Option<String>,
    /// Specifies whether and how to configure a source address for the peerings generated by this BGPPeer resource.  Default value "UseNodeIP" means to configure the node IP as the source address.  "None" means not to configure a source address.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceAddress")]
    pub source_address: Option<String>,
    /// TTLSecurity enables the generalized TTL security mechanism (GTSM) which protects against spoofed packets by ignoring received packets with a smaller than expected TTL value. The provided value is the number of hops (edges) between the peers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ttlSecurity")]
    pub ttl_security: Option<i64>,
}

/// Optional BGP password for the peerings generated by this BGPPeer resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BGPPeerPassword {
    /// Selects a key of a secret in the node pod's namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<BGPPeerPasswordSecretKeyRef>,
}

/// Selects a key of a secret in the node pod's namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BGPPeerPasswordSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

