// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/network-policy-api/policy.networking.k8s.io/v1alpha1/baselineadminnetworkpolicies.yaml --derive=Default --derive=PartialEq
// kopium version: 0.18.0

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;

/// Specification of the desired behavior of BaselineAdminNetworkPolicy.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "policy.networking.k8s.io", version = "v1alpha1", kind = "BaselineAdminNetworkPolicy", plural = "baselineadminnetworkpolicies")]
#[kube(status = "BaselineAdminNetworkPolicyStatus")]
#[kube(schema = "disabled")]
pub struct BaselineAdminNetworkPolicySpec {
    /// Egress is the list of Egress rules to be applied to the selected pods if
    /// they are not matched by any AdminNetworkPolicy or NetworkPolicy rules.
    /// A total of 100 Egress rules will be allowed in each BANP instance.
    /// The relative precedence of egress rules within a single BANP object
    /// will be determined by the order in which the rule is written.
    /// Thus, a rule that appears at the top of the egress rules
    /// would take the highest precedence.
    /// BANPs with no egress rules do not affect egress traffic.
    /// 
    /// 
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub egress: Option<Vec<BaselineAdminNetworkPolicyEgress>>,
    /// Ingress is the list of Ingress rules to be applied to the selected pods
    /// if they are not matched by any AdminNetworkPolicy or NetworkPolicy rules.
    /// A total of 100 Ingress rules will be allowed in each BANP instance.
    /// The relative precedence of ingress rules within a single BANP object
    /// will be determined by the order in which the rule is written.
    /// Thus, a rule that appears at the top of the ingress rules
    /// would take the highest precedence.
    /// BANPs with no ingress rules do not affect ingress traffic.
    /// 
    /// 
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingress: Option<Vec<BaselineAdminNetworkPolicyIngress>>,
    /// Subject defines the pods to which this BaselineAdminNetworkPolicy applies.
    /// Note that host-networked pods are not included in subject selection.
    /// 
    /// 
    /// Support: Core
    pub subject: BaselineAdminNetworkPolicySubject,
}

/// BaselineAdminNetworkPolicyEgressRule describes an action to take on a particular
/// set of traffic originating from pods selected by a BaselineAdminNetworkPolicy's
/// Subject field.
/// <network-policy-api:experimental:validation>
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicyEgress {
    /// Action specifies the effect this rule will have on matching traffic.
    /// Currently the following actions are supported:
    /// Allow: allows the selected traffic
    /// Deny: denies the selected traffic
    /// 
    /// 
    /// Support: Core
    pub action: BaselineAdminNetworkPolicyEgressAction,
    /// Name is an identifier for this rule, that may be no more than 100 characters
    /// in length. This field should be used by the implementation to help
    /// improve observability, readability and error-reporting for any applied
    /// BaselineAdminNetworkPolicies.
    /// 
    /// 
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Ports allows for matching traffic based on port and protocols.
    /// This field is a list of destination ports for the outgoing egress traffic.
    /// If Ports is not set then the rule does not filter traffic via port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<BaselineAdminNetworkPolicyEgressPorts>>,
    /// To is the list of destinations whose traffic this rule applies to.
    /// If any AdminNetworkPolicyEgressPeer matches the destination of outgoing
    /// traffic then the specified action is applied.
    /// This field must be defined and contain at least one item.
    /// 
    /// 
    /// Support: Core
    pub to: Vec<BaselineAdminNetworkPolicyEgressTo>,
}

/// BaselineAdminNetworkPolicyEgressRule describes an action to take on a particular
/// set of traffic originating from pods selected by a BaselineAdminNetworkPolicy's
/// Subject field.
/// <network-policy-api:experimental:validation>
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BaselineAdminNetworkPolicyEgressAction {
    Allow,
    Deny,
}

/// AdminNetworkPolicyPort describes how to select network ports on pod(s).
/// Exactly one field must be set.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicyEgressPorts {
    /// Port selects a port on a pod(s) based on number.
    /// 
    /// 
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portNumber")]
    pub port_number: Option<BaselineAdminNetworkPolicyEgressPortsPortNumber>,
    /// PortRange selects a port range on a pod(s) based on provided start and end
    /// values.
    /// 
    /// 
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portRange")]
    pub port_range: Option<BaselineAdminNetworkPolicyEgressPortsPortRange>,
}

/// Port selects a port on a pod(s) based on number.
/// 
/// 
/// Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicyEgressPortsPortNumber {
    /// Number defines a network port value.
    /// 
    /// 
    /// Support: Core
    pub port: i32,
    /// Protocol is the network protocol (TCP, UDP, or SCTP) which traffic must
    /// match. If not specified, this field defaults to TCP.
    /// 
    /// 
    /// Support: Core
    pub protocol: String,
}

