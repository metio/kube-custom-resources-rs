// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/metallb/metallb-operator/metallb.io/v1beta1/bgpadvertisements.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// BGPAdvertisementSpec defines the desired state of BGPAdvertisement.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "metallb.io", version = "v1beta1", kind = "BGPAdvertisement", plural = "bgpadvertisements")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct BGPAdvertisementSpec {
    /// The aggregation-length advertisement option lets you “roll up” the /32s into a larger prefix. Defaults to 32. Works for IPv4 addresses.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "aggregationLength")]
    pub aggregation_length: Option<i32>,
    /// The aggregation-length advertisement option lets you “roll up” the /128s into a larger prefix. Defaults to 128. Works for IPv6 addresses.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "aggregationLengthV6")]
    pub aggregation_length_v6: Option<i32>,
    /// The BGP communities to be associated with the announcement. Each item can be a standard community of the
    /// form 1234:1234, a large community of the form large:1234:1234:1234 or the name of an alias defined in the
    /// Community CRD.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub communities: Option<Vec<String>>,
    /// A selector for the IPAddressPools which would get advertised via this advertisement.
    /// If no IPAddressPool is selected by this or by the list, the advertisement is applied to all the IPAddressPools.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipAddressPoolSelectors")]
    pub ip_address_pool_selectors: Option<Vec<BGPAdvertisementIpAddressPoolSelectors>>,
    /// The list of IPAddressPools to advertise via this advertisement, selected by name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipAddressPools")]
    pub ip_address_pools: Option<Vec<String>>,
    /// The BGP LOCAL_PREF attribute which is used by BGP best path algorithm,
    /// Path with higher localpref is preferred over one with lower localpref.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localPref")]
    pub local_pref: Option<i32>,
    /// NodeSelectors allows to limit the nodes to announce as next hops for the LoadBalancer IP. When empty, all the nodes having  are announced as next hops.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelectors")]
    pub node_selectors: Option<Vec<BGPAdvertisementNodeSelectors>>,
    /// Peers limits the bgppeer to advertise the ips of the selected pools to.
    /// When empty, the loadbalancer IP is announced to all the BGPPeers configured.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peers: Option<Vec<String>>,
}

/// A label selector is a label query over a set of resources. The result of matchLabels and
/// matchExpressions are ANDed. An empty label selector matches all objects. A null
/// label selector matches no objects.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BGPAdvertisementIpAddressPoolSelectors {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<BGPAdvertisementIpAddressPoolSelectorsMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BGPAdvertisementIpAddressPoolSelectorsMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// A label selector is a label query over a set of resources. The result of matchLabels and
/// matchExpressions are ANDed. An empty label selector matches all objects. A null
/// label selector matches no objects.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BGPAdvertisementNodeSelectors {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<BGPAdvertisementNodeSelectorsMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BGPAdvertisementNodeSelectorsMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// BGPAdvertisementStatus defines the observed state of BGPAdvertisement.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BGPAdvertisementStatus {
}

