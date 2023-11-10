// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/traefik/traefik/traefik.io/v1alpha1/tlsoptions.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// TLSOptionSpec defines the desired state of a TLSOption.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "traefik.io", version = "v1alpha1", kind = "TLSOption", plural = "tlsoptions")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct TLSOptionSpec {
    /// ALPNProtocols defines the list of supported application level protocols for the TLS handshake, in order of preference. More info: https://doc.traefik.io/traefik/v3.0/https/tls/#alpn-protocols
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "alpnProtocols")]
    pub alpn_protocols: Option<Vec<String>>,
    /// CipherSuites defines the list of supported cipher suites for TLS versions up to TLS 1.2. More info: https://doc.traefik.io/traefik/v3.0/https/tls/#cipher-suites
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cipherSuites")]
    pub cipher_suites: Option<Vec<String>>,
    /// ClientAuth defines the server's policy for TLS Client Authentication.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientAuth")]
    pub client_auth: Option<TLSOptionClientAuth>,
    /// CurvePreferences defines the preferred elliptic curves in a specific order. More info: https://doc.traefik.io/traefik/v3.0/https/tls/#curve-preferences
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "curvePreferences")]
    pub curve_preferences: Option<Vec<String>>,
    /// MaxVersion defines the maximum TLS version that Traefik will accept. Possible values: VersionTLS10, VersionTLS11, VersionTLS12, VersionTLS13. Default: None.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxVersion")]
    pub max_version: Option<String>,
    /// MinVersion defines the minimum TLS version that Traefik will accept. Possible values: VersionTLS10, VersionTLS11, VersionTLS12, VersionTLS13. Default: VersionTLS10.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minVersion")]
    pub min_version: Option<String>,
    /// SniStrict defines whether Traefik allows connections from clients connections that do not specify a server_name extension.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sniStrict")]
    pub sni_strict: Option<bool>,
}

/// ClientAuth defines the server's policy for TLS Client Authentication.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TLSOptionClientAuth {
    /// ClientAuthType defines the client authentication type to apply.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientAuthType")]
    pub client_auth_type: Option<TLSOptionClientAuthClientAuthType>,
    /// SecretNames defines the names of the referenced Kubernetes Secret storing certificate details.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretNames")]
    pub secret_names: Option<Vec<String>>,
}

/// ClientAuth defines the server's policy for TLS Client Authentication.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TLSOptionClientAuthClientAuthType {
    NoClientCert,
    RequestClientCert,
    RequireAnyClientCert,
    VerifyClientCertIfGiven,
    RequireAndVerifyClientCert,
}

