// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubernetes-sigs/cluster-api-provider-vsphere/infrastructure.cluster.x-k8s.io/v1alpha4/vspherefailuredomains.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// VSphereFailureDomainSpec defines the desired state of VSphereFailureDomain
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "infrastructure.cluster.x-k8s.io", version = "v1alpha4", kind = "VSphereFailureDomain", plural = "vspherefailuredomains")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct VSphereFailureDomainSpec {
    /// Region defines the name and type of a region
    pub region: VSphereFailureDomainRegion,
    /// Topology describes a given failure domain using vSphere constructs
    pub topology: VSphereFailureDomainTopology,
    /// Zone defines the name and type of a zone
    pub zone: VSphereFailureDomainZone,
}

/// Region defines the name and type of a region
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VSphereFailureDomainRegion {
    /// AutoConfigure tags the Type which is specified in the Topology
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoConfigure")]
    pub auto_configure: Option<bool>,
    /// Name is the name of the tag that represents this failure domain
    pub name: String,
    /// TagCategory is the category used for the tag
    #[serde(rename = "tagCategory")]
    pub tag_category: String,
    /// Type is the type of failure domain, the current values are "Datacenter", "ComputeCluster" and "HostGroup"
    #[serde(rename = "type")]
    pub r#type: VSphereFailureDomainRegionType,
}

/// Region defines the name and type of a region
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VSphereFailureDomainRegionType {
    Datacenter,
    ComputeCluster,
    HostGroup,
}

/// Topology describes a given failure domain using vSphere constructs
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereFailureDomainTopology {
    /// ComputeCluster as the failure domain
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "computeCluster")]
    pub compute_cluster: Option<String>,
    /// The underlying infrastructure for this failure domain
    /// Datacenter as the failure domain
    pub datacenter: String,
    /// Datastore is the name or inventory path of the datastore in which the
    /// virtual machine is created/located.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datastore: Option<String>,
    /// Hosts has information required for placement of machines on VSphere hosts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosts: Option<VSphereFailureDomainTopologyHosts>,
    /// Networks is the list of networks within this failure domain
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<String>>,
}

/// Hosts has information required for placement of machines on VSphere hosts.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereFailureDomainTopologyHosts {
    /// HostGroupName is the name of the Host group
    #[serde(rename = "hostGroupName")]
    pub host_group_name: String,
    /// VMGroupName is the name of the VM group
    #[serde(rename = "vmGroupName")]
    pub vm_group_name: String,
}

/// Zone defines the name and type of a zone
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VSphereFailureDomainZone {
    /// AutoConfigure tags the Type which is specified in the Topology
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoConfigure")]
    pub auto_configure: Option<bool>,
    /// Name is the name of the tag that represents this failure domain
    pub name: String,
    /// TagCategory is the category used for the tag
    #[serde(rename = "tagCategory")]
    pub tag_category: String,
    /// Type is the type of failure domain, the current values are "Datacenter", "ComputeCluster" and "HostGroup"
    #[serde(rename = "type")]
    pub r#type: VSphereFailureDomainZoneType,
}

/// Zone defines the name and type of a zone
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VSphereFailureDomainZoneType {
    Datacenter,
    ComputeCluster,
    HostGroup,
}

