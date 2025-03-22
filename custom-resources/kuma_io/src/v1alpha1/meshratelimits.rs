// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kumahq/kuma/kuma.io/v1alpha1/meshratelimits.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// Spec is the specification of the Kuma MeshRateLimit resource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kuma.io", version = "v1alpha1", kind = "MeshRateLimit", plural = "meshratelimits")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct MeshRateLimitSpec {
    /// From list makes a match between clients and corresponding configurations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<MeshRateLimitFrom>>,
    /// Rules defines inbound rate limiting configurations. Currently limited to
    /// selecting all inbound traffic, as L7 matching is not yet implemented.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<MeshRateLimitRules>>,
    /// TargetRef is a reference to the resource the policy takes an effect on.
    /// The resource could be either a real store object or virtual resource
    /// defined inplace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetRef")]
    pub target_ref: Option<MeshRateLimitTargetRef>,
    /// To list makes a match between clients and corresponding configurations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<MeshRateLimitTo>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshRateLimitFrom {
    /// Default is a configuration specific to the group of clients referenced in
    /// 'targetRef'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<MeshRateLimitFromDefault>,
    /// TargetRef is a reference to the resource that represents a group of
    /// clients.
    #[serde(rename = "targetRef")]
    pub target_ref: MeshRateLimitFromTargetRef,
}

/// Default is a configuration specific to the group of clients referenced in
/// 'targetRef'
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitFromDefault {
    /// LocalConf defines local http or/and tcp rate limit configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local: Option<MeshRateLimitFromDefaultLocal>,
}

/// LocalConf defines local http or/and tcp rate limit configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitFromDefaultLocal {
    /// LocalHTTP defines configuration of local HTTP rate limiting
    /// https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/local_rate_limit_filter
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<MeshRateLimitFromDefaultLocalHttp>,
    /// LocalTCP defines confguration of local TCP rate limiting
    /// https://www.envoyproxy.io/docs/envoy/latest/configuration/listeners/network_filters/local_rate_limit_filter
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp: Option<MeshRateLimitFromDefaultLocalTcp>,
}

/// LocalHTTP defines configuration of local HTTP rate limiting
/// https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/local_rate_limit_filter
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitFromDefaultLocalHttp {
    /// Define if rate limiting should be disabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// Describes the actions to take on a rate limit event
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onRateLimit")]
    pub on_rate_limit: Option<MeshRateLimitFromDefaultLocalHttpOnRateLimit>,
    /// Defines how many requests are allowed per interval.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestRate")]
    pub request_rate: Option<MeshRateLimitFromDefaultLocalHttpRequestRate>,
}

