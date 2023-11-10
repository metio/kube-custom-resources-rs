// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/longhorn/longhorn/longhorn.io/v1beta2/replicas.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// ReplicaSpec defines the desired state of the Longhorn replica
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "longhorn.io", version = "v1beta2", kind = "Replica", plural = "replicas")]
#[kube(namespaced)]
#[kube(status = "ReplicaStatus")]
#[kube(schema = "disabled")]
pub struct ReplicaSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backendStoreDriver")]
    pub backend_store_driver: Option<ReplicaBackendStoreDriver>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backingImage")]
    pub backing_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataDirectoryName")]
    pub data_directory_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "desireState")]
    pub desire_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "diskID")]
    pub disk_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "diskPath")]
    pub disk_path: Option<String>,
    /// Deprecated: Replaced by field `image`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "engineImage")]
    pub engine_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "engineName")]
    pub engine_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failedAt")]
    pub failed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hardNodeAffinity")]
    pub hard_node_affinity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "healthyAt")]
    pub healthy_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logRequested")]
    pub log_requested: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeID")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rebuildRetryCount")]
    pub rebuild_retry_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "revisionCounterDisabled")]
    pub revision_counter_disabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "salvageRequested")]
    pub salvage_requested: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unmapMarkDiskChainRemovedEnabled")]
    pub unmap_mark_disk_chain_removed_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeName")]
    pub volume_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeSize")]
    pub volume_size: Option<String>,
}

/// ReplicaSpec defines the desired state of the Longhorn replica
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ReplicaBackendStoreDriver {
    #[serde(rename = "v1")]
    V1,
    #[serde(rename = "v2")]
    V2,
}

/// ReplicaStatus defines the observed state of the Longhorn replica
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReplicaStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ReplicaStatusConditions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentImage")]
    pub current_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentState")]
    pub current_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "evictionRequested")]
    pub eviction_requested: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceManagerName")]
    pub instance_manager_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logFetched")]
    pub log_fetched: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "salvageExecuted")]
    pub salvage_executed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageIP")]
    pub storage_ip: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReplicaStatusConditions {
    /// Last time we probed the condition.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastProbeTime")]
    pub last_probe_time: Option<String>,
    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// Human-readable message indicating details about last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Status is the status of the condition. Can be True, False, Unknown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Type is the type of the condition.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

