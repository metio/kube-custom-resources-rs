// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/nginxinc/kubernetes-ingress/k8s.nginx.org/v1/transportservers.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// TransportServerSpec is the spec of the TransportServer resource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "k8s.nginx.org", version = "v1", kind = "TransportServer", plural = "transportservers")]
#[kube(namespaced)]
#[kube(status = "TransportServerStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct TransportServerSpec {
    /// TransportServerAction defines an action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<TransportServerAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ingressClassName")]
    pub ingress_class_name: Option<String>,
    /// TransportServerListener defines a listener for a TransportServer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listener: Option<TransportServerListener>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serverSnippets")]
    pub server_snippets: Option<String>,
    /// SessionParameters defines session parameters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionParameters")]
    pub session_parameters: Option<TransportServerSessionParameters>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "streamSnippets")]
    pub stream_snippets: Option<String>,
    /// TransportServerTLS defines TransportServerTLS configuration for a TransportServer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<TransportServerTls>,
    /// UpstreamParameters defines parameters for an upstream.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "upstreamParameters")]
    pub upstream_parameters: Option<TransportServerUpstreamParameters>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstreams: Option<Vec<TransportServerUpstreams>>,
}

/// TransportServerAction defines an action.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransportServerAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pass: Option<String>,
}

/// TransportServerListener defines a listener for a TransportServer.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransportServerListener {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

/// SessionParameters defines session parameters.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransportServerSessionParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// TransportServerTLS defines TransportServerTLS configuration for a TransportServer.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransportServerTls {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

/// UpstreamParameters defines parameters for an upstream.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransportServerUpstreamParameters {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "connectTimeout")]
    pub connect_timeout: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nextUpstream")]
    pub next_upstream: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nextUpstreamTimeout")]
    pub next_upstream_timeout: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nextUpstreamTries")]
    pub next_upstream_tries: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "udpRequests")]
    pub udp_requests: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "udpResponses")]
    pub udp_responses: Option<i64>,
}

/// TransportServerUpstream defines an upstream.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransportServerUpstreams {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupPort")]
    pub backup_port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failTimeout")]
    pub fail_timeout: Option<String>,
    /// TransportServerHealthCheck defines the parameters for active Upstream HealthChecks.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "healthCheck")]
    pub health_check: Option<TransportServerUpstreamsHealthCheck>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "loadBalancingMethod")]
    pub load_balancing_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxConns")]
    pub max_conns: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxFails")]
    pub max_fails: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

/// TransportServerHealthCheck defines the parameters for active Upstream HealthChecks.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransportServerUpstreamsHealthCheck {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fails: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jitter: Option<String>,
    /// TransportServerMatch defines the parameters of a custom health check.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<TransportServerUpstreamsHealthCheckMatch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub passes: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// TransportServerMatch defines the parameters of a custom health check.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransportServerUpstreamsHealthCheckMatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expect: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send: Option<String>,
}

/// TransportServerStatus defines the status for the TransportServer resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransportServerStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

