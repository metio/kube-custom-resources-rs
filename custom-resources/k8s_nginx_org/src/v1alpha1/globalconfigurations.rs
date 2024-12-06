// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/nginxinc/kubernetes-ingress/k8s.nginx.org/v1alpha1/globalconfigurations.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// GlobalConfigurationSpec is the spec of the GlobalConfiguration resource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "k8s.nginx.org", version = "v1alpha1", kind = "GlobalConfiguration", plural = "globalconfigurations")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct GlobalConfigurationSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<GlobalConfigurationListeners>>,
}

/// Listener defines a listener.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GlobalConfigurationListeners {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssl: Option<bool>,
}

