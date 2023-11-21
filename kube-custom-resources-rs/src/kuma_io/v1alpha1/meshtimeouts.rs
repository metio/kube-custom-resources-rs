// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kumahq/kuma/kuma.io/v1alpha1/meshtimeouts.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// Spec is the specification of the Kuma MeshTimeout resource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kuma.io", version = "v1alpha1", kind = "MeshTimeout", plural = "meshtimeouts")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct MeshTimeoutSpec {
    /// From list makes a match between clients and corresponding configurations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<MeshTimeoutFrom>>,
    /// TargetRef is a reference to the resource the policy takes an effect on. The resource could be either a real store object or virtual resource defined inplace.
    #[serde(rename = "targetRef")]
    pub target_ref: MeshTimeoutTargetRef,
    /// To list makes a match between the consumed services and corresponding configurations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<MeshTimeoutTo>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshTimeoutFrom {
    /// Default is a configuration specific to the group of clients referenced in 'targetRef'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<MeshTimeoutFromDefault>,
    /// TargetRef is a reference to the resource that represents a group of clients.
    #[serde(rename = "targetRef")]
    pub target_ref: MeshTimeoutFromTargetRef,
}

/// Default is a configuration specific to the group of clients referenced in 'targetRef'
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshTimeoutFromDefault {
    /// ConnectionTimeout specifies the amount of time proxy will wait for an TCP connection to be established. Default value is 5 seconds. Cannot be set to 0.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "connectionTimeout")]
    pub connection_timeout: Option<String>,
    /// Http provides configuration for HTTP specific timeouts
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<MeshTimeoutFromDefaultHttp>,
    /// IdleTimeout is defined as the period in which there are no bytes sent or received on connection Setting this timeout to 0 will disable it. Be cautious when disabling it because it can lead to connection leaking. Default value is 1h.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "idleTimeout")]
    pub idle_timeout: Option<String>,
}

/// Http provides configuration for HTTP specific timeouts
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshTimeoutFromDefaultHttp {
    /// MaxConnectionDuration is the time after which a connection will be drained and/or closed, starting from when it was first established. Setting this timeout to 0 will disable it. Disabled by default.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxConnectionDuration")]
    pub max_connection_duration: Option<String>,
    /// MaxStreamDuration is the maximum time that a stream’s lifetime will span. Setting this timeout to 0 will disable it. Disabled by default.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxStreamDuration")]
    pub max_stream_duration: Option<String>,
    /// RequestTimeout The amount of time that proxy will wait for the entire request to be received. The timer is activated when the request is initiated, and is disarmed when the last byte of the request is sent, OR when the response is initiated. Setting this timeout to 0 will disable it. Default is 15s.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestTimeout")]
    pub request_timeout: Option<String>,
    /// StreamIdleTimeout is the amount of time that proxy will allow a stream to exist with no activity. Setting this timeout to 0 will disable it. Default is 30m
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "streamIdleTimeout")]
    pub stream_idle_timeout: Option<String>,
}

/// TargetRef is a reference to the resource that represents a group of clients.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshTimeoutFromTargetRef {
    /// Kind of the referenced resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<MeshTimeoutFromTargetRefKind>,
    /// Mesh is reserved for future use to identify cross mesh resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<String>,
    /// Name of the referenced resource. Can only be used with kinds: `MeshService`, `MeshServiceSubset` and `MeshGatewayRoute`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Tags used to select a subset of proxies by tags. Can only be used with kinds `MeshSubset` and `MeshServiceSubset`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

/// TargetRef is a reference to the resource that represents a group of clients.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshTimeoutFromTargetRefKind {
    Mesh,
    MeshSubset,
    MeshGateway,
    MeshService,
    MeshServiceSubset,
    #[serde(rename = "MeshHTTPRoute")]
    MeshHttpRoute,
}

/// TargetRef is a reference to the resource the policy takes an effect on. The resource could be either a real store object or virtual resource defined inplace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshTimeoutTargetRef {
    /// Kind of the referenced resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<MeshTimeoutTargetRefKind>,
    /// Mesh is reserved for future use to identify cross mesh resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<String>,
    /// Name of the referenced resource. Can only be used with kinds: `MeshService`, `MeshServiceSubset` and `MeshGatewayRoute`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Tags used to select a subset of proxies by tags. Can only be used with kinds `MeshSubset` and `MeshServiceSubset`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

/// TargetRef is a reference to the resource the policy takes an effect on. The resource could be either a real store object or virtual resource defined inplace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshTimeoutTargetRefKind {
    Mesh,
    MeshSubset,
    MeshGateway,
    MeshService,
    MeshServiceSubset,
    #[serde(rename = "MeshHTTPRoute")]
    MeshHttpRoute,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshTimeoutTo {
    /// Default is a configuration specific to the group of destinations referenced in 'targetRef'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<MeshTimeoutToDefault>,
    /// TargetRef is a reference to the resource that represents a group of destinations.
    #[serde(rename = "targetRef")]
    pub target_ref: MeshTimeoutToTargetRef,
}

/// Default is a configuration specific to the group of destinations referenced in 'targetRef'
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshTimeoutToDefault {
    /// ConnectionTimeout specifies the amount of time proxy will wait for an TCP connection to be established. Default value is 5 seconds. Cannot be set to 0.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "connectionTimeout")]
    pub connection_timeout: Option<String>,
    /// Http provides configuration for HTTP specific timeouts
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<MeshTimeoutToDefaultHttp>,
    /// IdleTimeout is defined as the period in which there are no bytes sent or received on connection Setting this timeout to 0 will disable it. Be cautious when disabling it because it can lead to connection leaking. Default value is 1h.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "idleTimeout")]
    pub idle_timeout: Option<String>,
}

/// Http provides configuration for HTTP specific timeouts
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshTimeoutToDefaultHttp {
    /// MaxConnectionDuration is the time after which a connection will be drained and/or closed, starting from when it was first established. Setting this timeout to 0 will disable it. Disabled by default.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxConnectionDuration")]
    pub max_connection_duration: Option<String>,
    /// MaxStreamDuration is the maximum time that a stream’s lifetime will span. Setting this timeout to 0 will disable it. Disabled by default.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxStreamDuration")]
    pub max_stream_duration: Option<String>,
    /// RequestTimeout The amount of time that proxy will wait for the entire request to be received. The timer is activated when the request is initiated, and is disarmed when the last byte of the request is sent, OR when the response is initiated. Setting this timeout to 0 will disable it. Default is 15s.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestTimeout")]
    pub request_timeout: Option<String>,
    /// StreamIdleTimeout is the amount of time that proxy will allow a stream to exist with no activity. Setting this timeout to 0 will disable it. Default is 30m
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "streamIdleTimeout")]
    pub stream_idle_timeout: Option<String>,
}

/// TargetRef is a reference to the resource that represents a group of destinations.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshTimeoutToTargetRef {
    /// Kind of the referenced resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<MeshTimeoutToTargetRefKind>,
    /// Mesh is reserved for future use to identify cross mesh resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<String>,
    /// Name of the referenced resource. Can only be used with kinds: `MeshService`, `MeshServiceSubset` and `MeshGatewayRoute`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Tags used to select a subset of proxies by tags. Can only be used with kinds `MeshSubset` and `MeshServiceSubset`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

/// TargetRef is a reference to the resource that represents a group of destinations.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshTimeoutToTargetRefKind {
    Mesh,
    MeshSubset,
    MeshGateway,
    MeshService,
    MeshServiceSubset,
    #[serde(rename = "MeshHTTPRoute")]
    MeshHttpRoute,
}
