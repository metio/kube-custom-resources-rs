// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/cilium/cilium/cilium.io/v2/ciliumbgppeerconfigs.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// Spec is the specification of the desired behavior of the CiliumBGPPeerConfig.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "cilium.io", version = "v2", kind = "CiliumBGPPeerConfig", plural = "ciliumbgppeerconfigs")]
#[kube(status = "CiliumBGPPeerConfigStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct CiliumBGPPeerConfigSpec {
    /// AuthSecretRef is the name of the secret to use to fetch a TCP
    /// authentication password for this peer.
    /// 
    /// If not specified, no authentication is used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authSecretRef")]
    pub auth_secret_ref: Option<String>,
    /// EBGPMultihopTTL controls the multi-hop feature for eBGP peers.
    /// Its value defines the Time To Live (TTL) value used in BGP
    /// packets sent to the peer.
    /// 
    /// If not specified, EBGP multihop is disabled. This field is ignored for iBGP neighbors.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ebgpMultihop")]
    pub ebgp_multihop: Option<i32>,
    /// Families, if provided, defines a set of AFI/SAFIs the speaker will
    /// negotiate with it's peer.
    /// 
    /// If not specified, the default families of IPv6/unicast and IPv4/unicast will be created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub families: Option<Vec<CiliumBGPPeerConfigFamilies>>,
    /// GracefulRestart defines graceful restart parameters which are negotiated
    /// with this peer.
    /// 
    /// If not specified, the graceful restart capability is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gracefulRestart")]
    pub graceful_restart: Option<CiliumBGPPeerConfigGracefulRestart>,
    /// Timers defines the BGP timers for the peer.
    /// 
    /// If not specified, the default timers are used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timers: Option<CiliumBGPPeerConfigTimers>,
    /// Transport defines the BGP transport parameters for the peer.
    /// 
    /// If not specified, the default transport parameters are used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport: Option<CiliumBGPPeerConfigTransport>,
}

/// CiliumBGPFamilyWithAdverts represents a AFI/SAFI address family pair along with reference to BGP Advertisements.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CiliumBGPPeerConfigFamilies {
    /// Advertisements selects group of BGP Advertisement(s) to advertise for this family.
    /// 
    /// If not specified, no advertisements are sent for this family.
    /// 
    /// This field is ignored in CiliumBGPNeighbor which is used in CiliumBGPPeeringPolicy.
    /// Use CiliumBGPPeeringPolicy advertisement options instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub advertisements: Option<CiliumBGPPeerConfigFamiliesAdvertisements>,
    /// Afi is the Address Family Identifier (AFI) of the family.
    pub afi: CiliumBGPPeerConfigFamiliesAfi,
    /// Safi is the Subsequent Address Family Identifier (SAFI) of the family.
    pub safi: CiliumBGPPeerConfigFamiliesSafi,
}

/// Advertisements selects group of BGP Advertisement(s) to advertise for this family.
/// 
/// If not specified, no advertisements are sent for this family.
/// 
/// This field is ignored in CiliumBGPNeighbor which is used in CiliumBGPPeeringPolicy.
/// Use CiliumBGPPeeringPolicy advertisement options instead.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CiliumBGPPeerConfigFamiliesAdvertisements {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CiliumBGPPeerConfigFamiliesAdvertisementsMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CiliumBGPPeerConfigFamiliesAdvertisementsMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: CiliumBGPPeerConfigFamiliesAdvertisementsMatchExpressionsOperator,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CiliumBGPPeerConfigFamiliesAdvertisementsMatchExpressionsOperator {
    In,
    NotIn,
    Exists,
    DoesNotExist,
}

/// CiliumBGPFamilyWithAdverts represents a AFI/SAFI address family pair along with reference to BGP Advertisements.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CiliumBGPPeerConfigFamiliesAfi {
    #[serde(rename = "ipv4")]
    Ipv4,
    #[serde(rename = "ipv6")]
    Ipv6,
    #[serde(rename = "l2vpn")]
    L2vpn,
    #[serde(rename = "ls")]
    Ls,
    #[serde(rename = "opaque")]
    Opaque,
}

/// CiliumBGPFamilyWithAdverts represents a AFI/SAFI address family pair along with reference to BGP Advertisements.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CiliumBGPPeerConfigFamiliesSafi {
    #[serde(rename = "unicast")]
    Unicast,
    #[serde(rename = "multicast")]
    Multicast,
    #[serde(rename = "mpls_label")]
    MplsLabel,
    #[serde(rename = "encapsulation")]
    Encapsulation,
    #[serde(rename = "vpls")]
    Vpls,
    #[serde(rename = "evpn")]
    Evpn,
    #[serde(rename = "ls")]
    Ls,
    #[serde(rename = "sr_policy")]
    SrPolicy,
    #[serde(rename = "mup")]
    Mup,
    #[serde(rename = "mpls_vpn")]
    MplsVpn,
    #[serde(rename = "mpls_vpn_multicast")]
    MplsVpnMulticast,
    #[serde(rename = "route_target_constraints")]
    RouteTargetConstraints,
    #[serde(rename = "flowspec_unicast")]
    FlowspecUnicast,
    #[serde(rename = "flowspec_vpn")]
    FlowspecVpn,
    #[serde(rename = "key_value")]
    KeyValue,
}

/// GracefulRestart defines graceful restart parameters which are negotiated
/// with this peer.
/// 
/// If not specified, the graceful restart capability is disabled.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CiliumBGPPeerConfigGracefulRestart {
    /// Enabled flag, when set enables graceful restart capability.
    pub enabled: bool,
    /// RestartTimeSeconds is the estimated time it will take for the BGP
    /// session to be re-established with peer after a restart.
    /// After this period, peer will remove stale routes. This is
    /// described RFC 4724 section 4.2.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "restartTimeSeconds")]
    pub restart_time_seconds: Option<i32>,
}

/// Timers defines the BGP timers for the peer.
/// 
/// If not specified, the default timers are used.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CiliumBGPPeerConfigTimers {
    /// ConnectRetryTimeSeconds defines the initial value for the BGP ConnectRetryTimer (RFC 4271, Section 8).
    /// 
    /// If not specified, defaults to 120 seconds.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "connectRetryTimeSeconds")]
    pub connect_retry_time_seconds: Option<i32>,
    /// HoldTimeSeconds defines the initial value for the BGP HoldTimer (RFC 4271, Section 4.2).
    /// Updating this value will cause a session reset.
    /// 
    /// If not specified, defaults to 90 seconds.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "holdTimeSeconds")]
    pub hold_time_seconds: Option<i32>,
    /// KeepaliveTimeSeconds defines the initial value for the BGP KeepaliveTimer (RFC 4271, Section 8).
    /// It can not be larger than HoldTimeSeconds. Updating this value will cause a session reset.
    /// 
    /// If not specified, defaults to 30 seconds.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keepAliveTimeSeconds")]
    pub keep_alive_time_seconds: Option<i32>,
}

/// Transport defines the BGP transport parameters for the peer.
/// 
/// If not specified, the default transport parameters are used.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CiliumBGPPeerConfigTransport {
    /// PeerPort is the peer port to be used for the BGP session.
    /// 
    /// If not specified, defaults to TCP port 179.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "peerPort")]
    pub peer_port: Option<i32>,
}

/// Status is the running status of the CiliumBGPPeerConfig
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CiliumBGPPeerConfigStatus {
    /// The current conditions of the CiliumBGPPeerConfig
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