/// Describes the actions to take on a rate limit event
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitFromDefaultLocalHttpOnRateLimit {
    /// The Headers to be added to the HTTP response on a rate limit event
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<MeshRateLimitFromDefaultLocalHttpOnRateLimitHeaders>,
    /// The HTTP status code to be set on a rate limit event
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

/// The Headers to be added to the HTTP response on a rate limit event
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitFromDefaultLocalHttpOnRateLimitHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<MeshRateLimitFromDefaultLocalHttpOnRateLimitHeadersAdd>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<Vec<MeshRateLimitFromDefaultLocalHttpOnRateLimitHeadersSet>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitFromDefaultLocalHttpOnRateLimitHeadersAdd {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitFromDefaultLocalHttpOnRateLimitHeadersSet {
    pub name: String,
    pub value: String,
}

/// Defines how many requests are allowed per interval.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitFromDefaultLocalHttpRequestRate {
    /// The interval the number of units is accounted for.
    pub interval: String,
    /// Number of units per interval (depending on usage it can be a number of requests,
    /// or a number of connections).
    pub num: i32,
}

/// LocalTCP defines confguration of local TCP rate limiting
/// https://www.envoyproxy.io/docs/envoy/latest/configuration/listeners/network_filters/local_rate_limit_filter
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitFromDefaultLocalTcp {
    /// Defines how many connections are allowed per interval.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "connectionRate")]
    pub connection_rate: Option<MeshRateLimitFromDefaultLocalTcpConnectionRate>,
    /// Define if rate limiting should be disabled.
    /// Default: false
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

/// Defines how many connections are allowed per interval.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitFromDefaultLocalTcpConnectionRate {
    /// The interval the number of units is accounted for.
    pub interval: String,
    /// Number of units per interval (depending on usage it can be a number of requests,
    /// or a number of connections).
    pub num: i32,
}

/// TargetRef is a reference to the resource that represents a group of
/// clients.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshRateLimitFromTargetRef {
    /// Kind of the referenced resource
    pub kind: MeshRateLimitFromTargetRefKind,
    /// Labels are used to select group of MeshServices that match labels. Either Labels or
    /// Name and Namespace can be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// Mesh is reserved for future use to identify cross mesh resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<String>,
    /// Name of the referenced resource. Can only be used with kinds: `MeshService`,
    /// `MeshServiceSubset` and `MeshGatewayRoute`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace specifies the namespace of target resource. If empty only resources in policy namespace
    /// will be targeted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// ProxyTypes specifies the data plane types that are subject to the policy. When not specified,
    /// all data plane types are targeted by the policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "proxyTypes")]
    pub proxy_types: Option<Vec<String>>,
    /// SectionName is used to target specific section of resource.
    /// For example, you can target port from MeshService.ports[] by its name. Only traffic to this port will be affected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sectionName")]
    pub section_name: Option<String>,
    /// Tags used to select a subset of proxies by tags. Can only be used with kinds
    /// `MeshSubset` and `MeshServiceSubset`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

/// TargetRef is a reference to the resource that represents a group of
/// clients.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshRateLimitFromTargetRefKind {
    Mesh,
    MeshSubset,
    MeshGateway,
    MeshService,
    MeshExternalService,
    MeshMultiZoneService,
    MeshServiceSubset,
    #[serde(rename = "MeshHTTPRoute")]
    MeshHttpRoute,
    Dataplane,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitRules {
    /// Default contains configuration of the inbound rate limits
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<MeshRateLimitRulesDefault>,
}

/// Default contains configuration of the inbound rate limits
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitRulesDefault {
    /// LocalConf defines local http or/and tcp rate limit configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local: Option<MeshRateLimitRulesDefaultLocal>,
}

/// LocalConf defines local http or/and tcp rate limit configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitRulesDefaultLocal {
    /// LocalHTTP defines configuration of local HTTP rate limiting
    /// https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/local_rate_limit_filter
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<MeshRateLimitRulesDefaultLocalHttp>,
    /// LocalTCP defines confguration of local TCP rate limiting
    /// https://www.envoyproxy.io/docs/envoy/latest/configuration/listeners/network_filters/local_rate_limit_filter
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp: Option<MeshRateLimitRulesDefaultLocalTcp>,
}

/// LocalHTTP defines configuration of local HTTP rate limiting
/// https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/local_rate_limit_filter
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitRulesDefaultLocalHttp {
    /// Define if rate limiting should be disabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// Describes the actions to take on a rate limit event
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onRateLimit")]
    pub on_rate_limit: Option<MeshRateLimitRulesDefaultLocalHttpOnRateLimit>,
    /// Defines how many requests are allowed per interval.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestRate")]
    pub request_rate: Option<MeshRateLimitRulesDefaultLocalHttpRequestRate>,
}

