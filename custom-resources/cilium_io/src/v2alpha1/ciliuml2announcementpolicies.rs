// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/cilium/cilium/cilium.io/v2alpha1/ciliuml2announcementpolicies.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// Spec is a human readable description of a L2 announcement policy
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "cilium.io", version = "v2alpha1", kind = "CiliumL2AnnouncementPolicy", plural = "ciliuml2announcementpolicies")]
#[kube(status = "CiliumL2AnnouncementPolicyStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct CiliumL2AnnouncementPolicySpec {
    /// If true, the external IPs of the services are announced
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalIPs")]
    pub external_i_ps: Option<bool>,
    /// A list of regular expressions that express which network interface(s) should be used
    /// to announce the services over. If nil, all network interfaces are used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interfaces: Option<Vec<String>>,
    /// If true, the loadbalancer IPs of the services are announced
    /// 
    /// If nil this policy applies to all services.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "loadBalancerIPs")]
    pub load_balancer_i_ps: Option<bool>,
    /// NodeSelector selects a group of nodes which will announce the IPs for
    /// the services selected by the service selector.
    /// 
    /// If nil this policy applies to all nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<CiliumL2AnnouncementPolicyNodeSelector>,
    /// ServiceSelector selects a set of services which will be announced over L2 networks.
    /// The loadBalancerClass for a service must be nil or specify a supported class, e.g.
    /// "io.cilium/l2-announcer". Refer to the following document for additional details
    /// regarding load balancer classes:
    /// 
    ///   https://kubernetes.io/docs/concepts/services-networking/service/#load-balancer-class
    /// 
    /// If nil this policy applies to all services.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceSelector")]
    pub service_selector: Option<CiliumL2AnnouncementPolicyServiceSelector>,
}

/// NodeSelector selects a group of nodes which will announce the IPs for
/// the services selected by the service selector.
/// 
/// If nil this policy applies to all nodes.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CiliumL2AnnouncementPolicyNodeSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CiliumL2AnnouncementPolicyNodeSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CiliumL2AnnouncementPolicyNodeSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: CiliumL2AnnouncementPolicyNodeSelectorMatchExpressionsOperator,
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
pub enum CiliumL2AnnouncementPolicyNodeSelectorMatchExpressionsOperator {
    In,
    NotIn,
    Exists,
    DoesNotExist,
}

/// ServiceSelector selects a set of services which will be announced over L2 networks.
/// The loadBalancerClass for a service must be nil or specify a supported class, e.g.
/// "io.cilium/l2-announcer". Refer to the following document for additional details
/// regarding load balancer classes:
/// 
///   https://kubernetes.io/docs/concepts/services-networking/service/#load-balancer-class
/// 
/// If nil this policy applies to all services.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CiliumL2AnnouncementPolicyServiceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CiliumL2AnnouncementPolicyServiceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CiliumL2AnnouncementPolicyServiceSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: CiliumL2AnnouncementPolicyServiceSelectorMatchExpressionsOperator,
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
pub enum CiliumL2AnnouncementPolicyServiceSelectorMatchExpressionsOperator {
    In,
    NotIn,
    Exists,
    DoesNotExist,
}

/// Status is the status of the policy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CiliumL2AnnouncementPolicyStatus {
    /// Current service state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

