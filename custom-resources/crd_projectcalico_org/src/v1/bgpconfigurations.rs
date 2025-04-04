// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/projectcalico/calico/crd.projectcalico.org/v1/bgpconfigurations.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// BGPConfigurationSpec contains the values of the BGP configuration.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "crd.projectcalico.org", version = "v1", kind = "BGPConfiguration", plural = "bgpconfigurations")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct BGPConfigurationSpec {
    /// ASNumber is the default AS number used by a node. [Default: 64512]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "asNumber")]
    pub as_number: Option<i32>,
    /// BindMode indicates whether to listen for BGP connections on all addresses (None)
    /// or only on the node's canonical IP address Node.Spec.BGP.IPvXAddress (NodeIP).
    /// Default behaviour is to listen for BGP connections on all addresses.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bindMode")]
    pub bind_mode: Option<String>,
    /// Communities is a list of BGP community values and their arbitrary names for tagging routes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub communities: Option<Vec<BGPConfigurationCommunities>>,
    /// IgnoredInterfaces indicates the network interfaces that needs to be excluded when reading device routes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ignoredInterfaces")]
    pub ignored_interfaces: Option<Vec<String>>,
    /// ListenPort is the port where BGP protocol should listen. Defaults to 179
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "listenPort")]
    pub listen_port: Option<i64>,
    /// The virtual IPv4 address of the node with which its local workload is expected to peer.
    /// It is recommended to use a link-local address.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localWorkloadPeeringIPV4")]
    pub local_workload_peering_ipv4: Option<String>,
    /// The virtual IPv6 address of the node with which its local workload is expected to peer.
    /// It is recommended to use a link-local address.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localWorkloadPeeringIPV6")]
    pub local_workload_peering_ipv6: Option<String>,
    /// LogSeverityScreen is the log severity above which logs are sent to the stdout. [Default: INFO]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logSeverityScreen")]
    pub log_severity_screen: Option<String>,
    /// Time to allow for software restart for node-to-mesh peerings.  When specified, this is configured
    /// as the graceful restart timeout.  When not specified, the BIRD default of 120s is used.
    /// This field can only be set on the default BGPConfiguration instance and requires that NodeMesh is enabled
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeMeshMaxRestartTime")]
    pub node_mesh_max_restart_time: Option<String>,
    /// Optional BGP password for full node-to-mesh peerings.
    /// This field can only be set on the default BGPConfiguration instance and requires that NodeMesh is enabled
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeMeshPassword")]
    pub node_mesh_password: Option<BGPConfigurationNodeMeshPassword>,
    /// NodeToNodeMeshEnabled sets whether full node to node BGP mesh is enabled. [Default: true]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeToNodeMeshEnabled")]
    pub node_to_node_mesh_enabled: Option<bool>,
    /// PrefixAdvertisements contains per-prefix advertisement configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "prefixAdvertisements")]
    pub prefix_advertisements: Option<Vec<BGPConfigurationPrefixAdvertisements>>,
    /// ServiceClusterIPs are the CIDR blocks from which service cluster IPs are allocated.
    /// If specified, Calico will advertise these blocks, as well as any cluster IPs within them.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceClusterIPs")]
    pub service_cluster_i_ps: Option<Vec<BGPConfigurationServiceClusterIPs>>,
    /// ServiceExternalIPs are the CIDR blocks for Kubernetes Service External IPs.
    /// Kubernetes Service ExternalIPs will only be advertised if they are within one of these blocks.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceExternalIPs")]
    pub service_external_i_ps: Option<Vec<BGPConfigurationServiceExternalIPs>>,
    /// ServiceLoadBalancerIPs are the CIDR blocks for Kubernetes Service LoadBalancer IPs.
    /// Kubernetes Service status.LoadBalancer.Ingress IPs will only be advertised if they are within one of these blocks.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceLoadBalancerIPs")]
    pub service_load_balancer_i_ps: Option<Vec<BGPConfigurationServiceLoadBalancerIPs>>,
}

/// Community contains standard or large community value and its name.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BGPConfigurationCommunities {
    /// Name given to community value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Value must be of format `aa:nn` or `aa:nn:mm`.
    /// For standard community use `aa:nn` format, where `aa` and `nn` are 16 bit number.
    /// For large community use `aa:nn:mm` format, where `aa`, `nn` and `mm` are 32 bit number.
    /// Where, `aa` is an AS Number, `nn` and `mm` are per-AS identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Optional BGP password for full node-to-mesh peerings.
/// This field can only be set on the default BGPConfiguration instance and requires that NodeMesh is enabled
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BGPConfigurationNodeMeshPassword {
    /// Selects a key of a secret in the node pod's namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<BGPConfigurationNodeMeshPasswordSecretKeyRef>,
}

/// Selects a key of a secret in the node pod's namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BGPConfigurationNodeMeshPasswordSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// PrefixAdvertisement configures advertisement properties for the specified CIDR.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BGPConfigurationPrefixAdvertisements {
    /// CIDR for which properties should be advertised.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
    /// Communities can be list of either community names already defined in `Specs.Communities` or community value of format `aa:nn` or `aa:nn:mm`.
    /// For standard community use `aa:nn` format, where `aa` and `nn` are 16 bit number.
    /// For large community use `aa:nn:mm` format, where `aa`, `nn` and `mm` are 32 bit number.
    /// Where,`aa` is an AS Number, `nn` and `mm` are per-AS identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub communities: Option<Vec<String>>,
}

/// ServiceClusterIPBlock represents a single allowed ClusterIP CIDR block.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BGPConfigurationServiceClusterIPs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
}

/// ServiceExternalIPBlock represents a single allowed External IP CIDR block.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BGPConfigurationServiceExternalIPs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
}

/// ServiceLoadBalancerIPBlock represents a single allowed LoadBalancer IP CIDR block.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BGPConfigurationServiceLoadBalancerIPs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
}

