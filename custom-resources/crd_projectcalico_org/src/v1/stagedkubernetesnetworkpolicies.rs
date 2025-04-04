// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/projectcalico/calico/crd.projectcalico.org/v1/stagedkubernetesnetworkpolicies.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "crd.projectcalico.org", version = "v1", kind = "StagedKubernetesNetworkPolicy", plural = "stagedkubernetesnetworkpolicies")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct StagedKubernetesNetworkPolicySpec {
    /// List of egress rules to be applied to the selected pods. Outgoing traffic is
    /// allowed if there are no NetworkPolicies selecting the pod (and cluster policy
    /// otherwise allows the traffic), OR if the traffic matches at least one egress rule
    /// across all of the NetworkPolicy objects whose podSelector matches the pod. If
    /// this field is empty then this NetworkPolicy limits all outgoing traffic (and serves
    /// solely to ensure that the pods it selects are isolated by default).
    /// This field is beta-level in 1.8
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub egress: Option<Vec<StagedKubernetesNetworkPolicyEgress>>,
    /// List of ingress rules to be applied to the selected pods. Traffic is allowed to
    /// a pod if there are no NetworkPolicies selecting the pod
    /// (and cluster policy otherwise allows the traffic), OR if the traffic source is
    /// the pod's local node, OR if the traffic matches at least one ingress rule
    /// across all of the NetworkPolicy objects whose podSelector matches the pod. If
    /// this field is empty then this NetworkPolicy does not allow any traffic (and serves
    /// solely to ensure that the pods it selects are isolated by default)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingress: Option<Vec<StagedKubernetesNetworkPolicyIngress>>,
    /// Selects the pods to which this NetworkPolicy object applies. The array of
    /// ingress rules is applied to any pods selected by this field. Multiple network
    /// policies can select the same set of pods. In this case, the ingress rules for
    /// each are combined additively. This field is NOT optional and follows standard
    /// label selector semantics. An empty podSelector matches all pods in this
    /// namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podSelector")]
    pub pod_selector: Option<StagedKubernetesNetworkPolicyPodSelector>,
    /// List of rule types that the NetworkPolicy relates to.
    /// Valid options are Ingress, Egress, or Ingress,Egress.
    /// If this field is not specified, it will default based on the existence of Ingress or Egress rules;
    /// policies that contain an Egress section are assumed to affect Egress, and all policies
    /// (whether or not they contain an Ingress section) are assumed to affect Ingress.
    /// If you want to write an egress-only policy, you must explicitly specify policyTypes [ "Egress" ].
    /// Likewise, if you want to write a policy that specifies that no egress is allowed,
    /// you must specify a policyTypes value that include "Egress" (since such a policy would not include
    /// an Egress section and would otherwise default to just [ "Ingress" ]).
    /// This field is beta-level in 1.8
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "policyTypes")]
    pub policy_types: Option<Vec<String>>,
    /// The staged action. If this is omitted, the default is Set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stagedAction")]
    pub staged_action: Option<String>,
}

/// NetworkPolicyEgressRule describes a particular set of traffic that is allowed out of pods
/// matched by a NetworkPolicySpec's podSelector. The traffic must match both ports and to.
/// This type is beta-level in 1.8
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StagedKubernetesNetworkPolicyEgress {
    /// ports is a list of destination ports for outgoing traffic.
    /// Each item in this list is combined using a logical OR. If this field is
    /// empty or missing, this rule matches all ports (traffic not restricted by port).
    /// If this field is present and contains at least one item, then this rule allows
    /// traffic only if the traffic matches at least one port in the list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<StagedKubernetesNetworkPolicyEgressPorts>>,
    /// to is a list of destinations for outgoing traffic of pods selected for this rule.
    /// Items in this list are combined using a logical OR operation. If this field is
    /// empty or missing, this rule matches all destinations (traffic not restricted by
    /// destination). If this field is present and contains at least one item, this rule
    /// allows traffic only if the traffic matches at least one item in the to list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<StagedKubernetesNetworkPolicyEgressTo>>,
}

