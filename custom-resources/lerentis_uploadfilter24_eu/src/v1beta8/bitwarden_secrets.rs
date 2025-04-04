// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/Lerentis/bitwarden-crd-operator/lerentis.uploadfilter24.eu/v1beta8/bitwarden-secrets.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "lerentis.uploadfilter24.eu", version = "v1beta8", kind = "BitwardenSecret", plural = "bitwarden-secrets")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct BitwardenSecretSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<BitwardenSecretContent>>,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, serde_json::Value>>,
    pub name: String,
    pub namespace: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretType")]
    pub secret_type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BitwardenSecretContent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub element: Option<BitwardenSecretContentElement>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BitwardenSecretContentElement {
    #[serde(rename = "secretName")]
    pub secret_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretScope")]
    pub secret_scope: Option<String>,
}

