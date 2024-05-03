// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/apecloud/kubeblocks/apps.kubeblocks.io/v1alpha1/componentversions.yaml --derive=Default --derive=PartialEq
// kopium version: 0.19.0

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// ComponentVersionSpec defines the desired state of ComponentVersion
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "apps.kubeblocks.io", version = "v1alpha1", kind = "ComponentVersion", plural = "componentversions")]
#[kube(status = "ComponentVersionStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ComponentVersionSpec {
    /// CompatibilityRules defines compatibility rules between sets of component definitions and releases.
    #[serde(rename = "compatibilityRules")]
    pub compatibility_rules: Vec<ComponentVersionCompatibilityRules>,
    /// Releases represents different releases of component instances within this ComponentVersion.
    pub releases: Vec<ComponentVersionReleases>,
}

/// ComponentVersionCompatibilityRule defines the compatibility between a set of component definitions and a set of releases.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ComponentVersionCompatibilityRules {
    /// CompDefs specifies names for the component definitions associated with this ComponentVersion. Each name in the list can represent an exact name, or a name prefix. 
    ///  For example: 
    ///  - "mysql-8.0.30-v1alpha1": Matches the exact name "mysql-8.0.30-v1alpha1" - "mysql-8.0.30": Matches all names starting with "mysql-8.0.30"
    #[serde(rename = "compDefs")]
    pub comp_defs: Vec<String>,
    /// Releases is a list of identifiers for the releases.
    pub releases: Vec<String>,
}

/// ComponentVersionRelease represents a release of component instances within a ComponentVersion.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ComponentVersionReleases {
    /// Changes provides information about the changes made in this release.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<String>,
    /// Images define the new images for different containers within the release.
    pub images: BTreeMap<String, String>,
    /// Name is a unique identifier for this release. Cannot be updated.
    pub name: String,
    /// ServiceVersion defines the version of the well-known service that the component provides. The version should follow the syntax and semantics of the "Semantic Versioning" specification (http://semver.org/). If the release is used, it will serve as the service version for component instances, overriding the one defined in the component definition. Cannot be updated.
    #[serde(rename = "serviceVersion")]
    pub service_version: String,
}

/// ComponentVersionStatus defines the observed state of ComponentVersion
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ComponentVersionStatus {
    /// Extra message for current phase.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// ObservedGeneration is the most recent generation observed for this ComponentVersion.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// Phase valid values are ``, `Available`, 'Unavailable`. Available is ComponentVersion become available, and can be used for co-related objects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<ComponentVersionStatusPhase>,
    /// ServiceVersions represent the supported service versions of this ComponentVersion.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceVersions")]
    pub service_versions: Option<String>,
}

/// ComponentVersionStatus defines the observed state of ComponentVersion
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ComponentVersionStatusPhase {
    Available,
    Unavailable,
}

