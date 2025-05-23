// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/Lerentis/bitwarden-crd-operator/lerentis.uploadfilter24.eu/v1beta4/registry-credentials.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "lerentis.uploadfilter24.eu", version = "v1beta4", kind = "RegistryCredential", plural = "registry-credentials")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct RegistryCredentialSpec {
    pub id: String,
    pub name: String,
    pub namespace: String,
    #[serde(rename = "passwordRef")]
    pub password_ref: String,
    pub registry: String,
    #[serde(rename = "usernameRef")]
    pub username_ref: String,
}