/// NetworkPolicyPort describes a port to allow traffic on
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StagedKubernetesNetworkPolicyEgressPorts {
    /// endPort indicates that the range of ports from port to endPort if set, inclusive,
    /// should be allowed by the policy. This field cannot be defined if the port field
    /// is not defined or if the port field is defined as a named (string) port.
    /// The endPort must be equal or greater than port.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endPort")]
    pub end_port: Option<i32>,
    /// port represents the port on the given protocol. This can either be a numerical or named
    /// port on a pod. If this field is not provided, this matches all port names and
    /// numbers.
    /// If present, only traffic on the specified protocol AND port will be matched.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<IntOrString>,
    /// protocol represents the protocol (TCP, UDP, or SCTP) which traffic must match.
    /// If not specified, this field defaults to TCP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

/// NetworkPolicyPeer describes a peer to allow traffic to/from. Only certain combinations of
/// fields are allowed
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StagedKubernetesNetworkPolicyEgressTo {
    /// ipBlock defines policy on a particular IPBlock. If this field is set then
    /// neither of the other fields can be.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipBlock")]
    pub ip_block: Option<StagedKubernetesNetworkPolicyEgressToIpBlock>,
    /// namespaceSelector selects namespaces using cluster-scoped labels. This field follows
    /// standard label selector semantics; if present but empty, it selects all namespaces.
    /// 
    /// If podSelector is also set, then the NetworkPolicyPeer as a whole selects
    /// the pods matching podSelector in the namespaces selected by namespaceSelector.
    /// Otherwise it selects all pods in the namespaces selected by namespaceSelector.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceSelector")]
    pub namespace_selector: Option<StagedKubernetesNetworkPolicyEgressToNamespaceSelector>,
    /// podSelector is a label selector which selects pods. This field follows standard label
    /// selector semantics; if present but empty, it selects all pods.
    /// 
    /// If namespaceSelector is also set, then the NetworkPolicyPeer as a whole selects
    /// the pods matching podSelector in the Namespaces selected by NamespaceSelector.
    /// Otherwise it selects the pods matching podSelector in the policy's own namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podSelector")]
    pub pod_selector: Option<StagedKubernetesNetworkPolicyEgressToPodSelector>,
}

/// ipBlock defines policy on a particular IPBlock. If this field is set then
/// neither of the other fields can be.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StagedKubernetesNetworkPolicyEgressToIpBlock {
    /// cidr is a string representing the IPBlock
    /// Valid examples are "192.168.1.0/24" or "2001:db8::/64"
    pub cidr: String,
    /// except is a slice of CIDRs that should not be included within an IPBlock
    /// Valid examples are "192.168.1.0/24" or "2001:db8::/64"
    /// Except values will be rejected if they are outside the cidr range
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub except: Option<Vec<String>>,
}

/// namespaceSelector selects namespaces using cluster-scoped labels. This field follows
/// standard label selector semantics; if present but empty, it selects all namespaces.
/// 
/// If podSelector is also set, then the NetworkPolicyPeer as a whole selects
/// the pods matching podSelector in the namespaces selected by namespaceSelector.
/// Otherwise it selects all pods in the namespaces selected by namespaceSelector.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StagedKubernetesNetworkPolicyEgressToNamespaceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<StagedKubernetesNetworkPolicyEgressToNamespaceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StagedKubernetesNetworkPolicyEgressToNamespaceSelectorMatchExpressions {
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

/// podSelector is a label selector which selects pods. This field follows standard label
/// selector semantics; if present but empty, it selects all pods.
/// 
/// If namespaceSelector is also set, then the NetworkPolicyPeer as a whole selects
/// the pods matching podSelector in the Namespaces selected by NamespaceSelector.
/// Otherwise it selects the pods matching podSelector in the policy's own namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StagedKubernetesNetworkPolicyEgressToPodSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<StagedKubernetesNetworkPolicyEgressToPodSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StagedKubernetesNetworkPolicyEgressToPodSelectorMatchExpressions {
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

/// NetworkPolicyIngressRule describes a particular set of traffic that is allowed to the pods
/// matched by a NetworkPolicySpec's podSelector. The traffic must match both ports and from.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StagedKubernetesNetworkPolicyIngress {
    /// from is a list of sources which should be able to access the pods selected for this rule.
    /// Items in this list are combined using a logical OR operation. If this field is
    /// empty or missing, this rule matches all sources (traffic not restricted by
    /// source). If this field is present and contains at least one item, this rule
    /// allows traffic only if the traffic matches at least one item in the from list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<StagedKubernetesNetworkPolicyIngressFrom>>,
    /// ports is a list of ports which should be made accessible on the pods selected for
    /// this rule. Each item in this list is combined using a logical OR. If this field is
    /// empty or missing, this rule matches all ports (traffic not restricted by port).
    /// If this field is present and contains at least one item, then this rule allows
    /// traffic only if the traffic matches at least one port in the list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<StagedKubernetesNetworkPolicyIngressPorts>>,
}

