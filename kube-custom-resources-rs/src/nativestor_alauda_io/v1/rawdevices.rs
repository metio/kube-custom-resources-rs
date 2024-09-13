// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/alauda/nativestor/nativestor.alauda.io/v1/rawdevices.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// RawDeviceSpec defines the desired state of RawDevice
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "nativestor.alauda.io", version = "v1", kind = "RawDevice", plural = "rawdevices")]
#[kube(status = "RawDeviceStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct RawDeviceSpec {
    pub available: bool,
    pub major: i32,
    pub minor: i32,
    /// INSERT ADDITIONAL SPEC FIELDS - desired state of cluster Important: Run "make" to regenerate code after modifying this file
    #[serde(rename = "nodeName")]
    pub node_name: String,
    #[serde(rename = "realPath")]
    pub real_path: String,
    pub size: i64,
    #[serde(rename = "type")]
    pub r#type: String,
    pub uuid: String,
}

/// RawDeviceStatus defines the observed state of RawDevice
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RawDeviceStatus {
    /// INSERT ADDITIONAL STATUS FIELD - define observed state of cluster Important: Run "make" to regenerate code after modifying this file
    pub name: String,
}

