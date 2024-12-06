// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/FyraLabs/chisel-operator/chisel-operator.io/v2/exitnodes.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// ExitNode is a custom resource that represents a Chisel exit node. It will be used as the reverse proxy for all services in the cluster.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "chisel-operator.io", version = "v2", kind = "ExitNode", plural = "exitnodes")]
#[kube(namespaced)]
#[kube(status = "ExitNodeStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ExitNodeSpec {
    /// Optional authentication secret name to connect to the control plane
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<String>,
    /// Optional value for the chisel client image used to connect to the chisel server If not provided, jpillora/chisel:latest is used
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chisel_image: Option<String>,
    /// Optional boolean value for whether to make the exit node the default route for the cluster If true, the exit node will be the default route for the cluster default value is false
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_route: Option<bool>,
    /// Optional real external hostname/IP of exit node If not provided, the host field will be used
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_host: Option<String>,
    /// Optional but highly recommended fingerprint to perform host-key validation against the server's public key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// Hostname or IP address of the chisel server
    pub host: String,
    /// Control plane port of the chisel server
    pub port: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExitNodeStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub ip: String,
    pub name: String,
    pub provider: String,
    pub service_binding: Vec<ExitNodeStatusServiceBinding>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExitNodeStatusServiceBinding {
    pub name: String,
    pub namespace: String,
}

