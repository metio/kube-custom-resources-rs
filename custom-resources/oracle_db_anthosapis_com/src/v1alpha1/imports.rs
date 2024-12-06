// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/GoogleCloudPlatform/elcarro-oracle-operator/oracle.db.anthosapis.com/v1alpha1/imports.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// ImportSpec defines the desired state of Import.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "oracle.db.anthosapis.com", version = "v1alpha1", kind = "Import", plural = "imports")]
#[kube(namespaced)]
#[kube(status = "ImportStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ImportSpec {
    /// DatabaseName is the database resource name within Instance to import into.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "databaseName")]
    pub database_name: Option<String>,
    /// GcsLogPath is an optional path in GCS to copy import log to. A user is to ensure proper write access to the bucket from within the Oracle Operator.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gcsLogPath")]
    pub gcs_log_path: Option<String>,
    /// GcsPath is a full path to the input file in GCS containing import data. A user is to ensure proper write access to the bucket from within the Oracle Operator.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gcsPath")]
    pub gcs_path: Option<String>,
    /// Instance is the resource name within same namespace to import into.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    /// Options is a map of options and their values for usage with the specified Import Type. Right now this is only supported for passing additional impdp specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<BTreeMap<String, String>>,
    /// Type of the Import. If not specified, the default of DataPump is assumed, which is the only supported option currently.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<ImportType>,
}

/// ImportSpec defines the desired state of Import.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ImportType {
    DataPump,
}

/// ImportStatus defines the observed state of Import.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ImportStatus {
    /// Conditions represents the latest available observations of the import's current state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

