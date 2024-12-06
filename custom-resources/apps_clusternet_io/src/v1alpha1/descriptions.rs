// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/clusternet/clusternet/apps.clusternet.io/v1alpha1/descriptions.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// DescriptionSpec defines the spec of Description
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "apps.clusternet.io", version = "v1alpha1", kind = "Description", plural = "descriptions")]
#[kube(namespaced)]
#[kube(status = "DescriptionStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct DescriptionSpec {
    /// ChartRaw is the underlying serialization of all helm chart objects.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "chartRaw")]
    pub chart_raw: Option<Vec<String>>,
    /// Charts describe all the helm charts to be installed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub charts: Option<Vec<DescriptionCharts>>,
    /// Deployer indicates the deployer for this Description
    pub deployer: DescriptionDeployer,
    /// Raw is the underlying serialization of all objects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DescriptionCharts {
    /// Name of the HelmChart.
    pub name: String,
    /// Namespace of the HelmChart.
    pub namespace: String,
}

/// DescriptionSpec defines the spec of Description
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DescriptionDeployer {
    Helm,
    Generic,
}

/// DescriptionStatus defines the observed state of Description
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DescriptionStatus {
    /// ManifestStatuses contains a list of running statuses of manifests in DescriptionSpec.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "manifestStatuses")]
    pub manifest_statuses: Option<Vec<DescriptionStatusManifestStatuses>>,
    /// Phase denotes the phase of Description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<DescriptionStatusPhase>,
    /// Reason indicates the reason of DescriptionPhase
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// ManifestStatus contains details for the current status of this feed.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DescriptionStatusManifestStatuses {
    /// APIVersion defines the versioned schema of this representation of an object.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Kind is a string value representing the REST resource this object represents.
    /// In CamelCase.
    pub kind: String,
    /// Name of the target resource.
    pub name: String,
    /// Namespace of the target resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// ObservedStatus reflects observed status of current feed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedStatus")]
    pub observed_status: Option<BTreeMap<String, serde_json::Value>>,
}

/// DescriptionStatus defines the observed state of Description
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DescriptionStatusPhase {
    Pending,
    Success,
    Failure,
    Installing,
    Upgrading,
    Uninstalling,
    Superseded,
    Unknown,
}

