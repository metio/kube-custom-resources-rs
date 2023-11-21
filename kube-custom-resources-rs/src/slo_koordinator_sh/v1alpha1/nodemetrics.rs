// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/koordinator-sh/koordinator/slo.koordinator.sh/v1alpha1/nodemetrics.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// NodeMetricSpec defines the desired state of NodeMetric
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "slo.koordinator.sh", version = "v1alpha1", kind = "NodeMetric", plural = "nodemetrics")]
#[kube(status = "NodeMetricStatus")]
#[kube(schema = "disabled")]
pub struct NodeMetricSpec {
    /// CollectPolicy defines the Metric collection policy
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metricCollectPolicy")]
    pub metric_collect_policy: Option<NodeMetricMetricCollectPolicy>,
}

/// CollectPolicy defines the Metric collection policy
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricMetricCollectPolicy {
    /// AggregateDurationSeconds represents the aggregation period in seconds
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "aggregateDurationSeconds")]
    pub aggregate_duration_seconds: Option<i64>,
    /// NodeAggregatePolicy represents the target grain of node aggregated usage
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeAggregatePolicy")]
    pub node_aggregate_policy: Option<NodeMetricMetricCollectPolicyNodeAggregatePolicy>,
    /// NodeMemoryPolicy represents apply which method collect memory info
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeMemoryCollectPolicy")]
    pub node_memory_collect_policy: Option<NodeMetricMetricCollectPolicyNodeMemoryCollectPolicy>,
    /// ReportIntervalSeconds represents the report period in seconds
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reportIntervalSeconds")]
    pub report_interval_seconds: Option<i64>,
}

/// NodeAggregatePolicy represents the target grain of node aggregated usage
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricMetricCollectPolicyNodeAggregatePolicy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub durations: Option<Vec<String>>,
}

/// CollectPolicy defines the Metric collection policy
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NodeMetricMetricCollectPolicyNodeMemoryCollectPolicy {
    #[serde(rename = "usageWithHotPageCache")]
    UsageWithHotPageCache,
    #[serde(rename = "usageWithoutPageCache")]
    UsageWithoutPageCache,
    #[serde(rename = "usageWithPageCache")]
    UsageWithPageCache,
}

/// NodeMetricStatus defines the observed state of NodeMetric
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatus {
    /// HostApplicationMetric contains the metrics of out-out-band applications on node.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostApplicationMetric")]
    pub host_application_metric: Option<Vec<NodeMetricStatusHostApplicationMetric>>,
    /// NodeMetric contains the metrics for this node.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeMetric")]
    pub node_metric: Option<NodeMetricStatusNodeMetric>,
    /// PodsMetric contains the metrics for pods belong to this node.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podsMetric")]
    pub pods_metric: Option<Vec<NodeMetricStatusPodsMetric>>,
    /// ProdReclaimableMetric is the indicator statistics of Prod type resources reclaimable
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "prodReclaimableMetric")]
    pub prod_reclaimable_metric: Option<NodeMetricStatusProdReclaimableMetric>,
    /// UpdateTime is the last time this NodeMetric was updated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updateTime")]
    pub update_time: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusHostApplicationMetric {
    /// Name of the host application
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Priority class of the application
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    /// QoS class of the application
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qos: Option<String>,
    /// Resource usage of the host application
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<NodeMetricStatusHostApplicationMetricUsage>,
}

/// Resource usage of the host application
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusHostApplicationMetricUsage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<NodeMetricStatusHostApplicationMetricUsageDevices>>,
    /// ResourceList is a set of (resource name, quantity) pairs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<BTreeMap<String, IntOrString>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusHostApplicationMetricUsageDevices {
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
    pub topology: Option<NodeMetricStatusHostApplicationMetricUsageDevicesTopology>,
    /// Type represents the type of device
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// VFGroups represents the virtual function devices
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vfGroups")]
    pub vf_groups: Option<Vec<NodeMetricStatusHostApplicationMetricUsageDevicesVfGroups>>,
}

/// Topology represents the topology information about the device
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusHostApplicationMetricUsageDevicesTopology {
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
pub struct NodeMetricStatusHostApplicationMetricUsageDevicesVfGroups {
    /// Labels represents the Virtual Function properties that can be used to organize and categorize (scope and select) objects
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// VFs are the virtual function devices which belong to the group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vfs: Option<Vec<NodeMetricStatusHostApplicationMetricUsageDevicesVfGroupsVfs>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusHostApplicationMetricUsageDevicesVfGroupsVfs {
    /// BusID is the domain:bus:device.function formatted identifier of PCI/PCIE virtual function device
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "busID")]
    pub bus_id: Option<String>,
    /// Minor represents the Minor number of VirtualFunction, starting from 0, used to identify virtual function.
    pub minor: i32,
}

