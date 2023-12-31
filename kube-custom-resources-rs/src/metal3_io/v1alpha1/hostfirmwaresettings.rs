// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/metal3-io/baremetal-operator/metal3.io/v1alpha1/hostfirmwaresettings.yaml --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// HostFirmwareSettingsSpec defines the desired state of HostFirmwareSettings
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "metal3.io", version = "v1alpha1", kind = "HostFirmwareSettings", plural = "hostfirmwaresettings")]
#[kube(namespaced)]
#[kube(status = "HostFirmwareSettingsStatus")]
#[kube(schema = "disabled")]
pub struct HostFirmwareSettingsSpec {
    /// Settings are the desired firmware settings stored as name/value pairs.
    pub settings: BTreeMap<String, IntOrString>,
}

/// HostFirmwareSettingsStatus defines the observed state of HostFirmwareSettings
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HostFirmwareSettingsStatus {
    /// Track whether settings stored in the spec are valid based on the schema
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<HostFirmwareSettingsStatusConditions>>,
    /// Time that the status was last updated
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUpdated")]
    pub last_updated: Option<String>,
    /// FirmwareSchema is a reference to the Schema used to describe each FirmwareSetting. By default, this will be a Schema in the same Namespace as the settings but it can be overwritten in the Spec
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<HostFirmwareSettingsStatusSchema>,
    /// Settings are the firmware settings stored as name/value pairs
    pub settings: BTreeMap<String, String>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HostFirmwareSettingsStatusConditions {
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
    pub status: HostFirmwareSettingsStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum HostFirmwareSettingsStatusConditionsStatus {
    True,
    False,
    Unknown,
}

/// FirmwareSchema is a reference to the Schema used to describe each FirmwareSetting. By default, this will be a Schema in the same Namespace as the settings but it can be overwritten in the Spec
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HostFirmwareSettingsStatusSchema {
    /// `name` is the reference to the schema.
    pub name: String,
    /// `namespace` is the namespace of the where the schema is stored.
    pub namespace: String,
}

