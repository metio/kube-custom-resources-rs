// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename ./crd-catalog/devfile/registry-operator/registry.devfile.io/v1alpha1/devfileregistrieslists.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// DevfileRegistriesListSpec defines the desired state of DevfileRegistriesList
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "registry.devfile.io", version = "v1alpha1", kind = "DevfileRegistriesList", plural = "devfileregistrieslists")]
#[kube(namespaced)]
#[kube(status = "DevfileRegistriesListStatus")]
#[kube(schema = "disabled")]
pub struct DevfileRegistriesListSpec {
    /// DevfileRegistries is a list of devfile registry services
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "devfileRegistries")]
    pub devfile_registries: Option<Vec<DevfileRegistriesListDevfileRegistries>>,
}

/// DevfileRegistryService represents the properties used to identify a devfile registry service.
#[derive(Serialize, Deserialize, Clone, Debug)]
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
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DevfileRegistriesListStatus {
    /// Conditions shows the state of this CR's devfile registry list.  If registries are no longer reachable, they will be listed here
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<DevfileRegistriesListStatusConditions>>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DevfileRegistriesListStatusConditions {
    /// lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// message is a human readable message indicating details about the transition. This may be an empty string.
    pub message: String,
    /// observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
    pub reason: String,
    /// status of the condition, one of True, False, Unknown.
    pub status: DevfileRegistriesListStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DevfileRegistriesListStatusConditionsStatus {
    True,
    False,
    Unknown,
}