/// PortRange selects a port range on a pod(s) based on provided start and end
/// values.
/// 
/// 
/// Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicyEgressPortsPortRange {
    /// End defines a network port that is the end of a port range, the End value
    /// must be greater than Start.
    /// 
    /// 
    /// Support: Core
    pub end: i32,
    /// Protocol is the network protocol (TCP, UDP, or SCTP) which traffic must
    /// match. If not specified, this field defaults to TCP.
    /// 
    /// 
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// Start defines a network port that is the start of a port range, the Start
    /// value must be less than End.
    /// 
    /// 
    /// Support: Core
    pub start: i32,
}

/// AdminNetworkPolicyEgressPeer defines a peer to allow traffic to.
/// Exactly one of the selector pointers must be set for a given peer. If a
/// consumer observes none of its fields are set, they must assume an unknown
/// option has been specified and fail closed.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicyEgressTo {
    /// Namespaces defines a way to select all pods within a set of Namespaces.
    /// Note that host-networked pods are not included in this type of peer.
    /// 
    /// 
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<BaselineAdminNetworkPolicyEgressToNamespaces>,
    /// Pods defines a way to select a set of pods in
    /// a set of namespaces. Note that host-networked pods
    /// are not included in this type of peer.
    /// 
    /// 
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pods: Option<BaselineAdminNetworkPolicyEgressToPods>,
}

/// Namespaces defines a way to select all pods within a set of Namespaces.
/// Note that host-networked pods are not included in this type of peer.
/// 
/// 
/// Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicyEgressToNamespaces {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<BaselineAdminNetworkPolicyEgressToNamespacesMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicyEgressToNamespacesMatchExpressions {
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

/// Pods defines a way to select a set of pods in
/// a set of namespaces. Note that host-networked pods
/// are not included in this type of peer.
/// 
/// 
/// Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicyEgressToPods {
    /// NamespaceSelector follows standard label selector semantics; if empty,
    /// it selects all Namespaces.
    #[serde(rename = "namespaceSelector")]
    pub namespace_selector: BaselineAdminNetworkPolicyEgressToPodsNamespaceSelector,
    /// PodSelector is used to explicitly select pods within a namespace; if empty,
    /// it selects all Pods.
    #[serde(rename = "podSelector")]
    pub pod_selector: BaselineAdminNetworkPolicyEgressToPodsPodSelector,
}

/// NamespaceSelector follows standard label selector semantics; if empty,
/// it selects all Namespaces.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicyEgressToPodsNamespaceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<BaselineAdminNetworkPolicyEgressToPodsNamespaceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicyEgressToPodsNamespaceSelectorMatchExpressions {
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

/// PodSelector is used to explicitly select pods within a namespace; if empty,
/// it selects all Pods.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicyEgressToPodsPodSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<BaselineAdminNetworkPolicyEgressToPodsPodSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicyEgressToPodsPodSelectorMatchExpressions {
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

/// BaselineAdminNetworkPolicyIngressRule describes an action to take on a particular
/// set of traffic destined for pods selected by a BaselineAdminNetworkPolicy's
/// Subject field.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicyIngress {
    /// Action specifies the effect this rule will have on matching traffic.
    /// Currently the following actions are supported:
    /// Allow: allows the selected traffic
    /// Deny: denies the selected traffic
    /// 
    /// 
    /// Support: Core
    pub action: BaselineAdminNetworkPolicyIngressAction,
    /// From is the list of sources whose traffic this rule applies to.
    /// If any AdminNetworkPolicyIngressPeer matches the source of incoming
    /// traffic then the specified action is applied.
    /// This field must be defined and contain at least one item.
    /// 
    /// 
    /// Support: Core
    pub from: Vec<BaselineAdminNetworkPolicyIngressFrom>,
    /// Name is an identifier for this rule, that may be no more than 100 characters
    /// in length. This field should be used by the implementation to help
    /// improve observability, readability and error-reporting for any applied
    /// BaselineAdminNetworkPolicies.
    /// 
    /// 
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Ports allows for matching traffic based on port and protocols.
    /// This field is a list of ports which should be matched on
    /// the pods selected for this policy i.e the subject of the policy.
    /// So it matches on the destination port for the ingress traffic.
    /// If Ports is not set then the rule does not filter traffic via port.
    /// 
    /// 
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<BaselineAdminNetworkPolicyIngressPorts>>,
}

/// BaselineAdminNetworkPolicyIngressRule describes an action to take on a particular
/// set of traffic destined for pods selected by a BaselineAdminNetworkPolicy's
/// Subject field.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BaselineAdminNetworkPolicyIngressAction {
    Allow,
    Deny,
}

/// AdminNetworkPolicyIngressPeer defines an in-cluster peer to allow traffic from.
/// Exactly one of the selector pointers must be set for a given peer. If a
/// consumer observes none of its fields are set, they must assume an unknown
/// option has been specified and fail closed.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicyIngressFrom {
    /// Namespaces defines a way to select all pods within a set of Namespaces.
    /// Note that host-networked pods are not included in this type of peer.
    /// 
    /// 
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<BaselineAdminNetworkPolicyIngressFromNamespaces>,
    /// Pods defines a way to select a set of pods in
    /// a set of namespaces. Note that host-networked pods
    /// are not included in this type of peer.
    /// 
    /// 
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pods: Option<BaselineAdminNetworkPolicyIngressFromPods>,
}

