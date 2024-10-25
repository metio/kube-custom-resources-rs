// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/metal3-io/baremetal-operator/metal3.io/v1alpha1/hostfirmwarecomponents.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// HostFirmwareComponentsSpec defines the desired state of HostFirmwareComponents.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "metal3.io", version = "v1alpha1", kind = "HostFirmwareComponents", plural = "hostfirmwarecomponents")]
#[kube(namespaced)]
#[kube(status = "HostFirmwareComponentsStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct HostFirmwareComponentsSpec {
    pub updates: Vec<HostFirmwareComponentsUpdates>,
}

/// FirmwareUpdate defines a firmware update specification.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HostFirmwareComponentsUpdates {
    pub component: String,
    pub url: String,
}

/// HostFirmwareComponentsStatus defines the observed state of HostFirmwareComponents.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HostFirmwareComponentsStatus {
    /// Components is the list of all available firmware components and their information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<HostFirmwareComponentsStatusComponents>>,
    /// Track whether updates stored in the spec are valid based on the schema
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Time that the status was last updated
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUpdated")]
    pub last_updated: Option<String>,
    /// Updates is the list of all firmware components that should be updated
    /// they are specified via name and url fields.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updates: Option<Vec<HostFirmwareComponentsStatusUpdates>>,
}

/// FirmwareComponentStatus defines the status of a firmware component.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HostFirmwareComponentsStatusComponents {
    pub component: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentVersion")]
    pub current_version: Option<String>,
    #[serde(rename = "initialVersion")]
    pub initial_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastVersionFlashed")]
    pub last_version_flashed: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updatedAt")]
    pub updated_at: Option<String>,
}

/// FirmwareUpdate defines a firmware update specification.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HostFirmwareComponentsStatusUpdates {
    pub component: String,
    pub url: String,
}

