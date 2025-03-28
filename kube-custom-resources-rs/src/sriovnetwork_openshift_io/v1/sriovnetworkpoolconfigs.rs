// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/openshift/sriov-network-operator/sriovnetwork.openshift.io/v1/sriovnetworkpoolconfigs.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
}
use self::prelude::*;

/// SriovNetworkPoolConfigSpec defines the desired state of SriovNetworkPoolConfig
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "sriovnetwork.openshift.io", version = "v1", kind = "SriovNetworkPoolConfig", plural = "sriovnetworkpoolconfigs")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct SriovNetworkPoolConfigSpec {
    /// maxUnavailable defines either an integer number or percentage
    /// of nodes in the pool that can go Unavailable during an update.
    /// 
    /// 
    /// A value larger than 1 will mean multiple nodes going unavailable during
    /// the update, which may affect your workload stress on the remaining nodes.
    /// Drain will respect Pod Disruption Budgets (PDBs) such as etcd quorum guards,
    /// even if maxUnavailable is greater than one.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxUnavailable")]
    pub max_unavailable: Option<IntOrString>,
    /// nodeSelector specifies a label selector for Nodes
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<SriovNetworkPoolConfigNodeSelector>,
    /// OvsHardwareOffloadConfig describes the OVS HWOL configuration for selected Nodes
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ovsHardwareOffloadConfig")]
    pub ovs_hardware_offload_config: Option<SriovNetworkPoolConfigOvsHardwareOffloadConfig>,
    /// RDMA subsystem. Allowed value "shared", "exclusive".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rdmaMode")]
    pub rdma_mode: Option<SriovNetworkPoolConfigRdmaMode>,
}

/// nodeSelector specifies a label selector for Nodes
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SriovNetworkPoolConfigNodeSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<SriovNetworkPoolConfigNodeSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SriovNetworkPoolConfigNodeSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// OvsHardwareOffloadConfig describes the OVS HWOL configuration for selected Nodes
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SriovNetworkPoolConfigOvsHardwareOffloadConfig {
    /// Name is mandatory and must be unique.
    /// On Kubernetes:
    /// Name is the name of OvsHardwareOffloadConfig
    /// On OpenShift:
    /// Name is the name of MachineConfigPool to be enabled with OVS hardware offload
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// SriovNetworkPoolConfigSpec defines the desired state of SriovNetworkPoolConfig
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SriovNetworkPoolConfigRdmaMode {
    #[serde(rename = "shared")]
    Shared,
    #[serde(rename = "exclusive")]
    Exclusive,
}

/// SriovNetworkPoolConfigStatus defines the observed state of SriovNetworkPoolConfig
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SriovNetworkPoolConfigStatus {
}

