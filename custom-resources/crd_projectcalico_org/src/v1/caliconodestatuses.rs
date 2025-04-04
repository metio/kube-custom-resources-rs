// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/projectcalico/calico/crd.projectcalico.org/v1/caliconodestatuses.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// CalicoNodeStatusSpec contains the specification for a CalicoNodeStatus resource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "crd.projectcalico.org", version = "v1", kind = "CalicoNodeStatus", plural = "caliconodestatuses")]
#[kube(status = "CalicoNodeStatusStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct CalicoNodeStatusSpec {
    /// Classes declares the types of information to monitor for this calico/node,
    /// and allows for selective status reporting about certain subsets of information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classes: Option<Vec<String>>,
    /// The node name identifies the Calico node instance for node status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
    /// UpdatePeriodSeconds is the period at which CalicoNodeStatus should be updated.
    /// Set to 0 to disable CalicoNodeStatus refresh. Maximum update period is one day.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updatePeriodSeconds")]
    pub update_period_seconds: Option<i32>,
}

/// CalicoNodeStatusStatus defines the observed state of CalicoNodeStatus.
/// No validation needed for status since it is updated by Calico.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CalicoNodeStatusStatus {
    /// Agent holds agent status on the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<CalicoNodeStatusStatusAgent>,
    /// BGP holds node BGP status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bgp: Option<CalicoNodeStatusStatusBgp>,
    /// LastUpdated is a timestamp representing the server time when CalicoNodeStatus object
    /// last updated. It is represented in RFC3339 form and is in UTC.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUpdated")]
    pub last_updated: Option<String>,
    /// Routes reports routes known to the Calico BGP daemon on the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routes: Option<CalicoNodeStatusStatusRoutes>,
}

/// Agent holds agent status on the node.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CalicoNodeStatusStatusAgent {
    /// BIRDV4 represents the latest observed status of bird4.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "birdV4")]
    pub bird_v4: Option<CalicoNodeStatusStatusAgentBirdV4>,
    /// BIRDV6 represents the latest observed status of bird6.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "birdV6")]
    pub bird_v6: Option<CalicoNodeStatusStatusAgentBirdV6>,
}

/// BIRDV4 represents the latest observed status of bird4.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CalicoNodeStatusStatusAgentBirdV4 {
    /// LastBootTime holds the value of lastBootTime from bird.ctl output.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastBootTime")]
    pub last_boot_time: Option<String>,
    /// LastReconfigurationTime holds the value of lastReconfigTime from bird.ctl output.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastReconfigurationTime")]
    pub last_reconfiguration_time: Option<String>,
    /// Router ID used by bird.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routerID")]
    pub router_id: Option<String>,
    /// The state of the BGP Daemon.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Version of the BGP daemon
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// BIRDV6 represents the latest observed status of bird6.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CalicoNodeStatusStatusAgentBirdV6 {
    /// LastBootTime holds the value of lastBootTime from bird.ctl output.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastBootTime")]
    pub last_boot_time: Option<String>,
    /// LastReconfigurationTime holds the value of lastReconfigTime from bird.ctl output.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastReconfigurationTime")]
    pub last_reconfiguration_time: Option<String>,
    /// Router ID used by bird.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routerID")]
    pub router_id: Option<String>,
    /// The state of the BGP Daemon.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Version of the BGP daemon
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// BGP holds node BGP status.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CalicoNodeStatusStatusBgp {
    /// The total number of IPv4 established bgp sessions.
    #[serde(rename = "numberEstablishedV4")]
    pub number_established_v4: i64,
    /// The total number of IPv6 established bgp sessions.
    #[serde(rename = "numberEstablishedV6")]
    pub number_established_v6: i64,
    /// The total number of IPv4 non-established bgp sessions.
    #[serde(rename = "numberNotEstablishedV4")]
    pub number_not_established_v4: i64,
    /// The total number of IPv6 non-established bgp sessions.
    #[serde(rename = "numberNotEstablishedV6")]
    pub number_not_established_v6: i64,
    /// PeersV4 represents IPv4 BGP peers status on the node.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "peersV4")]
    pub peers_v4: Option<Vec<CalicoNodeStatusStatusBgpPeersV4>>,
    /// PeersV6 represents IPv6 BGP peers status on the node.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "peersV6")]
    pub peers_v6: Option<Vec<CalicoNodeStatusStatusBgpPeersV6>>,
}

/// CalicoNodePeer contains the status of BGP peers on the node.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CalicoNodeStatusStatusBgpPeersV4 {
    /// IP address of the peer whose condition we are reporting.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "peerIP")]
    pub peer_ip: Option<String>,
    /// Since the state or reason last changed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    /// State is the BGP session state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Type indicates whether this peer is configured via the node-to-node mesh,
    /// or via en explicit global or per-node BGPPeer object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// CalicoNodePeer contains the status of BGP peers on the node.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CalicoNodeStatusStatusBgpPeersV6 {
    /// IP address of the peer whose condition we are reporting.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "peerIP")]
    pub peer_ip: Option<String>,
    /// Since the state or reason last changed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    /// State is the BGP session state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Type indicates whether this peer is configured via the node-to-node mesh,
    /// or via en explicit global or per-node BGPPeer object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// Routes reports routes known to the Calico BGP daemon on the node.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CalicoNodeStatusStatusRoutes {
    /// RoutesV4 represents IPv4 routes on the node.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routesV4")]
    pub routes_v4: Option<Vec<CalicoNodeStatusStatusRoutesRoutesV4>>,
    /// RoutesV6 represents IPv6 routes on the node.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routesV6")]
    pub routes_v6: Option<Vec<CalicoNodeStatusStatusRoutesRoutesV6>>,
}

/// CalicoNodeRoute contains the status of BGP routes on the node.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CalicoNodeStatusStatusRoutesRoutesV4 {
    /// Destination of the route.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// Gateway for the destination.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    /// Interface for the destination
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interface: Option<String>,
    /// LearnedFrom contains information regarding where this route originated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "learnedFrom")]
    pub learned_from: Option<CalicoNodeStatusStatusRoutesRoutesV4LearnedFrom>,
    /// Type indicates if the route is being used for forwarding or not.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// LearnedFrom contains information regarding where this route originated.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CalicoNodeStatusStatusRoutesRoutesV4LearnedFrom {
    /// If sourceType is NodeMesh or BGPPeer, IP address of the router that sent us this route.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "peerIP")]
    pub peer_ip: Option<String>,
    /// Type of the source where a route is learned from.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceType")]
    pub source_type: Option<String>,
}

/// CalicoNodeRoute contains the status of BGP routes on the node.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CalicoNodeStatusStatusRoutesRoutesV6 {
    /// Destination of the route.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// Gateway for the destination.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    /// Interface for the destination
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interface: Option<String>,
    /// LearnedFrom contains information regarding where this route originated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "learnedFrom")]
    pub learned_from: Option<CalicoNodeStatusStatusRoutesRoutesV6LearnedFrom>,
    /// Type indicates if the route is being used for forwarding or not.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// LearnedFrom contains information regarding where this route originated.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CalicoNodeStatusStatusRoutesRoutesV6LearnedFrom {
    /// If sourceType is NodeMesh or BGPPeer, IP address of the router that sent us this route.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "peerIP")]
    pub peer_ip: Option<String>,
    /// Type of the source where a route is learned from.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceType")]
    pub source_type: Option<String>,
}

