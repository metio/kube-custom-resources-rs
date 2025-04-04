// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kumahq/kuma/kuma.io/v1alpha1/meshtimeouts.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// Spec is the specification of the Kuma MeshTimeout resource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kuma.io", version = "v1alpha1", kind = "MeshTimeout", plural = "meshtimeouts")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct MeshTimeoutSpec {
    /// From list makes a match between clients and corresponding configurations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<MeshTimeoutFrom>>,
    /// Rules defines inbound timeout configurations. Currently limited to exactly one rule containing
    /// default timeouts that apply to all inbound traffic, as L7 matching is not yet implemented.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<MeshTimeoutRules>>,
    /// TargetRef is a reference to the resource the policy takes an effect on.
    /// The resource could be either a real store object or virtual resource
    /// defined inplace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetRef")]
    pub target_ref: Option<MeshTimeoutTargetRef>,
    /// To list makes a match between the consumed services and corresponding configurations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<MeshTimeoutTo>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshTimeoutFrom {
    /// Default is a configuration specific to the group of clients referenced in
    /// 'targetRef'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<MeshTimeoutFromDefault>,
    /// TargetRef is a reference to the resource that represents a group of
    /// clients.
    #[serde(rename = "targetRef")]
    pub target_ref: MeshTimeoutFromTargetRef,
}

/// Default is a configuration specific to the group of clients referenced in
/// 'targetRef'
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshTimeoutFromDefault {
    /// ConnectionTimeout specifies the amount of time proxy will wait for an TCP connection to be established.
    /// Default value is 5 seconds. Cannot be set to 0.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "connectionTimeout")]
    pub connection_timeout: Option<String>,
    /// Http provides configuration for HTTP specific timeouts
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<MeshTimeoutFromDefaultHttp>,
    /// IdleTimeout is defined as the period in which there are no bytes sent or received on connection
    /// Setting this timeout to 0 will disable it. Be cautious when disabling it because
    /// it can lead to connection leaking. Default value is 1h.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "idleTimeout")]
    pub idle_timeout: Option<String>,
}

/// Http provides configuration for HTTP specific timeouts
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshTimeoutFromDefaultHttp {
    /// MaxConnectionDuration is the time after which a connection will be drained and/or closed,
    /// starting from when it was first established. Setting this timeout to 0 will disable it.
    /// Disabled by default.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxConnectionDuration")]
    pub max_connection_duration: Option<String>,
    /// MaxStreamDuration is the maximum time that a stream’s lifetime will span.
    /// Setting this timeout to 0 will disable it. Disabled by default.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxStreamDuration")]
    pub max_stream_duration: Option<String>,
    /// RequestHeadersTimeout The amount of time that proxy will wait for the request headers to be received. The timer is
    /// activated when the first byte of the headers is received, and is disarmed when the last byte of
    /// the headers has been received. If not specified or set to 0, this timeout is disabled.
    /// Disabled by default.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestHeadersTimeout")]
    pub request_headers_timeout: Option<String>,
    /// RequestTimeout The amount of time that proxy will wait for the entire request to be received.
    /// The timer is activated when the request is initiated, and is disarmed when the last byte of the request is sent,
    /// OR when the response is initiated. Setting this timeout to 0 will disable it.
    /// Default is 15s.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestTimeout")]
    pub request_timeout: Option<String>,
    /// StreamIdleTimeout is the amount of time that proxy will allow a stream to exist with no activity.
    /// Setting this timeout to 0 will disable it. Default is 30m
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "streamIdleTimeout")]
    pub stream_idle_timeout: Option<String>,
}

