// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/openshift/ptp-operator/ptp.openshift.io/v1/nodeptpdevices.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// NodePtpDeviceSpec defines the desired state of NodePtpDevice
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "ptp.openshift.io", version = "v1", kind = "NodePtpDevice", plural = "nodeptpdevices")]
#[kube(namespaced)]
#[kube(status = "NodePtpDeviceStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct NodePtpDeviceSpec {
}

/// NodePtpDeviceStatus defines the observed state of NodePtpDevice
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodePtpDeviceStatus {
    /// INSERT ADDITIONAL STATUS FIELD - define observed state of cluster Important: Run "make" to regenerate code after modifying this file
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<NodePtpDeviceStatusDevices>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hwconfig: Option<Vec<NodePtpDeviceStatusHwconfig>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodePtpDeviceStatusDevices {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodePtpDeviceStatusHwconfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deviceID")]
    pub device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vendorID")]
    pub vendor_id: Option<String>,
}

