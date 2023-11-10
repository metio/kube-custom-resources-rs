// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/emissary-ingress/emissary/getambassador.io/v3alpha1/listeners.yaml --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// ListenerSpec defines the desired state of this Port
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "getambassador.io", version = "v3alpha1", kind = "Listener", plural = "listeners")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct ListenerSpec {
    /// AmbassadorID declares which Ambassador instances should pay attention to this resource. If no value is provided, the default is: 
    ///  ambassador_id: - "default"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ambassador_id: Option<Vec<String>>,
    /// HostBinding allows restricting which Hosts will be used for this Listener.
    #[serde(rename = "hostBinding")]
    pub host_binding: ListenerHostBinding,
    /// L7Depth specifies how many layer 7 load balancers are between us and the edge of the network.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "l7Depth")]
    pub l7_depth: Option<i32>,
    /// Port is the network port. Only one Listener can use a given port.
    pub port: i32,
    /// Protocol is a shorthand for certain predefined stacks. Exactly one of Protocol or ProtocolStack must be supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<ListenerProtocol>,
    /// ProtocolStack explicitly specifies the protocol stack to set up. Exactly one of Protocol or ProtocolStack must be supplied.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "protocolStack")]
    pub protocol_stack: Option<Vec<String>>,
    /// SecurityModel specifies how to determine whether connections to this port are secure or insecure.
    #[serde(rename = "securityModel")]
    pub security_model: ListenerSecurityModel,
    /// StatsPrefix specifies the prefix for statistics sent by Envoy about this Listener. The default depends on the protocol: "ingress-http", "ingress-https", "ingress-tls-$port", or "ingress-$port".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statsPrefix")]
    pub stats_prefix: Option<String>,
}

/// HostBinding allows restricting which Hosts will be used for this Listener.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ListenerHostBinding {
    /// NamespaceBindingType defines we we specify which namespaces to look for Hosts in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<ListenerHostBindingNamespace>,
    /// A label selector is a label query over a set of resources. The result of matchLabels and matchExpressions are ANDed. An empty label selector matches all objects. A null label selector matches no objects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<ListenerHostBindingSelector>,
}

/// NamespaceBindingType defines we we specify which namespaces to look for Hosts in.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ListenerHostBindingNamespace {
    /// NamespaceFromType defines how we evaluate a NamespaceBindingType.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<ListenerHostBindingNamespaceFrom>,
}

/// NamespaceBindingType defines we we specify which namespaces to look for Hosts in.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ListenerHostBindingNamespaceFrom {
    #[serde(rename = "SELF")]
    r#_Self,
    #[serde(rename = "ALL")]
    All,
    #[serde(rename = "SELECTOR")]
    Selector,
}

/// A label selector is a label query over a set of resources. The result of matchLabels and matchExpressions are ANDed. An empty label selector matches all objects. A null label selector matches no objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ListenerHostBindingSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ListenerHostBindingSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ListenerHostBindingSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// ListenerSpec defines the desired state of this Port
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ListenerProtocol {
    #[serde(rename = "HTTP")]
    Http,
    #[serde(rename = "HTTPS")]
    Https,
    #[serde(rename = "HTTPPROXY")]
    Httpproxy,
    #[serde(rename = "HTTPSPROXY")]
    Httpsproxy,
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "TLS")]
    Tls,
    #[serde(rename = "UDP")]
    Udp,
}

/// ListenerSpec defines the desired state of this Port
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ListenerSecurityModel {
    #[serde(rename = "XFP")]
    Xfp,
    #[serde(rename = "SECURE")]
    Secure,
    #[serde(rename = "INSECURE")]
    Insecure,
}

