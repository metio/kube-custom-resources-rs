// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/koordinator-sh/koordinator/scheduling.koordinator.sh/v1alpha1/devices.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "scheduling.koordinator.sh", version = "v1alpha1", kind = "Device", plural = "devices")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DeviceSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<DeviceDevices>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceDevices {
    /// Health indicates whether the device is normal
    pub health: bool,
    /// UUID represents the UUID of device
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Labels represents the device properties that can be used to organize and categorize (scope and select) objects
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// Minor represents the Minor number of Device, starting from 0
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minor: Option<i32>,
    /// ModuleID represents the physical id of Device
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "moduleID")]
    pub module_id: Option<i32>,
    /// Resources is a set of (resource name, quantity) pairs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<BTreeMap<String, IntOrString>>,
    /// Topology represents the topology information about the device
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topology: Option<DeviceDevicesTopology>,
    /// Type represents the type of device
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// VFGroups represents the virtual function devices
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vfGroups")]
    pub vf_groups: Option<Vec<DeviceDevicesVfGroups>>,
}

/// Topology represents the topology information about the device
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceDevicesTopology {
    /// BusID is the domain:bus:device.function formatted identifier of PCI/PCIE device
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "busID")]
    pub bus_id: Option<String>,
    /// NodeID is the ID of NUMA Node to which the device belongs, it should be unique across different CPU Sockets
    #[serde(rename = "nodeID")]
    pub node_id: i32,
    /// PCIEID is the ID of PCIE Switch to which the device is connected, it should be unique across difference NUMANodes
    #[serde(rename = "pcieID")]
    pub pcie_id: String,
    /// SocketID is the ID of CPU Socket to which the device belongs
    #[serde(rename = "socketID")]
    pub socket_id: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceDevicesVfGroups {
    /// Labels represents the Virtual Function properties that can be used to organize and categorize (scope and select) objects
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// VFs are the virtual function devices which belong to the group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vfs: Option<Vec<DeviceDevicesVfGroupsVfs>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceDevicesVfGroupsVfs {
    /// BusID is the domain:bus:device.function formatted identifier of PCI/PCIE virtual function device
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "busID")]
    pub bus_id: Option<String>,
    /// Minor represents the Minor number of VirtualFunction, starting from 0, used to identify virtual function.
    pub minor: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocations: Option<Vec<DeviceStatusAllocations>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceStatusAllocations {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<DeviceStatusAllocationsEntries>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceStatusAllocationsEntries {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minors: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

