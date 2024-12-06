// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/rook/rook/ceph.rook.io/v1/cephobjectzonegroups.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// ObjectZoneGroupSpec represent the spec of an ObjectZoneGroup
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "ceph.rook.io", version = "v1", kind = "CephObjectZoneGroup", plural = "cephobjectzonegroups")]
#[kube(namespaced)]
#[kube(status = "CephObjectZoneGroupStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct CephObjectZoneGroupSpec {
    /// The display name for the ceph users
    pub realm: String,
}

/// Status represents the status of an object
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephObjectZoneGroupStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// ObservedGeneration is the latest generation observed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}

