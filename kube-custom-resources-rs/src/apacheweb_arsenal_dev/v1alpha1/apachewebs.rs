// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/arsenalzp/apch-operator/apacheweb.arsenal.dev/v1alpha1/apachewebs.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "apacheweb.arsenal.dev", version = "v1alpha1", kind = "Apacheweb", plural = "apachewebs")]
#[kube(namespaced)]
#[kube(status = "ApachewebStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct ApachewebSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "loadBalancer")]
    pub load_balancer: Option<ApachewebLoadBalancer>,
    #[serde(rename = "serverName")]
    pub server_name: String,
    pub size: i32,
    #[serde(rename = "type")]
    pub r#type: ApachewebType,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "webServer")]
    pub web_server: Option<ApachewebWebServer>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApachewebLoadBalancer {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backEndService")]
    pub back_end_service: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endPointsList")]
    pub end_points_list: Option<Vec<ApachewebLoadBalancerEndPointsList>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proto: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "proxyPaths")]
    pub proxy_paths: Option<Vec<ApachewebLoadBalancerProxyPaths>>,
    #[serde(rename = "serverPort")]
    pub server_port: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApachewebLoadBalancerEndPointsList {
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
    pub port: i32,
    pub proto: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApachewebLoadBalancerProxyPaths {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endPointsList")]
    pub end_points_list: Option<Vec<ApachewebLoadBalancerProxyPathsEndPointsList>>,
    pub path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApachewebLoadBalancerProxyPathsEndPointsList {
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
    pub port: i32,
    pub proto: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ApachewebType {
    #[serde(rename = "web")]
    Web,
    #[serde(rename = "lb")]
    Lb,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApachewebWebServer {
    #[serde(rename = "documentRoot")]
    pub document_root: String,
    #[serde(rename = "serverAdmin")]
    pub server_admin: String,
    #[serde(rename = "serverPort")]
    pub server_port: i32,
}

/// ApachewebStatus defines the observed state of Apacheweb
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApachewebStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endPoints")]
    pub end_points: Option<Vec<ApachewebStatusEndPoints>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "proxyPaths")]
    pub proxy_paths: Option<Vec<ApachewebStatusProxyPaths>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "webServer")]
    pub web_server: Option<ApachewebStatusWebServer>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApachewebStatusEndPoints {
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
    pub port: i32,
    pub proto: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApachewebStatusProxyPaths {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endPointsList")]
    pub end_points_list: Option<Vec<ApachewebStatusProxyPathsEndPointsList>>,
    pub path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApachewebStatusProxyPathsEndPointsList {
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
    pub port: i32,
    pub proto: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApachewebStatusWebServer {
    #[serde(rename = "documentRoot")]
    pub document_root: String,
    #[serde(rename = "serverAdmin")]
    pub server_admin: String,
    #[serde(rename = "serverPort")]
    pub server_port: i32,
}