/// Describes the actions to take on a rate limit event
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitRulesDefaultLocalHttpOnRateLimit {
    /// The Headers to be added to the HTTP response on a rate limit event
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<MeshRateLimitRulesDefaultLocalHttpOnRateLimitHeaders>,
    /// The HTTP status code to be set on a rate limit event
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

/// The Headers to be added to the HTTP response on a rate limit event
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitRulesDefaultLocalHttpOnRateLimitHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<MeshRateLimitRulesDefaultLocalHttpOnRateLimitHeadersAdd>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<Vec<MeshRateLimitRulesDefaultLocalHttpOnRateLimitHeadersSet>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitRulesDefaultLocalHttpOnRateLimitHeadersAdd {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitRulesDefaultLocalHttpOnRateLimitHeadersSet {
    pub name: String,
    pub value: String,
}

/// Defines how many requests are allowed per interval.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitRulesDefaultLocalHttpRequestRate {
    /// The interval the number of units is accounted for.
    pub interval: String,
    /// Number of units per interval (depending on usage it can be a number of requests,
    /// or a number of connections).
    pub num: i32,
}

/// LocalTCP defines confguration of local TCP rate limiting
/// https://www.envoyproxy.io/docs/envoy/latest/configuration/listeners/network_filters/local_rate_limit_filter
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitRulesDefaultLocalTcp {
    /// Defines how many connections are allowed per interval.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "connectionRate")]
    pub connection_rate: Option<MeshRateLimitRulesDefaultLocalTcpConnectionRate>,
    /// Define if rate limiting should be disabled.
    /// Default: false
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

/// Defines how many connections are allowed per interval.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitRulesDefaultLocalTcpConnectionRate {
    /// The interval the number of units is accounted for.
    pub interval: String,
    /// Number of units per interval (depending on usage it can be a number of requests,
    /// or a number of connections).
    pub num: i32,
}

/// TargetRef is a reference to the resource the policy takes an effect on.
/// The resource could be either a real store object or virtual resource
/// defined inplace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshRateLimitTargetRef {
    /// Kind of the referenced resource
    pub kind: MeshRateLimitTargetRefKind,
    /// Labels are used to select group of MeshServices that match labels. Either Labels or
    /// Name and Namespace can be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// Mesh is reserved for future use to identify cross mesh resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<String>,
    /// Name of the referenced resource. Can only be used with kinds: `MeshService`,
    /// `MeshServiceSubset` and `MeshGatewayRoute`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace specifies the namespace of target resource. If empty only resources in policy namespace
    /// will be targeted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// ProxyTypes specifies the data plane types that are subject to the policy. When not specified,
    /// all data plane types are targeted by the policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "proxyTypes")]
    pub proxy_types: Option<Vec<String>>,
    /// SectionName is used to target specific section of resource.
    /// For example, you can target port from MeshService.ports[] by its name. Only traffic to this port will be affected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sectionName")]
    pub section_name: Option<String>,
    /// Tags used to select a subset of proxies by tags. Can only be used with kinds
    /// `MeshSubset` and `MeshServiceSubset`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

/// TargetRef is a reference to the resource the policy takes an effect on.
/// The resource could be either a real store object or virtual resource
/// defined inplace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshRateLimitTargetRefKind {
    Mesh,
    MeshSubset,
    MeshGateway,
    MeshService,
    MeshExternalService,
    MeshMultiZoneService,
    MeshServiceSubset,
    #[serde(rename = "MeshHTTPRoute")]
    MeshHttpRoute,
    Dataplane,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshRateLimitTo {
    /// Default is a configuration specific to the group of clients referenced in
    /// 'targetRef'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<MeshRateLimitToDefault>,
    /// TargetRef is a reference to the resource that represents a group of
    /// clients.
    #[serde(rename = "targetRef")]
    pub target_ref: MeshRateLimitToTargetRef,
}

/// Default is a configuration specific to the group of clients referenced in
/// 'targetRef'
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitToDefault {
    /// LocalConf defines local http or/and tcp rate limit configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local: Option<MeshRateLimitToDefaultLocal>,
}

/// LocalConf defines local http or/and tcp rate limit configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitToDefaultLocal {
    /// LocalHTTP defines configuration of local HTTP rate limiting
    /// https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/local_rate_limit_filter
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<MeshRateLimitToDefaultLocalHttp>,
    /// LocalTCP defines confguration of local TCP rate limiting
    /// https://www.envoyproxy.io/docs/envoy/latest/configuration/listeners/network_filters/local_rate_limit_filter
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp: Option<MeshRateLimitToDefaultLocalTcp>,
}

