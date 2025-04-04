// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/emissary-ingress/emissary/getambassador.io/v3alpha1/ratelimitservices.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// RateLimitServiceSpec defines the desired state of RateLimitService
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "getambassador.io", version = "v3alpha1", kind = "RateLimitService", plural = "ratelimitservices")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct RateLimitServiceSpec {
    /// Common to all Ambassador objects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ambassador_id: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// FailureModeDeny when set to true, envoy will deny traffic if it
    /// is unable to communicate with the rate limit service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_mode_deny: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grpc: Option<RateLimitServiceGrpc>,
    /// ProtocolVersion is the envoy api transport protocol version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol_version: Option<RateLimitServiceProtocolVersion>,
    pub service: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats_name: Option<String>,
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
    pub v2_explicit_tls: Option<RateLimitServiceV2ExplicitTls>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RateLimitServiceGrpc {
    /// UseResourceExhaustedCode, when set to true, will cause envoy
    /// to return a `RESOURCE_EXHAUSTED` gRPC code instead of the default
    /// `UNAVAILABLE` gRPC code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_resource_exhausted_code: Option<bool>,
}

/// RateLimitServiceSpec defines the desired state of RateLimitService
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RateLimitServiceProtocolVersion {
    #[serde(rename = "v2")]
    V2,
    #[serde(rename = "v3")]
    V3,
}

/// V2ExplicitTLS controls some vanity/stylistic elements when converting
/// from v3alpha1 to v2.  The values in an V2ExplicitTLS should not in any
/// way affect the runtime operation of Emissary; except that it may affect
/// internal names in the Envoy config, which may in turn affect stats
/// names.  But it should not affect any end-user observable behavior.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RateLimitServiceV2ExplicitTls {
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
    pub tls: Option<RateLimitServiceV2ExplicitTlsTls>,
}

/// V2ExplicitTLS controls some vanity/stylistic elements when converting
/// from v3alpha1 to v2.  The values in an V2ExplicitTLS should not in any
/// way affect the runtime operation of Emissary; except that it may affect
/// internal names in the Envoy config, which may in turn affect stats
/// names.  But it should not affect any end-user observable behavior.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RateLimitServiceV2ExplicitTlsTls {
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

