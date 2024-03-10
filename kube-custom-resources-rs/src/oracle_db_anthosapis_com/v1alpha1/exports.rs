// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/GoogleCloudPlatform/elcarro-oracle-operator/oracle.db.anthosapis.com/v1alpha1/exports.yaml --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// ExportSpec defines the desired state of Export
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "oracle.db.anthosapis.com", version = "v1alpha1", kind = "Export", plural = "exports")]
#[kube(namespaced)]
#[kube(status = "ExportStatus")]
#[kube(schema = "disabled")]
pub struct ExportSpec {
    /// DatabaseName is the database resource name within Instance to export from.
    #[serde(rename = "databaseName")]
    pub database_name: String,
    /// ExportObjectType is the type of objects to export. If omitted, the default of Schemas is assumed. Supported options at this point are: Schemas or Tables.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exportObjectType")]
    pub export_object_type: Option<ExportExportObjectType>,
    /// ExportObjects are objects, schemas or tables, exported by DataPump.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exportObjects")]
    pub export_objects: Option<Vec<String>>,
    /// FlashbackTime is an optional time. If this time is set, the SCN that most closely matches the time is found, and this SCN is used to enable the Flashback utility. The export operation is performed with data that is consistent up to this SCN.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "flashbackTime")]
    pub flashback_time: Option<String>,
    /// GcsLogPath is an optional full path in GCS. If set up ahead of time, export logs can be optionally transferred to set GCS bucket. A user is to ensure proper write access to the bucket from within the Oracle Operator.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gcsLogPath")]
    pub gcs_log_path: Option<String>,
    /// GcsPath is a full path in GCS bucket to transfer exported files to. A user is to ensure proper write access to the bucket from within the Oracle Operator.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gcsPath")]
    pub gcs_path: Option<String>,
    /// Instance is the resource name within namespace to export from.
    pub instance: String,
    /// Type of the Export. If omitted, the default of DataPump is assumed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<ExportType>,
}

/// ExportSpec defines the desired state of Export
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExportExportObjectType {
    Schemas,
    Tables,
}

/// ExportSpec defines the desired state of Export
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExportType {
    DataPump,
}

/// ExportStatus defines the observed state of Export.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ExportStatus {
    /// Conditions represents the latest available observations of the export's current state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ExportStatusConditions>>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ExportStatusConditions {
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
    pub status: ExportStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExportStatusConditionsStatus {
    True,
    False,
    Unknown,
}