/// LocalHTTP defines configuration of local HTTP rate limiting
/// https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/local_rate_limit_filter
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitToDefaultLocalHttp {
    /// Define if rate limiting should be disabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// Describes the actions to take on a rate limit event
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onRateLimit")]
    pub on_rate_limit: Option<MeshRateLimitToDefaultLocalHttpOnRateLimit>,
    /// Defines how many requests are allowed per interval.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestRate")]
    pub request_rate: Option<MeshRateLimitToDefaultLocalHttpRequestRate>,
}

/// Describes the actions to take on a rate limit event
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitToDefaultLocalHttpOnRateLimit {
    /// The Headers to be added to the HTTP response on a rate limit event
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<MeshRateLimitToDefaultLocalHttpOnRateLimitHeaders>,
    /// The HTTP status code to be set on a rate limit event
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

/// The Headers to be added to the HTTP response on a rate limit event
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitToDefaultLocalHttpOnRateLimitHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<MeshRateLimitToDefaultLocalHttpOnRateLimitHeadersAdd>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<Vec<MeshRateLimitToDefaultLocalHttpOnRateLimitHeadersSet>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitToDefaultLocalHttpOnRateLimitHeadersAdd {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitToDefaultLocalHttpOnRateLimitHeadersSet {
    pub name: String,
    pub value: String,
}

/// Defines how many requests are allowed per interval.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitToDefaultLocalHttpRequestRate {
    /// The interval the number of units is accounted for.
    pub interval: String,
    /// Number of units per interval (depending on usage it can be a number of requests,
    /// or a number of connections).
    pub num: i32,
}

/// LocalTCP defines confguration of local TCP rate limiting
/// https://www.envoyproxy.io/docs/envoy/latest/configuration/listeners/network_filters/local_rate_limit_filter
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitToDefaultLocalTcp {
    /// Defines how many connections are allowed per interval.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "connectionRate")]
    pub connection_rate: Option<MeshRateLimitToDefaultLocalTcpConnectionRate>,
    /// Define if rate limiting should be disabled.
    /// Default: false
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

/// Defines how many connections are allowed per interval.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshRateLimitToDefaultLocalTcpConnectionRate {
    /// The interval the number of units is accounted for.
    pub interval: String,
    /// Number of units per interval (depending on usage it can be a number of requests,
    /// or a number of connections).
    pub num: i32,
}

/// TargetRef is a reference to the resource that represents a group of
/// clients.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshRateLimitToTargetRef {
    /// Kind of the referenced resource
    pub kind: MeshRateLimitToTargetRefKind,
    /// Labels are used to select group of MeshServices that match labels. Either Labels or
    /// Name and Namespace can be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// Mesh is reserved for future use to identify cross mesh resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<String>,
    /// Name of the referenced resource. Can only be used with kinds: `MeshService`,
    /// `MeshServiceSubset` and `MeshGatewayRoute`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace specifies the namespace of target resource. If empty only resources in policy namespace
    /// will be targeted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// ProxyTypes specifies the data plane types that are subject to the policy. When not specified,
    /// all data plane types are targeted by the policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "proxyTypes")]
    pub proxy_types: Option<Vec<String>>,
    /// SectionName is used to target specific section of resource.
    /// For example, you can target port from MeshService.ports[] by its name. Only traffic to this port will be affected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sectionName")]
    pub section_name: Option<String>,
    /// Tags used to select a subset of proxies by tags. Can only be used with kinds
    /// `MeshSubset` and `MeshServiceSubset`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

/// TargetRef is a reference to the resource that represents a group of
/// clients.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshRateLimitToTargetRefKind {
    Mesh,
    MeshSubset,
    MeshGateway,
    MeshService,
    MeshExternalService,
    MeshMultiZoneService,
    MeshServiceSubset,
    #[serde(rename = "MeshHTTPRoute")]
    MeshHttpRoute,
    Dataplane,
}

