// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/GoogleCloudPlatform/elcarro-oracle-operator/oracle.db.anthosapis.com/v1alpha1/exports.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// ExportSpec defines the desired state of Export
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "oracle.db.anthosapis.com", version = "v1alpha1", kind = "Export", plural = "exports")]
#[kube(namespaced)]
#[kube(status = "ExportStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
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
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExportStatus {
    /// Conditions represents the latest available observations of the export's current state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

