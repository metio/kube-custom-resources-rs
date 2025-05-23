// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/Lerentis/bitwarden-crd-operator/lerentis.uploadfilter24.eu/v1beta8/bitwarden-templates.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "lerentis.uploadfilter24.eu", version = "v1beta8", kind = "BitwardenTemplate", plural = "bitwarden-templates")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct BitwardenTemplateSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<BitwardenTemplateContent>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, serde_json::Value>>,
    pub name: String,
    pub namespace: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretType")]
    pub secret_type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BitwardenTemplateContent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub element: Option<BitwardenTemplateContentElement>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BitwardenTemplateContentElement {
    pub filename: String,
    pub template: String,
}

