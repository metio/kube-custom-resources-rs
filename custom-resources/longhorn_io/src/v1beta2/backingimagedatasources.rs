// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/longhorn/longhorn/longhorn.io/v1beta2/backingimagedatasources.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// BackingImageDataSourceSpec defines the desired state of the Longhorn backing image data source
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "longhorn.io", version = "v1beta2", kind = "BackingImageDataSource", plural = "backingimagedatasources")]
#[kube(namespaced)]
#[kube(status = "BackingImageDataSourceStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct BackingImageDataSourceSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "diskPath")]
    pub disk_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "diskUUID")]
    pub disk_uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fileTransferred")]
    pub file_transferred: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeID")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceType")]
    pub source_type: Option<BackingImageDataSourceSourceType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

/// BackingImageDataSourceSpec defines the desired state of the Longhorn backing image data source
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BackingImageDataSourceSourceType {
    #[serde(rename = "download")]
    Download,
    #[serde(rename = "upload")]
    Upload,
    #[serde(rename = "export-from-volume")]
    ExportFromVolume,
    #[serde(rename = "restore")]
    Restore,
    #[serde(rename = "clone")]
    Clone,
}

/// BackingImageDataSourceStatus defines the observed state of the Longhorn backing image data source
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackingImageDataSourceStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentState")]
    pub current_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runningParameters")]
    pub running_parameters: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageIP")]
    pub storage_ip: Option<String>,
}

