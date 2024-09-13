// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/b3scale/b3scale-operator/b3scale.io/v1/bbbfrontends.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// Desired state of the BBBFrontend resource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "b3scale.io", version = "v1", kind = "BBBFrontend", plural = "bbbfrontends")]
#[kube(namespaced)]
#[kube(status = "BBBFrontendStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct BBBFrontendSpec {
    /// Predefined credentials for the B3scale instance
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<BBBFrontendCredentials>,
    /// Protect BBB frontend resource from deletion in b3scale API
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deletionProtection")]
    pub deletion_protection: Option<bool>,
    /// Unique BBB frontend ID generated by the b3scale API during creation
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "frontendID")]
    pub frontend_id: Option<String>,
    /// Settings defines the B3Scale instance settings
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<BBBFrontendSettings>,
}

/// Predefined credentials for the B3scale instance
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BBBFrontendCredentials {
    /// Predefined key for B3scale instance
    pub frontend: String,
    /// SecretRef is a reference to a key in a Secret resource containing the key to connect to the BBB instance.
    #[serde(rename = "secretRef")]
    pub secret_ref: BBBFrontendCredentialsSecretRef,
}

/// SecretRef is a reference to a key in a Secret resource containing the key to connect to the BBB instance.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BBBFrontendCredentialsSecretRef {
    /// The key of the entry in the Secret resource's `data` field to be used.
    pub key: String,
    /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: String,
}

/// Settings defines the B3Scale instance settings
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BBBFrontendSettings {
    /// See https://github.com/b3scale/b3scale#configure-create-parameter-defaults-and-overrides
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createDefaultParams")]
    pub create_default_params: Option<BTreeMap<String, String>>,
    /// See https://github.com/b3scale/b3scale#configure-create-parameter-defaults-and-overrides
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createOverrideParams")]
    pub create_override_params: Option<BTreeMap<String, String>>,
    /// See https://github.com/b3scale/b3scale#middleware-configuration
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultPresentation")]
    pub default_presentation: Option<BBBFrontendSettingsDefaultPresentation>,
    /// See https://github.com/b3scale/b3scale#middleware-configuration
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requiredTags")]
    pub required_tags: Option<Vec<String>>,
}

/// See https://github.com/b3scale/b3scale#middleware-configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BBBFrontendSettingsDefaultPresentation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Status of the BBBFrontend. This is set and managed automatically.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BBBFrontendStatus {
    /// List of status conditions to indicate the status of the BBB frontend. Known condition types are `Ready`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

