// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/nginxinc/nginx-kubernetes-gateway/gateway.nginx.org/v1alpha1/nginxgateways.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// NginxGatewaySpec defines the desired state of the NginxGateway.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "gateway.nginx.org", version = "v1alpha1", kind = "NginxGateway", plural = "nginxgateways")]
#[kube(namespaced)]
#[kube(status = "NginxGatewayStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct NginxGatewaySpec {
    /// Logging defines logging related settings for the control plane.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<NginxGatewayLogging>,
}

/// Logging defines logging related settings for the control plane.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NginxGatewayLogging {
    /// Level defines the logging level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<NginxGatewayLoggingLevel>,
}

/// Logging defines logging related settings for the control plane.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NginxGatewayLoggingLevel {
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "error")]
    Error,
}

/// NginxGatewayStatus defines the state of the NginxGateway.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NginxGatewayStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