/// TargetRef is a reference to the resource that represents a group of
/// clients.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshTimeoutFromTargetRef {
    /// Kind of the referenced resource
    pub kind: MeshTimeoutFromTargetRefKind,
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
pub enum MeshTimeoutFromTargetRefKind {
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
pub struct MeshTimeoutRules {
    /// Default contains configuration of the inbound timeouts
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<MeshTimeoutRulesDefault>,
}

/// Default contains configuration of the inbound timeouts
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshTimeoutRulesDefault {
    /// ConnectionTimeout specifies the amount of time proxy will wait for an TCP connection to be established.
    /// Default value is 5 seconds. Cannot be set to 0.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "connectionTimeout")]
    pub connection_timeout: Option<String>,
    /// Http provides configuration for HTTP specific timeouts
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<MeshTimeoutRulesDefaultHttp>,
    /// IdleTimeout is defined as the period in which there are no bytes sent or received on connection
    /// Setting this timeout to 0 will disable it. Be cautious when disabling it because
    /// it can lead to connection leaking. Default value is 1h.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "idleTimeout")]
    pub idle_timeout: Option<String>,
}

/// Http provides configuration for HTTP specific timeouts
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshTimeoutRulesDefaultHttp {
    /// MaxConnectionDuration is the time after which a connection will be drained and/or closed,
    /// starting from when it was first established. Setting this timeout to 0 will disable it.
    /// Disabled by default.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxConnectionDuration")]
    pub max_connection_duration: Option<String>,
    /// MaxStreamDuration is the maximum time that a stream’s lifetime will span.
    /// Setting this timeout to 0 will disable it. Disabled by default.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxStreamDuration")]
    pub max_stream_duration: Option<String>,
    /// RequestHeadersTimeout The amount of time that proxy will wait for the request headers to be received. The timer is
    /// activated when the first byte of the headers is received, and is disarmed when the last byte of
    /// the headers has been received. If not specified or set to 0, this timeout is disabled.
    /// Disabled by default.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestHeadersTimeout")]
    pub request_headers_timeout: Option<String>,
    /// RequestTimeout The amount of time that proxy will wait for the entire request to be received.
    /// The timer is activated when the request is initiated, and is disarmed when the last byte of the request is sent,
    /// OR when the response is initiated. Setting this timeout to 0 will disable it.
    /// Default is 15s.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestTimeout")]
    pub request_timeout: Option<String>,
    /// StreamIdleTimeout is the amount of time that proxy will allow a stream to exist with no activity.
    /// Setting this timeout to 0 will disable it. Default is 30m
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "streamIdleTimeout")]
    pub stream_idle_timeout: Option<String>,
}

/// TargetRef is a reference to the resource the policy takes an effect on.
/// The resource could be either a real store object or virtual resource
/// defined inplace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshTimeoutTargetRef {
    /// Kind of the referenced resource
    pub kind: MeshTimeoutTargetRefKind,
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
pub enum MeshTimeoutTargetRefKind {
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
pub struct MeshTimeoutTo {
    /// Default is a configuration specific to the group of destinations referenced in
    /// 'targetRef'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<MeshTimeoutToDefault>,
    /// TargetRef is a reference to the resource that represents a group of
    /// destinations.
    #[serde(rename = "targetRef")]
    pub target_ref: MeshTimeoutToTargetRef,
}

/// Default is a configuration specific to the group of destinations referenced in
/// 'targetRef'
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshTimeoutToDefault {
    /// ConnectionTimeout specifies the amount of time proxy will wait for an TCP connection to be established.
    /// Default value is 5 seconds. Cannot be set to 0.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "connectionTimeout")]
    pub connection_timeout: Option<String>,
    /// Http provides configuration for HTTP specific timeouts
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<MeshTimeoutToDefaultHttp>,
    /// IdleTimeout is defined as the period in which there are no bytes sent or received on connection
    /// Setting this timeout to 0 will disable it. Be cautious when disabling it because
    /// it can lead to connection leaking. Default value is 1h.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "idleTimeout")]
    pub idle_timeout: Option<String>,
}

/// Http provides configuration for HTTP specific timeouts
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshTimeoutToDefaultHttp {
    /// MaxConnectionDuration is the time after which a connection will be drained and/or closed,
    /// starting from when it was first established. Setting this timeout to 0 will disable it.
    /// Disabled by default.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxConnectionDuration")]
    pub max_connection_duration: Option<String>,
    /// MaxStreamDuration is the maximum time that a stream’s lifetime will span.
    /// Setting this timeout to 0 will disable it. Disabled by default.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxStreamDuration")]
    pub max_stream_duration: Option<String>,
    /// RequestHeadersTimeout The amount of time that proxy will wait for the request headers to be received. The timer is
    /// activated when the first byte of the headers is received, and is disarmed when the last byte of
    /// the headers has been received. If not specified or set to 0, this timeout is disabled.
    /// Disabled by default.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestHeadersTimeout")]
    pub request_headers_timeout: Option<String>,
    /// RequestTimeout The amount of time that proxy will wait for the entire request to be received.
    /// The timer is activated when the request is initiated, and is disarmed when the last byte of the request is sent,
    /// OR when the response is initiated. Setting this timeout to 0 will disable it.
    /// Default is 15s.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestTimeout")]
    pub request_timeout: Option<String>,
    /// StreamIdleTimeout is the amount of time that proxy will allow a stream to exist with no activity.
    /// Setting this timeout to 0 will disable it. Default is 30m
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "streamIdleTimeout")]
    pub stream_idle_timeout: Option<String>,
}

/// TargetRef is a reference to the resource that represents a group of
/// destinations.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshTimeoutToTargetRef {
    /// Kind of the referenced resource
    pub kind: MeshTimeoutToTargetRefKind,
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
/// destinations.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshTimeoutToTargetRefKind {
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