/// NodeMetric contains the metrics for this node.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusNodeMetric {
    /// AggregatedNodeUsages will report only if there are enough samples
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "aggregatedNodeUsages")]
    pub aggregated_node_usages: Option<Vec<NodeMetricStatusNodeMetricAggregatedNodeUsages>>,
    /// AggregatedSystemUsages will report only if there are enough samples Deleted pods will be excluded during aggregation
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "aggregatedSystemUsages")]
    pub aggregated_system_usages: Option<Vec<NodeMetricStatusNodeMetricAggregatedSystemUsages>>,
    /// NodeUsage is the total resource usage of node
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeUsage")]
    pub node_usage: Option<NodeMetricStatusNodeMetricNodeUsage>,
    /// SystemUsage is the resource usage of daemon processes and OS kernel, calculated by `NodeUsage - sum(podUsage)`
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "systemUsage")]
    pub system_usage: Option<NodeMetricStatusNodeMetricSystemUsage>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusNodeMetricAggregatedNodeUsages {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<BTreeMap<String, NodeMetricStatusNodeMetricAggregatedNodeUsagesUsage>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusNodeMetricAggregatedNodeUsagesUsage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<NodeMetricStatusNodeMetricAggregatedNodeUsagesUsageDevices>>,
    /// ResourceList is a set of (resource name, quantity) pairs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<BTreeMap<String, IntOrString>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusNodeMetricAggregatedNodeUsagesUsageDevices {
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
    pub topology: Option<NodeMetricStatusNodeMetricAggregatedNodeUsagesUsageDevicesTopology>,
    /// Type represents the type of device
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// VFGroups represents the virtual function devices
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vfGroups")]
    pub vf_groups: Option<Vec<NodeMetricStatusNodeMetricAggregatedNodeUsagesUsageDevicesVfGroups>>,
}

/// Topology represents the topology information about the device
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusNodeMetricAggregatedNodeUsagesUsageDevicesTopology {
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
pub struct NodeMetricStatusNodeMetricAggregatedNodeUsagesUsageDevicesVfGroups {
    /// Labels represents the Virtual Function properties that can be used to organize and categorize (scope and select) objects
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// VFs are the virtual function devices which belong to the group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vfs: Option<Vec<NodeMetricStatusNodeMetricAggregatedNodeUsagesUsageDevicesVfGroupsVfs>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusNodeMetricAggregatedNodeUsagesUsageDevicesVfGroupsVfs {
    /// BusID is the domain:bus:device.function formatted identifier of PCI/PCIE virtual function device
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "busID")]
    pub bus_id: Option<String>,
    /// Minor represents the Minor number of VirtualFunction, starting from 0, used to identify virtual function.
    pub minor: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusNodeMetricAggregatedSystemUsages {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<BTreeMap<String, NodeMetricStatusNodeMetricAggregatedSystemUsagesUsage>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusNodeMetricAggregatedSystemUsagesUsage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<NodeMetricStatusNodeMetricAggregatedSystemUsagesUsageDevices>>,
    /// ResourceList is a set of (resource name, quantity) pairs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<BTreeMap<String, IntOrString>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusNodeMetricAggregatedSystemUsagesUsageDevices {
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
    pub topology: Option<NodeMetricStatusNodeMetricAggregatedSystemUsagesUsageDevicesTopology>,
    /// Type represents the type of device
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// VFGroups represents the virtual function devices
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vfGroups")]
    pub vf_groups: Option<Vec<NodeMetricStatusNodeMetricAggregatedSystemUsagesUsageDevicesVfGroups>>,
}

/// Topology represents the topology information about the device
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusNodeMetricAggregatedSystemUsagesUsageDevicesTopology {
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
pub struct NodeMetricStatusNodeMetricAggregatedSystemUsagesUsageDevicesVfGroups {
    /// Labels represents the Virtual Function properties that can be used to organize and categorize (scope and select) objects
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// VFs are the virtual function devices which belong to the group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vfs: Option<Vec<NodeMetricStatusNodeMetricAggregatedSystemUsagesUsageDevicesVfGroupsVfs>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusNodeMetricAggregatedSystemUsagesUsageDevicesVfGroupsVfs {
    /// BusID is the domain:bus:device.function formatted identifier of PCI/PCIE virtual function device
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "busID")]
    pub bus_id: Option<String>,
    /// Minor represents the Minor number of VirtualFunction, starting from 0, used to identify virtual function.
    pub minor: i32,
}

/// NodeUsage is the total resource usage of node
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusNodeMetricNodeUsage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<NodeMetricStatusNodeMetricNodeUsageDevices>>,
    /// ResourceList is a set of (resource name, quantity) pairs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<BTreeMap<String, IntOrString>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusNodeMetricNodeUsageDevices {
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
    pub topology: Option<NodeMetricStatusNodeMetricNodeUsageDevicesTopology>,
    /// Type represents the type of device
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// VFGroups represents the virtual function devices
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vfGroups")]
    pub vf_groups: Option<Vec<NodeMetricStatusNodeMetricNodeUsageDevicesVfGroups>>,
}

/// Topology represents the topology information about the device
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusNodeMetricNodeUsageDevicesTopology {
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
pub struct NodeMetricStatusNodeMetricNodeUsageDevicesVfGroups {
    /// Labels represents the Virtual Function properties that can be used to organize and categorize (scope and select) objects
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// VFs are the virtual function devices which belong to the group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vfs: Option<Vec<NodeMetricStatusNodeMetricNodeUsageDevicesVfGroupsVfs>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusNodeMetricNodeUsageDevicesVfGroupsVfs {
    /// BusID is the domain:bus:device.function formatted identifier of PCI/PCIE virtual function device
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "busID")]
    pub bus_id: Option<String>,
    /// Minor represents the Minor number of VirtualFunction, starting from 0, used to identify virtual function.
    pub minor: i32,
}

