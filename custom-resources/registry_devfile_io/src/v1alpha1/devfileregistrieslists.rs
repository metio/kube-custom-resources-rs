// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/devfile/registry-operator/registry.devfile.io/v1alpha1/devfileregistrieslists.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// DevfileRegistriesListSpec defines the desired state of DevfileRegistriesList
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "registry.devfile.io", version = "v1alpha1", kind = "DevfileRegistriesList", plural = "devfileregistrieslists")]
#[kube(namespaced)]
#[kube(status = "DevfileRegistriesListStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DevfileRegistriesListSpec {
    /// DevfileRegistries is a list of devfile registry services
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "devfileRegistries")]
    pub devfile_registries: Option<Vec<DevfileRegistriesListDevfileRegistries>>,
}

/// DevfileRegistryService represents the properties used to identify a devfile registry service.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevfileRegistriesListDevfileRegistries {
    /// Name is the unique Name of the devfile registry.
    pub name: String,
    /// SkipTLSVerify defaults to false.  Set to true in a non-production environment to bypass certificate checking
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "skipTLSVerify")]
    pub skip_tls_verify: Option<bool>,
    /// URL is the unique URL of the devfile registry.
    pub url: String,
}

/// DevfileRegistriesListStatus defines the observed state of DevfileRegistriesList
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevfileRegistriesListStatus {
    /// Conditions shows the state of this CR's devfile registry list.  If registries are no longer reachable, they will be listed here
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

