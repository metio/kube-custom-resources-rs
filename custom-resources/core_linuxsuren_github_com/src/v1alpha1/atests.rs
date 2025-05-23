// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/LinuxSuRen/atest-operator/core.linuxsuren.github.com/v1alpha1/atests.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// ATestSpec defines the desired state of ATest
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "core.linuxsuren.github.com", version = "v1alpha1", kind = "ATest", plural = "atests")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ATestSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Persistent defines the persistent volume claim
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persistent: Option<ATestPersistent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// Service Type string describes ingress methods for a service
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceType")]
    pub service_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Persistent defines the persistent volume claim
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ATestPersistent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageClass")]
    pub storage_class: Option<String>,
}

/// ATestStatus defines the observed state of ATest
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ATestStatus {
}

