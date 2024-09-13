// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/traefik/traefik/traefik.io/v1alpha1/middlewaretcps.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// MiddlewareTCPSpec defines the desired state of a MiddlewareTCP.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "traefik.io", version = "v1alpha1", kind = "MiddlewareTCP", plural = "middlewaretcps")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct MiddlewareTCPSpec {
    /// InFlightConn defines the InFlightConn middleware configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inFlightConn")]
    pub in_flight_conn: Option<MiddlewareTCPInFlightConn>,
    /// IPAllowList defines the IPAllowList middleware configuration.
    /// This middleware accepts/refuses connections based on the client IP.
    /// More info: https://doc.traefik.io/traefik/v3.1/middlewares/tcp/ipallowlist/
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipAllowList")]
    pub ip_allow_list: Option<MiddlewareTCPIpAllowList>,
    /// IPWhiteList defines the IPWhiteList middleware configuration.
    /// This middleware accepts/refuses connections based on the client IP.
    /// Deprecated: please use IPAllowList instead.
    /// More info: https://doc.traefik.io/traefik/v3.1/middlewares/tcp/ipwhitelist/
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipWhiteList")]
    pub ip_white_list: Option<MiddlewareTCPIpWhiteList>,
}

/// InFlightConn defines the InFlightConn middleware configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MiddlewareTCPInFlightConn {
    /// Amount defines the maximum amount of allowed simultaneous connections.
    /// The middleware closes the connection if there are already amount connections opened.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}

/// IPAllowList defines the IPAllowList middleware configuration.
/// This middleware accepts/refuses connections based on the client IP.
/// More info: https://doc.traefik.io/traefik/v3.1/middlewares/tcp/ipallowlist/
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MiddlewareTCPIpAllowList {
    /// SourceRange defines the allowed IPs (or ranges of allowed IPs by using CIDR notation).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceRange")]
    pub source_range: Option<Vec<String>>,
}

/// IPWhiteList defines the IPWhiteList middleware configuration.
/// This middleware accepts/refuses connections based on the client IP.
/// Deprecated: please use IPAllowList instead.
/// More info: https://doc.traefik.io/traefik/v3.1/middlewares/tcp/ipwhitelist/
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MiddlewareTCPIpWhiteList {
    /// SourceRange defines the allowed IPs (or ranges of allowed IPs by using CIDR notation).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceRange")]
    pub source_range: Option<Vec<String>>,
}

