// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/nginxinc/kubernetes-ingress/appprotect.f5.com/v1beta1/aplogconfs.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// APLogConfSpec defines the desired state of APLogConf
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "appprotect.f5.com", version = "v1beta1", kind = "APLogConf", plural = "aplogconfs")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct APLogConfSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<APLogConfContent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<APLogConfFilter>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APLogConfContent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub escaping_characters: Option<Vec<APLogConfContentEscapingCharacters>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<APLogConfContentFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format_string: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_delimiter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_suffix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_message_size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_request_size: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APLogConfContentEscapingCharacters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum APLogConfContentFormat {
    #[serde(rename = "splunk")]
    Splunk,
    #[serde(rename = "arcsight")]
    Arcsight,
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "user-defined")]
    UserDefined,
    #[serde(rename = "grpc")]
    Grpc,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APLogConfFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_type: Option<APLogConfFilterRequestType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum APLogConfFilterRequestType {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "illegal")]
    Illegal,
    #[serde(rename = "blocked")]
    Blocked,
}

