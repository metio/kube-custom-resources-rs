// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/nginxinc/kubernetes-ingress/k8s.nginx.org/v1alpha1/policies.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// PolicySpec is the spec of the Policy resource.
/// The spec includes multiple fields, where each field represents a different policy.
/// Only one policy (field) is allowed.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "k8s.nginx.org", version = "v1alpha1", kind = "Policy", plural = "policies")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct PolicySpec {
    /// AccessControl defines an access policy based on the source IP of a request.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessControl")]
    pub access_control: Option<PolicyAccessControl>,
    /// EgressMTLS defines an Egress MTLS policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "egressMTLS")]
    pub egress_mtls: Option<PolicyEgressMtls>,
    /// IngressMTLS defines an Ingress MTLS policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ingressMTLS")]
    pub ingress_mtls: Option<PolicyIngressMtls>,
    /// JWTAuth holds JWT authentication configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jwt: Option<PolicyJwt>,
    /// RateLimit defines a rate limit policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rateLimit")]
    pub rate_limit: Option<PolicyRateLimit>,
}

/// AccessControl defines an access policy based on the source IP of a request.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyAccessControl {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deny: Option<Vec<String>>,
}

/// EgressMTLS defines an Egress MTLS policy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyEgressMtls {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ciphers: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocols: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serverName")]
    pub server_name: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionReuse")]
    pub session_reuse: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sslName")]
    pub ssl_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsSecret")]
    pub tls_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "trustedCertSecret")]
    pub trusted_cert_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verifyDepth")]
    pub verify_depth: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verifyServer")]
    pub verify_server: Option<bool>,
}

/// IngressMTLS defines an Ingress MTLS policy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyIngressMtls {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientCertSecret")]
    pub client_cert_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verifyClient")]
    pub verify_client: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verifyDepth")]
    pub verify_depth: Option<i64>,
}

/// JWTAuth holds JWT authentication configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyJwt {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub realm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// RateLimit defines a rate limit policy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyRateLimit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub burst: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dryRun")]
    pub dry_run: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logLevel")]
    pub log_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "noDelay")]
    pub no_delay: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rejectCode")]
    pub reject_code: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "zoneSize")]
    pub zone_size: Option<String>,
}

