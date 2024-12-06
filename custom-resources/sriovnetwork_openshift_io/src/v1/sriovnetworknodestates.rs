// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/openshift/sriov-network-operator/sriovnetwork.openshift.io/v1/sriovnetworknodestates.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// SriovNetworkNodeStateSpec defines the desired state of SriovNetworkNodeState
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "sriovnetwork.openshift.io", version = "v1", kind = "SriovNetworkNodeState", plural = "sriovnetworknodestates")]
#[kube(namespaced)]
#[kube(status = "SriovNetworkNodeStateStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct SriovNetworkNodeStateSpec {
    /// Bridges contains list of bridges
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bridges: Option<SriovNetworkNodeStateBridges>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interfaces: Option<Vec<SriovNetworkNodeStateInterfaces>>,
}

/// Bridges contains list of bridges
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SriovNetworkNodeStateBridges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ovs: Option<Vec<SriovNetworkNodeStateBridgesOvs>>,
}

/// OVSConfigExt contains configuration for the concrete OVS bridge
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SriovNetworkNodeStateBridgesOvs {
    /// bridge-level configuration for the bridge
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bridge: Option<SriovNetworkNodeStateBridgesOvsBridge>,
    /// name of the bridge
    pub name: String,
    /// uplink-level bridge configuration for each uplink(PF).
    /// currently must contain only one element
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uplinks: Option<Vec<SriovNetworkNodeStateBridgesOvsUplinks>>,
}

/// bridge-level configuration for the bridge
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SriovNetworkNodeStateBridgesOvsBridge {
    /// configure datapath_type field in the Bridge table in OVSDB
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "datapathType")]
    pub datapath_type: Option<String>,
    /// IDs to inject to external_ids field in the Bridge table in OVSDB
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalIDs")]
    pub external_i_ds: Option<BTreeMap<String, String>>,
    /// additional options to inject to other_config field in the bridge table in OVSDB
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "otherConfig")]
    pub other_config: Option<BTreeMap<String, String>>,
}

/// OVSUplinkConfigExt contains configuration for the concrete OVS uplink(PF)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SriovNetworkNodeStateBridgesOvsUplinks {
    /// configuration from the Interface OVS table for the PF
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interface: Option<SriovNetworkNodeStateBridgesOvsUplinksInterface>,
    /// name of the PF interface
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// pci address of the PF
    #[serde(rename = "pciAddress")]
    pub pci_address: String,
}

/// configuration from the Interface OVS table for the PF
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SriovNetworkNodeStateBridgesOvsUplinksInterface {
    /// external_ids field in the Interface table in OVSDB
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalIDs")]
    pub external_i_ds: Option<BTreeMap<String, String>>,
    /// options field in the Interface table in OVSDB
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<BTreeMap<String, String>>,
    /// other_config field in the Interface table in OVSDB
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "otherConfig")]
    pub other_config: Option<BTreeMap<String, String>>,
    /// type field in the Interface table in OVSDB
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SriovNetworkNodeStateInterfaces {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "eSwitchMode")]
    pub e_switch_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externallyManaged")]
    pub externally_managed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "linkType")]
    pub link_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "numVfs")]
    pub num_vfs: Option<i64>,
    #[serde(rename = "pciAddress")]
    pub pci_address: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vfGroups")]
    pub vf_groups: Option<Vec<SriovNetworkNodeStateInterfacesVfGroups>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SriovNetworkNodeStateInterfacesVfGroups {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deviceType")]
    pub device_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isRdma")]
    pub is_rdma: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "policyName")]
    pub policy_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceName")]
    pub resource_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vdpaType")]
    pub vdpa_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vfRange")]
    pub vf_range: Option<String>,
}

/// SriovNetworkNodeStateStatus defines the observed state of SriovNetworkNodeState
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SriovNetworkNodeStateStatus {
    /// Bridges contains list of bridges
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bridges: Option<SriovNetworkNodeStateStatusBridges>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interfaces: Option<Vec<SriovNetworkNodeStateStatusInterfaces>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastSyncError")]
    pub last_sync_error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "syncStatus")]
    pub sync_status: Option<String>,
}

/// Bridges contains list of bridges
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SriovNetworkNodeStateStatusBridges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ovs: Option<Vec<SriovNetworkNodeStateStatusBridgesOvs>>,
}

/// OVSConfigExt contains configuration for the concrete OVS bridge
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SriovNetworkNodeStateStatusBridgesOvs {
    /// bridge-level configuration for the bridge
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bridge: Option<SriovNetworkNodeStateStatusBridgesOvsBridge>,
    /// name of the bridge
    pub name: String,
    /// uplink-level bridge configuration for each uplink(PF).
    /// currently must contain only one element
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uplinks: Option<Vec<SriovNetworkNodeStateStatusBridgesOvsUplinks>>,
}

/// bridge-level configuration for the bridge
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SriovNetworkNodeStateStatusBridgesOvsBridge {
    /// configure datapath_type field in the Bridge table in OVSDB
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "datapathType")]
    pub datapath_type: Option<String>,
    /// IDs to inject to external_ids field in the Bridge table in OVSDB
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalIDs")]
    pub external_i_ds: Option<BTreeMap<String, String>>,
    /// additional options to inject to other_config field in the bridge table in OVSDB
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "otherConfig")]
    pub other_config: Option<BTreeMap<String, String>>,
}

/// OVSUplinkConfigExt contains configuration for the concrete OVS uplink(PF)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SriovNetworkNodeStateStatusBridgesOvsUplinks {
    /// configuration from the Interface OVS table for the PF
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interface: Option<SriovNetworkNodeStateStatusBridgesOvsUplinksInterface>,
    /// name of the PF interface
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// pci address of the PF
    #[serde(rename = "pciAddress")]
    pub pci_address: String,
}

/// configuration from the Interface OVS table for the PF
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SriovNetworkNodeStateStatusBridgesOvsUplinksInterface {
    /// external_ids field in the Interface table in OVSDB
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalIDs")]
    pub external_i_ds: Option<BTreeMap<String, String>>,
    /// options field in the Interface table in OVSDB
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<BTreeMap<String, String>>,
    /// other_config field in the Interface table in OVSDB
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "otherConfig")]
    pub other_config: Option<BTreeMap<String, String>>,
    /// type field in the Interface table in OVSDB
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SriovNetworkNodeStateStatusInterfaces {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Vfs")]
    pub vfs: Option<Vec<SriovNetworkNodeStateStatusInterfacesVfs>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deviceID")]
    pub device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "eSwitchMode")]
    pub e_switch_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externallyManaged")]
    pub externally_managed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "linkAdminState")]
    pub link_admin_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "linkSpeed")]
    pub link_speed: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "linkType")]
    pub link_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "netFilter")]
    pub net_filter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "numVfs")]
    pub num_vfs: Option<i64>,
    #[serde(rename = "pciAddress")]
    pub pci_address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub totalvfs: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SriovNetworkNodeStateStatusInterfacesVfs {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Vlan")]
    pub vlan: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assigned: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deviceID")]
    pub device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pciAddress")]
    pub pci_address: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "representorName")]
    pub representor_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vdpaType")]
    pub vdpa_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[serde(rename = "vfID")]
    pub vf_id: i64,
}

