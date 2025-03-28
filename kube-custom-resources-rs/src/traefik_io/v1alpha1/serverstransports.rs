// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/traefik/traefik/traefik.io/v1alpha1/serverstransports.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
}
use self::prelude::*;

/// ServersTransportSpec defines the desired state of a ServersTransport.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "traefik.io", version = "v1alpha1", kind = "ServersTransport", plural = "serverstransports")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ServersTransportSpec {
    /// CertificatesSecrets defines a list of secret storing client certificates for mTLS.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certificatesSecrets")]
    pub certificates_secrets: Option<Vec<String>>,
    /// DisableHTTP2 disables HTTP/2 for connections with backend servers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disableHTTP2")]
    pub disable_http2: Option<bool>,
    /// ForwardingTimeouts defines the timeouts for requests forwarded to the backend servers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "forwardingTimeouts")]
    pub forwarding_timeouts: Option<ServersTransportForwardingTimeouts>,
    /// InsecureSkipVerify disables SSL certificate verification.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "insecureSkipVerify")]
    pub insecure_skip_verify: Option<bool>,
    /// MaxIdleConnsPerHost controls the maximum idle (keep-alive) to keep per-host.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxIdleConnsPerHost")]
    pub max_idle_conns_per_host: Option<i64>,
    /// PeerCertURI defines the peer cert URI used to match against SAN URI during the peer certificate verification.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "peerCertURI")]
    pub peer_cert_uri: Option<String>,
    /// RootCAs defines a list of CA certificate Secrets or ConfigMaps used to validate server certificates.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rootCAs")]
    pub root_c_as: Option<Vec<ServersTransportRootCAs>>,
    /// RootCAsSecrets defines a list of CA secret used to validate self-signed certificate.
    /// Deprecated: RootCAsSecrets is deprecated, please use the RootCAs option instead.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rootCAsSecrets")]
    pub root_c_as_secrets: Option<Vec<String>>,
    /// ServerName defines the server name used to contact the server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serverName")]
    pub server_name: Option<String>,
    /// Spiffe defines the SPIFFE configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spiffe: Option<ServersTransportSpiffe>,
}

/// ForwardingTimeouts defines the timeouts for requests forwarded to the backend servers.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServersTransportForwardingTimeouts {
    /// DialTimeout is the amount of time to wait until a connection to a backend server can be established.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dialTimeout")]
    pub dial_timeout: Option<IntOrString>,
    /// IdleConnTimeout is the maximum period for which an idle HTTP keep-alive connection will remain open before closing itself.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "idleConnTimeout")]
    pub idle_conn_timeout: Option<IntOrString>,
    /// PingTimeout is the timeout after which the HTTP/2 connection will be closed if a response to ping is not received.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pingTimeout")]
    pub ping_timeout: Option<IntOrString>,
    /// ReadIdleTimeout is the timeout after which a health check using ping frame will be carried out if no frame is received on the HTTP/2 connection.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readIdleTimeout")]
    pub read_idle_timeout: Option<IntOrString>,
    /// ResponseHeaderTimeout is the amount of time to wait for a server's response headers after fully writing the request (including its body, if any).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "responseHeaderTimeout")]
    pub response_header_timeout: Option<IntOrString>,
}

/// RootCA defines a reference to a Secret or a ConfigMap that holds a CA certificate.
/// If both a Secret and a ConfigMap reference are defined, the Secret reference takes precedence.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServersTransportRootCAs {
    /// ConfigMap defines the name of a ConfigMap that holds a CA certificate.
    /// The referenced ConfigMap must contain a certificate under either a tls.ca or a ca.crt key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMap")]
    pub config_map: Option<String>,
    /// Secret defines the name of a Secret that holds a CA certificate.
    /// The referenced Secret must contain a certificate under either a tls.ca or a ca.crt key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

/// Spiffe defines the SPIFFE configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServersTransportSpiffe {
    /// IDs defines the allowed SPIFFE IDs (takes precedence over the SPIFFE TrustDomain).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// TrustDomain defines the allowed SPIFFE trust domain.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "trustDomain")]
    pub trust_domain: Option<String>,
}

