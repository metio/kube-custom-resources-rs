// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/longhorn/longhorn/longhorn.io/v1beta2/backingimagemanagers.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// BackingImageManagerSpec defines the desired state of the Longhorn backing image manager
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "longhorn.io", version = "v1beta2", kind = "BackingImageManager", plural = "backingimagemanagers")]
#[kube(namespaced)]
#[kube(status = "BackingImageManagerStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct BackingImageManagerSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backingImages")]
    pub backing_images: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "diskPath")]
    pub disk_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "diskUUID")]
    pub disk_uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeID")]
    pub node_id: Option<String>,
}

/// BackingImageManagerStatus defines the observed state of the Longhorn backing image manager
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackingImageManagerStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiMinVersion")]
    pub api_min_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backingImageFileMap")]
    pub backing_image_file_map: Option<BTreeMap<String, BackingImageManagerStatusBackingImageFileMap>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentState")]
    pub current_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageIP")]
    pub storage_ip: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackingImageManagerStatusBackingImageFileMap {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentChecksum")]
    pub current_checksum: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "realSize")]
    pub real_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "senderManagerAddress")]
    pub sender_manager_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sendingReference")]
    pub sending_reference: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "virtualSize")]
    pub virtual_size: Option<i64>,
}

