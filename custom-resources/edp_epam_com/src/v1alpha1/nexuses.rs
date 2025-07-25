// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/epam/edp-nexus-operator/edp.epam.com/v1alpha1/nexuses.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// NexusSpec defines the desired state of Nexus.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "edp.epam.com", version = "v1alpha1", kind = "Nexus", plural = "nexuses")]
#[kube(namespaced)]
#[kube(status = "NexusStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct NexusSpec {
    /// Secret is the name of the k8s object Secret related to nexus.
    /// Secret should contain a user field with a nexus username and a password field with a nexus password.
    pub secret: String,
    /// Url is the url of nexus instance.
    pub url: String,
}

/// NexusStatus defines the observed state of Nexus.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NexusStatus {
    /// Connected shows if operator is connected to nexus.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,
    /// Error represents error message if something went wrong.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