/// SystemUsage is the resource usage of daemon processes and OS kernel, calculated by `NodeUsage - sum(podUsage)`
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusNodeMetricSystemUsage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<NodeMetricStatusNodeMetricSystemUsageDevices>>,
    /// ResourceList is a set of (resource name, quantity) pairs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<BTreeMap<String, IntOrString>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusNodeMetricSystemUsageDevices {
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
    pub topology: Option<NodeMetricStatusNodeMetricSystemUsageDevicesTopology>,
    /// Type represents the type of device
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// VFGroups represents the virtual function devices
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vfGroups")]
    pub vf_groups: Option<Vec<NodeMetricStatusNodeMetricSystemUsageDevicesVfGroups>>,
}

/// Topology represents the topology information about the device
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusNodeMetricSystemUsageDevicesTopology {
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
pub struct NodeMetricStatusNodeMetricSystemUsageDevicesVfGroups {
    /// Labels represents the Virtual Function properties that can be used to organize and categorize (scope and select) objects
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// VFs are the virtual function devices which belong to the group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vfs: Option<Vec<NodeMetricStatusNodeMetricSystemUsageDevicesVfGroupsVfs>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusNodeMetricSystemUsageDevicesVfGroupsVfs {
    /// BusID is the domain:bus:device.function formatted identifier of PCI/PCIE virtual function device
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "busID")]
    pub bus_id: Option<String>,
    /// Minor represents the Minor number of VirtualFunction, starting from 0, used to identify virtual function.
    pub minor: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusPodsMetric {
    /// Third party extensions for PodMetric
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podUsage")]
    pub pod_usage: Option<NodeMetricStatusPodsMetricPodUsage>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusPodsMetricPodUsage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<NodeMetricStatusPodsMetricPodUsageDevices>>,
    /// ResourceList is a set of (resource name, quantity) pairs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<BTreeMap<String, IntOrString>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusPodsMetricPodUsageDevices {
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
    pub topology: Option<NodeMetricStatusPodsMetricPodUsageDevicesTopology>,
    /// Type represents the type of device
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// VFGroups represents the virtual function devices
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vfGroups")]
    pub vf_groups: Option<Vec<NodeMetricStatusPodsMetricPodUsageDevicesVfGroups>>,
}

/// Topology represents the topology information about the device
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusPodsMetricPodUsageDevicesTopology {
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
pub struct NodeMetricStatusPodsMetricPodUsageDevicesVfGroups {
    /// Labels represents the Virtual Function properties that can be used to organize and categorize (scope and select) objects
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// VFs are the virtual function devices which belong to the group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vfs: Option<Vec<NodeMetricStatusPodsMetricPodUsageDevicesVfGroupsVfs>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusPodsMetricPodUsageDevicesVfGroupsVfs {
    /// BusID is the domain:bus:device.function formatted identifier of PCI/PCIE virtual function device
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "busID")]
    pub bus_id: Option<String>,
    /// Minor represents the Minor number of VirtualFunction, starting from 0, used to identify virtual function.
    pub minor: i32,
}

/// ProdReclaimableMetric is the indicator statistics of Prod type resources reclaimable
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusProdReclaimableMetric {
    /// Resource is the resource usage of the prediction
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<NodeMetricStatusProdReclaimableMetricResource>,
}

/// Resource is the resource usage of the prediction
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusProdReclaimableMetricResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<NodeMetricStatusProdReclaimableMetricResourceDevices>>,
    /// ResourceList is a set of (resource name, quantity) pairs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<BTreeMap<String, IntOrString>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusProdReclaimableMetricResourceDevices {
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
    pub topology: Option<NodeMetricStatusProdReclaimableMetricResourceDevicesTopology>,
    /// Type represents the type of device
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// VFGroups represents the virtual function devices
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vfGroups")]
    pub vf_groups: Option<Vec<NodeMetricStatusProdReclaimableMetricResourceDevicesVfGroups>>,
}

/// Topology represents the topology information about the device
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusProdReclaimableMetricResourceDevicesTopology {
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
pub struct NodeMetricStatusProdReclaimableMetricResourceDevicesVfGroups {
    /// Labels represents the Virtual Function properties that can be used to organize and categorize (scope and select) objects
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// VFs are the virtual function devices which belong to the group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vfs: Option<Vec<NodeMetricStatusProdReclaimableMetricResourceDevicesVfGroupsVfs>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeMetricStatusProdReclaimableMetricResourceDevicesVfGroupsVfs {
    /// BusID is the domain:bus:device.function formatted identifier of PCI/PCIE virtual function device
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "busID")]
    pub bus_id: Option<String>,
    /// Minor represents the Minor number of VirtualFunction, starting from 0, used to identify virtual function.
    pub minor: i32,
}
