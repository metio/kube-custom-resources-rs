// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/CleverCloud/clever-operator/api.clever-cloud.com/v1/configproviders.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "api.clever-cloud.com", version = "v1", kind = "ConfigProvider", plural = "configproviders")]
#[kube(namespaced)]
#[kube(status = "ConfigProviderStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ConfigProviderSpec {
    pub organisation: String,
    pub variables: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigProviderStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addon: Option<String>,
}

