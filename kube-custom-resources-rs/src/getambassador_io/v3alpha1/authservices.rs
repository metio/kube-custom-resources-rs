// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/emissary-ingress/emissary/getambassador.io/v3alpha1/authservices.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// AuthServiceSpec defines the desired state of AuthService
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "getambassador.io", version = "v3alpha1", kind = "AuthService", plural = "authservices")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct AuthServiceSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_auth_headers: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_linkerd_headers: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_request_body: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_authorization_headers: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_request_headers: Option<Vec<String>>,
    /// AmbassadorID declares which Ambassador instances should pay
    /// attention to this resource. If no value is provided, the default is:
    /// 
    /// 	ambassador_id:
    /// 	- "default"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ambassador_id: Option<Vec<String>>,
    pub auth_service: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub circuit_breakers: Option<Vec<AuthServiceCircuitBreakers>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_mode_allow: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_body: Option<AuthServiceIncludeBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proto: Option<AuthServiceProto>,
    /// ProtocolVersion is the envoy api transport protocol version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol_version: Option<AuthServiceProtocolVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_on_error: Option<AuthServiceStatusOnError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<String>,
    /// V2ExplicitTLS controls some vanity/stylistic elements when converting
    /// from v3alpha1 to v2.  The values in an V2ExplicitTLS should not in any
    /// way affect the runtime operation of Emissary; except that it may affect
    /// internal names in the Envoy config, which may in turn affect stats
    /// names.  But it should not affect any end-user observable behavior.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "v2ExplicitTLS")]
    pub v2_explicit_tls: Option<AuthServiceV2ExplicitTls>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AuthServiceCircuitBreakers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_pending_requests: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_requests: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<AuthServiceCircuitBreakersPriority>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AuthServiceCircuitBreakersPriority {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "high")]
    High,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AuthServiceIncludeBody {
    pub allow_partial: bool,
    /// These aren't pointer types because they are required.
    pub max_bytes: i64,
}

/// AuthServiceSpec defines the desired state of AuthService
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AuthServiceProto {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "grpc")]
    Grpc,
}

/// AuthServiceSpec defines the desired state of AuthService
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AuthServiceProtocolVersion {
    #[serde(rename = "v2")]
    V2,
    #[serde(rename = "v3")]
    V3,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AuthServiceStatusOnError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
}

/// V2ExplicitTLS controls some vanity/stylistic elements when converting
/// from v3alpha1 to v2.  The values in an V2ExplicitTLS should not in any
/// way affect the runtime operation of Emissary; except that it may affect
/// internal names in the Envoy config, which may in turn affect stats
/// names.  But it should not affect any end-user observable behavior.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AuthServiceV2ExplicitTls {
    /// ServiceScheme specifies how to spell and capitalize the scheme-part of the
    /// service URL.
    /// 
    /// Acceptable values are "http://" (case-insensitive), "https://"
    /// (case-insensitive), or "".  The value is used if it agrees with
    /// whether or not this resource enables TLS origination, or if
    /// something else in the resource overrides the scheme.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceScheme")]
    pub service_scheme: Option<String>,
    /// TLS controls whether and how to represent the "tls" field when
    /// its value could be implied by the "service" field.  In v2, there
    /// were a lot of different ways to spell an "empty" value, and this
    /// field specifies which way to spell it (and will therefore only
    /// be used if the value will indeed be empty).
    /// 
    ///  | Value        | Representation                        | Meaning of representation          |
    ///  |--------------+---------------------------------------+------------------------------------|
    ///  | ""           | omit the field                        | defer to service (no TLSContext)   |
    ///  | "null"       | store an explicit "null" in the field | defer to service (no TLSContext)   |
    ///  | "string"     | store an empty string in the field    | defer to service (no TLSContext)   |
    ///  | "bool:false" | store a Boolean "false" in the field  | defer to service (no TLSContext)   |
    ///  | "bool:true"  | store a Boolean "true" in the field   | originate TLS (no TLSContext)      |
    /// 
    /// If the meaning of the representation contradicts anything else
    /// (if a TLSContext is to be used, or in the case of "bool:true" if
    /// TLS is not to be originated), then this field is ignored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<AuthServiceV2ExplicitTlsTls>,
}

/// V2ExplicitTLS controls some vanity/stylistic elements when converting
/// from v3alpha1 to v2.  The values in an V2ExplicitTLS should not in any
/// way affect the runtime operation of Emissary; except that it may affect
/// internal names in the Envoy config, which may in turn affect stats
/// names.  But it should not affect any end-user observable behavior.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AuthServiceV2ExplicitTlsTls {
    #[serde(rename = "")]
    KopiumEmpty,
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "bool:true")]
    BoolTrue,
    #[serde(rename = "bool:false")]
    BoolFalse,
    #[serde(rename = "string")]
    String,
}