/// NetworkPolicyPeer describes a peer to allow traffic to/from. Only certain combinations of
/// fields are allowed
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StagedKubernetesNetworkPolicyIngressFrom {
    /// ipBlock defines policy on a particular IPBlock. If this field is set then
    /// neither of the other fields can be.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipBlock")]
    pub ip_block: Option<StagedKubernetesNetworkPolicyIngressFromIpBlock>,
    /// namespaceSelector selects namespaces using cluster-scoped labels. This field follows
    /// standard label selector semantics; if present but empty, it selects all namespaces.
    /// 
    /// If podSelector is also set, then the NetworkPolicyPeer as a whole selects
    /// the pods matching podSelector in the namespaces selected by namespaceSelector.
    /// Otherwise it selects all pods in the namespaces selected by namespaceSelector.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceSelector")]
    pub namespace_selector: Option<StagedKubernetesNetworkPolicyIngressFromNamespaceSelector>,
    /// podSelector is a label selector which selects pods. This field follows standard label
    /// selector semantics; if present but empty, it selects all pods.
    /// 
    /// If namespaceSelector is also set, then the NetworkPolicyPeer as a whole selects
    /// the pods matching podSelector in the Namespaces selected by NamespaceSelector.
    /// Otherwise it selects the pods matching podSelector in the policy's own namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podSelector")]
    pub pod_selector: Option<StagedKubernetesNetworkPolicyIngressFromPodSelector>,
}

/// ipBlock defines policy on a particular IPBlock. If this field is set then
/// neither of the other fields can be.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StagedKubernetesNetworkPolicyIngressFromIpBlock {
    /// cidr is a string representing the IPBlock
    /// Valid examples are "192.168.1.0/24" or "2001:db8::/64"
    pub cidr: String,
    /// except is a slice of CIDRs that should not be included within an IPBlock
    /// Valid examples are "192.168.1.0/24" or "2001:db8::/64"
    /// Except values will be rejected if they are outside the cidr range
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub except: Option<Vec<String>>,
}

/// namespaceSelector selects namespaces using cluster-scoped labels. This field follows
/// standard label selector semantics; if present but empty, it selects all namespaces.
/// 
/// If podSelector is also set, then the NetworkPolicyPeer as a whole selects
/// the pods matching podSelector in the namespaces selected by namespaceSelector.
/// Otherwise it selects all pods in the namespaces selected by namespaceSelector.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StagedKubernetesNetworkPolicyIngressFromNamespaceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<StagedKubernetesNetworkPolicyIngressFromNamespaceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StagedKubernetesNetworkPolicyIngressFromNamespaceSelectorMatchExpressions {
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

/// podSelector is a label selector which selects pods. This field follows standard label
/// selector semantics; if present but empty, it selects all pods.
/// 
/// If namespaceSelector is also set, then the NetworkPolicyPeer as a whole selects
/// the pods matching podSelector in the Namespaces selected by NamespaceSelector.
/// Otherwise it selects the pods matching podSelector in the policy's own namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StagedKubernetesNetworkPolicyIngressFromPodSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<StagedKubernetesNetworkPolicyIngressFromPodSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StagedKubernetesNetworkPolicyIngressFromPodSelectorMatchExpressions {
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

/// NetworkPolicyPort describes a port to allow traffic on
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StagedKubernetesNetworkPolicyIngressPorts {
    /// endPort indicates that the range of ports from port to endPort if set, inclusive,
    /// should be allowed by the policy. This field cannot be defined if the port field
    /// is not defined or if the port field is defined as a named (string) port.
    /// The endPort must be equal or greater than port.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endPort")]
    pub end_port: Option<i32>,
    /// port represents the port on the given protocol. This can either be a numerical or named
    /// port on a pod. If this field is not provided, this matches all port names and
    /// numbers.
    /// If present, only traffic on the specified protocol AND port will be matched.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<IntOrString>,
    /// protocol represents the protocol (TCP, UDP, or SCTP) which traffic must match.
    /// If not specified, this field defaults to TCP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

/// Selects the pods to which this NetworkPolicy object applies. The array of
/// ingress rules is applied to any pods selected by this field. Multiple network
/// policies can select the same set of pods. In this case, the ingress rules for
/// each are combined additively. This field is NOT optional and follows standard
/// label selector semantics. An empty podSelector matches all pods in this
/// namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StagedKubernetesNetworkPolicyPodSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<StagedKubernetesNetworkPolicyPodSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StagedKubernetesNetworkPolicyPodSelectorMatchExpressions {
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