/// Namespaces defines a way to select all pods within a set of Namespaces.
/// Note that host-networked pods are not included in this type of peer.
/// 
/// 
/// Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicyIngressFromNamespaces {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<BaselineAdminNetworkPolicyIngressFromNamespacesMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicyIngressFromNamespacesMatchExpressions {
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

/// Pods defines a way to select a set of pods in
/// a set of namespaces. Note that host-networked pods
/// are not included in this type of peer.
/// 
/// 
/// Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicyIngressFromPods {
    /// NamespaceSelector follows standard label selector semantics; if empty,
    /// it selects all Namespaces.
    #[serde(rename = "namespaceSelector")]
    pub namespace_selector: BaselineAdminNetworkPolicyIngressFromPodsNamespaceSelector,
    /// PodSelector is used to explicitly select pods within a namespace; if empty,
    /// it selects all Pods.
    #[serde(rename = "podSelector")]
    pub pod_selector: BaselineAdminNetworkPolicyIngressFromPodsPodSelector,
}

/// NamespaceSelector follows standard label selector semantics; if empty,
/// it selects all Namespaces.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicyIngressFromPodsNamespaceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<BaselineAdminNetworkPolicyIngressFromPodsNamespaceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicyIngressFromPodsNamespaceSelectorMatchExpressions {
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

/// PodSelector is used to explicitly select pods within a namespace; if empty,
/// it selects all Pods.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicyIngressFromPodsPodSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<BaselineAdminNetworkPolicyIngressFromPodsPodSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicyIngressFromPodsPodSelectorMatchExpressions {
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

/// AdminNetworkPolicyPort describes how to select network ports on pod(s).
/// Exactly one field must be set.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicyIngressPorts {
    /// Port selects a port on a pod(s) based on number.
    /// 
    /// 
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portNumber")]
    pub port_number: Option<BaselineAdminNetworkPolicyIngressPortsPortNumber>,
    /// PortRange selects a port range on a pod(s) based on provided start and end
    /// values.
    /// 
    /// 
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portRange")]
    pub port_range: Option<BaselineAdminNetworkPolicyIngressPortsPortRange>,
}

/// Port selects a port on a pod(s) based on number.
/// 
/// 
/// Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicyIngressPortsPortNumber {
    /// Number defines a network port value.
    /// 
    /// 
    /// Support: Core
    pub port: i32,
    /// Protocol is the network protocol (TCP, UDP, or SCTP) which traffic must
    /// match. If not specified, this field defaults to TCP.
    /// 
    /// 
    /// Support: Core
    pub protocol: String,
}

/// PortRange selects a port range on a pod(s) based on provided start and end
/// values.
/// 
/// 
/// Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicyIngressPortsPortRange {
    /// End defines a network port that is the end of a port range, the End value
    /// must be greater than Start.
    /// 
    /// 
    /// Support: Core
    pub end: i32,
    /// Protocol is the network protocol (TCP, UDP, or SCTP) which traffic must
    /// match. If not specified, this field defaults to TCP.
    /// 
    /// 
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// Start defines a network port that is the start of a port range, the Start
    /// value must be less than End.
    /// 
    /// 
    /// Support: Core
    pub start: i32,
}

/// Subject defines the pods to which this BaselineAdminNetworkPolicy applies.
/// Note that host-networked pods are not included in subject selection.
/// 
/// 
/// Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicySubject {
    /// Namespaces is used to select pods via namespace selectors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<BaselineAdminNetworkPolicySubjectNamespaces>,
    /// Pods is used to select pods via namespace AND pod selectors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pods: Option<BaselineAdminNetworkPolicySubjectPods>,
}

/// Namespaces is used to select pods via namespace selectors.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicySubjectNamespaces {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<BaselineAdminNetworkPolicySubjectNamespacesMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicySubjectNamespacesMatchExpressions {
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

/// Pods is used to select pods via namespace AND pod selectors.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicySubjectPods {
    /// NamespaceSelector follows standard label selector semantics; if empty,
    /// it selects all Namespaces.
    #[serde(rename = "namespaceSelector")]
    pub namespace_selector: BaselineAdminNetworkPolicySubjectPodsNamespaceSelector,
    /// PodSelector is used to explicitly select pods within a namespace; if empty,
    /// it selects all Pods.
    #[serde(rename = "podSelector")]
    pub pod_selector: BaselineAdminNetworkPolicySubjectPodsPodSelector,
}

/// NamespaceSelector follows standard label selector semantics; if empty,
/// it selects all Namespaces.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicySubjectPodsNamespaceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<BaselineAdminNetworkPolicySubjectPodsNamespaceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicySubjectPodsNamespaceSelectorMatchExpressions {
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

/// PodSelector is used to explicitly select pods within a namespace; if empty,
/// it selects all Pods.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicySubjectPodsPodSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<BaselineAdminNetworkPolicySubjectPodsPodSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicySubjectPodsPodSelectorMatchExpressions {
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

/// Status is the status to be reported by the implementation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BaselineAdminNetworkPolicyStatus {
    pub conditions: Vec<Condition>,
}

