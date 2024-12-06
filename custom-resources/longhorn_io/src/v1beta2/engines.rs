// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/longhorn/longhorn/longhorn.io/v1beta2/engines.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// EngineSpec defines the desired state of the Longhorn engine
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "longhorn.io", version = "v1beta2", kind = "Engine", plural = "engines")]
#[kube(namespaced)]
#[kube(status = "EngineStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct EngineSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Deprecated:Replaced by field `dataEngine`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backendStoreDriver")]
    pub backend_store_driver: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupVolume")]
    pub backup_volume: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataEngine")]
    pub data_engine: Option<EngineDataEngine>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "desireState")]
    pub desire_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disableFrontend")]
    pub disable_frontend: Option<bool>,
    /// Deprecated: Replaced by field `image`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "engineImage")]
    pub engine_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frontend: Option<EngineFrontend>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logRequested")]
    pub log_requested: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeID")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaAddressMap")]
    pub replica_address_map: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestedBackupRestore")]
    pub requested_backup_restore: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestedDataSource")]
    pub requested_data_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "revisionCounterDisabled")]
    pub revision_counter_disabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "salvageRequested")]
    pub salvage_requested: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotMaxCount")]
    pub snapshot_max_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotMaxSize")]
    pub snapshot_max_size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unmapMarkSnapChainRemovedEnabled")]
    pub unmap_mark_snap_chain_removed_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "upgradedReplicaAddressMap")]
    pub upgraded_replica_address_map: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeName")]
    pub volume_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeSize")]
    pub volume_size: Option<String>,
}

/// EngineSpec defines the desired state of the Longhorn engine
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EngineDataEngine {
    #[serde(rename = "v1")]
    V1,
    #[serde(rename = "v2")]
    V2,
}

/// EngineSpec defines the desired state of the Longhorn engine
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EngineFrontend {
    #[serde(rename = "blockdev")]
    Blockdev,
    #[serde(rename = "iscsi")]
    Iscsi,
    #[serde(rename = "nvmf")]
    Nvmf,
    #[serde(rename = "")]
    KopiumEmpty,
}

/// EngineStatus defines the observed state of the Longhorn engine
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EngineStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupStatus")]
    pub backup_status: Option<BTreeMap<String, EngineStatusBackupStatus>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cloneStatus")]
    pub clone_status: Option<BTreeMap<String, EngineStatusCloneStatus>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentImage")]
    pub current_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentReplicaAddressMap")]
    pub current_replica_address_map: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentSize")]
    pub current_size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentState")]
    pub current_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceManagerName")]
    pub instance_manager_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isExpanding")]
    pub is_expanding: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastExpansionError")]
    pub last_expansion_error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastExpansionFailedAt")]
    pub last_expansion_failed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastRestoredBackup")]
    pub last_restored_backup: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logFetched")]
    pub log_fetched: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "purgeStatus")]
    pub purge_status: Option<BTreeMap<String, EngineStatusPurgeStatus>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rebuildStatus")]
    pub rebuild_status: Option<BTreeMap<String, EngineStatusRebuildStatus>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaModeMap")]
    pub replica_mode_map: Option<BTreeMap<String, String>>,
    /// ReplicaTransitionTimeMap records the time a replica in ReplicaModeMap transitions from one mode to another (or
    /// from not being in the ReplicaModeMap to being in it). This information is sometimes required by other controllers
    /// (e.g. the volume controller uses it to determine the correct value for replica.Spec.lastHealthyAt).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaTransitionTimeMap")]
    pub replica_transition_time_map: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "restoreStatus")]
    pub restore_status: Option<BTreeMap<String, EngineStatusRestoreStatus>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "salvageExecuted")]
    pub salvage_executed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotMaxCount")]
    pub snapshot_max_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotMaxSize")]
    pub snapshot_max_size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshots: Option<BTreeMap<String, EngineStatusSnapshots>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotsError")]
    pub snapshots_error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageIP")]
    pub storage_ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unmapMarkSnapChainRemovedEnabled")]
    pub unmap_mark_snap_chain_removed_enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EngineStatusBackupStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupURL")]
    pub backup_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaAddress")]
    pub replica_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotName")]
    pub snapshot_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EngineStatusCloneStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fromReplicaAddress")]
    pub from_replica_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isCloning")]
    pub is_cloning: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotName")]
    pub snapshot_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EngineStatusPurgeStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isPurging")]
    pub is_purging: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EngineStatusRebuildStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fromReplicaAddress")]
    pub from_replica_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isRebuilding")]
    pub is_rebuilding: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EngineStatusRestoreStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupURL")]
    pub backup_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentRestoringBackup")]
    pub current_restoring_backup: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isRestoring")]
    pub is_restoring: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastRestored")]
    pub last_restored: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EngineStatusSnapshots {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<BTreeMap<String, bool>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub removed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usercreated: Option<bool>,
}

