// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/tinkerbell/tink/tinkerbell.org/v1alpha1/hardware.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
}
use self::prelude::*;

/// HardwareSpec defines the desired state of Hardware.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "tinkerbell.org", version = "v1alpha1", kind = "Hardware", plural = "hardware")]
#[kube(namespaced)]
#[kube(status = "HardwareStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct HardwareSpec {
    /// BMCRef contains a relation to a BMC state management type in the same
    /// namespace as the Hardware. This may be used for BMC management by
    /// orchestrators.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bmcRef")]
    pub bmc_ref: Option<HardwareBmcRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disks: Option<Vec<HardwareDisks>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interfaces: Option<Vec<HardwareInterfaces>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HardwareMetadata>,
    /// Resources represents known resources that are available on a machine.
    /// Resources may be used for scheduling by orchestrators.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<BTreeMap<String, IntOrString>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tinkVersion")]
    pub tink_version: Option<i64>,
    /// UserData is the user data to configure in the hardware's
    /// metadata
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userData")]
    pub user_data: Option<String>,
    /// VendorData is the vendor data to configure in the hardware's
    /// metadata
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vendorData")]
    pub vendor_data: Option<String>,
}

/// BMCRef contains a relation to a BMC state management type in the same
/// namespace as the Hardware. This may be used for BMC management by
/// orchestrators.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HardwareBmcRef {
    /// APIGroup is the group for the resource being referenced.
    /// If APIGroup is not specified, the specified Kind must be in the core API group.
    /// For any other third-party types, APIGroup is required.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiGroup")]
    pub api_group: Option<String>,
    /// Kind is the type of resource being referenced
    pub kind: String,
    /// Name is the name of resource being referenced
    pub name: String,
}

/// Disk represents a disk device for Tinkerbell Hardware.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HardwareDisks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
}

/// Interface represents a network interface configuration for Hardware.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HardwareInterfaces {
    /// DHCP configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dhcp: Option<HardwareInterfacesDhcp>,
    /// DisableDHCP disables DHCP for this interface.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disableDhcp")]
    pub disable_dhcp: Option<bool>,
    /// Netboot configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub netboot: Option<HardwareInterfacesNetboot>,
}

/// DHCP configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HardwareInterfacesDhcp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iface_name: Option<String>,
    /// IP configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<HardwareInterfacesDhcpIp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lease_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_servers: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_servers: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uefi: Option<bool>,
    /// validation pattern for VLANDID is a string number between 0-4096
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vlan_id: Option<String>,
}

/// IP configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HardwareInterfacesDhcpIp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub netmask: Option<String>,
}

/// Netboot configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HardwareInterfacesNetboot {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowPXE")]
    pub allow_pxe: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowWorkflow")]
    pub allow_workflow: Option<bool>,
    /// IPXE configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipxe: Option<HardwareInterfacesNetbootIpxe>,
    /// OSIE configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub osie: Option<HardwareInterfacesNetbootOsie>,
}

/// IPXE configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HardwareInterfacesNetbootIpxe {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contents: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// OSIE configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HardwareInterfacesNetbootOsie {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "baseURL")]
    pub base_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initrd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HardwareMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bonding_mode: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<HardwareMetadataCustom>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facility: Option<HardwareMetadataFacility>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<HardwareMetadataInstance>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<HardwareMetadataManufacturer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HardwareMetadataCustom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preinstalled_operating_system_version: Option<HardwareMetadataCustomPreinstalledOperatingSystemVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private_subnets: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HardwareMetadataCustomPreinstalledOperatingSystemVersion {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distro: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os_slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HardwareMetadataFacility {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facility_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_version_slug: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HardwareMetadataInstance {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_pxe: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub always_pxe: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crypted_root_password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ips: Option<Vec<HardwareMetadataInstanceIps>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipxe_script_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_ready: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<HardwareMetadataInstanceOperatingSystem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rescue: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssh_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage: Option<HardwareMetadataInstanceStorage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub userdata: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HardwareMetadataInstanceIps {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub management: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub netmask: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HardwareMetadataInstanceOperatingSystem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distro: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os_slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HardwareMetadataInstanceStorage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disks: Option<Vec<HardwareMetadataInstanceStorageDisks>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filesystems: Option<Vec<HardwareMetadataInstanceStorageFilesystems>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raid: Option<Vec<HardwareMetadataInstanceStorageRaid>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HardwareMetadataInstanceStorageDisks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partitions: Option<Vec<HardwareMetadataInstanceStorageDisksPartitions>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wipe_table: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HardwareMetadataInstanceStorageDisksPartitions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_guid: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HardwareMetadataInstanceStorageFilesystems {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mount: Option<HardwareMetadataInstanceStorageFilesystemsMount>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HardwareMetadataInstanceStorageFilesystemsMount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create: Option<HardwareMetadataInstanceStorageFilesystemsMountCreate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<HardwareMetadataInstanceStorageFilesystemsMountFiles>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub point: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HardwareMetadataInstanceStorageFilesystemsMountCreate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HardwareMetadataInstanceStorageFilesystemsMountFiles {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contents: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gid: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HardwareMetadataInstanceStorageRaid {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spare: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HardwareMetadataManufacturer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

/// HardwareStatus defines the observed state of Hardware.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HardwareStatus {
    /// HardwareState represents the hardware state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

