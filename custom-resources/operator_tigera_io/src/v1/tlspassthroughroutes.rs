// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/tigera/operator/operator.tigera.io/v1/tlspassthroughroutes.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// Dest is the destination URL
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "operator.tigera.io", version = "v1", kind = "TLSPassThroughRoute", plural = "tlspassthroughroutes")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct TLSPassThroughRouteSpec {
    /// Destination is the destination url to proxy the request to.
    pub destination: String,
    /// SNIMatch is used to match requests based on the server name for the intended destination server. Matching requests
    /// will be proxied to the Destination.
    #[serde(rename = "sniMatch")]
    pub sni_match: TLSPassThroughRouteSniMatch,
    pub target: TLSPassThroughRouteTarget,
}

/// SNIMatch is used to match requests based on the server name for the intended destination server. Matching requests
/// will be proxied to the Destination.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TLSPassThroughRouteSniMatch {
    /// ServerName is used to match the server name for the request.
    #[serde(rename = "serverName")]
    pub server_name: String,
}

/// Dest is the destination URL
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TLSPassThroughRouteTarget {
    UpstreamTunnel,
}

