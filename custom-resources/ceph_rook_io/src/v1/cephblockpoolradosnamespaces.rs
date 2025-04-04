// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/rook/rook/ceph.rook.io/v1/cephblockpoolradosnamespaces.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// Spec represents the specification of a Ceph BlockPool Rados Namespace
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "ceph.rook.io", version = "v1", kind = "CephBlockPoolRadosNamespace", plural = "cephblockpoolradosnamespaces")]
#[kube(namespaced)]
#[kube(status = "CephBlockPoolRadosNamespaceStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct CephBlockPoolRadosNamespaceSpec {
    /// BlockPoolName is the name of Ceph BlockPool. Typically it's the name of
    /// the CephBlockPool CR.
    #[serde(rename = "blockPoolName")]
    pub block_pool_name: String,
    /// Mirroring configuration of CephBlockPoolRadosNamespace
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirroring: Option<CephBlockPoolRadosNamespaceMirroring>,
    /// The name of the CephBlockPoolRadosNamespaceSpec namespace. If not set, the default is the name of the CR.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Mirroring configuration of CephBlockPoolRadosNamespace
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CephBlockPoolRadosNamespaceMirroring {
    /// Mode is the mirroring mode; either pool or image.
    pub mode: CephBlockPoolRadosNamespaceMirroringMode,
    /// RemoteNamespace is the name of the CephBlockPoolRadosNamespace on the secondary cluster CephBlockPool
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "remoteNamespace")]
    pub remote_namespace: Option<String>,
    /// SnapshotSchedules is the scheduling of snapshot for mirrored images
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotSchedules")]
    pub snapshot_schedules: Option<Vec<CephBlockPoolRadosNamespaceMirroringSnapshotSchedules>>,
}

/// Mirroring configuration of CephBlockPoolRadosNamespace
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephBlockPoolRadosNamespaceMirroringMode {
    #[serde(rename = "")]
    KopiumEmpty,
    #[serde(rename = "pool")]
    Pool,
    #[serde(rename = "image")]
    Image,
}

/// SnapshotScheduleSpec represents the snapshot scheduling settings of a mirrored pool
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolRadosNamespaceMirroringSnapshotSchedules {
    /// Interval represent the periodicity of the snapshot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Path is the path to snapshot, only valid for CephFS
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// StartTime indicates when to start the snapshot
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<String>,
}

/// Status represents the status of a CephBlockPool Rados Namespace
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolRadosNamespaceStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<BTreeMap<String, String>>,
    /// MirroringInfoSpec is the status of the pool/radosnamespace mirroring
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mirroringInfo")]
    pub mirroring_info: Option<CephBlockPoolRadosNamespaceStatusMirroringInfo>,
    /// MirroringStatusSpec is the status of the pool/radosNamespace mirroring
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mirroringStatus")]
    pub mirroring_status: Option<CephBlockPoolRadosNamespaceStatusMirroringStatus>,
    /// ConditionType represent a resource's status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// SnapshotScheduleStatusSpec is the status of the snapshot schedule
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotScheduleStatus")]
    pub snapshot_schedule_status: Option<CephBlockPoolRadosNamespaceStatusSnapshotScheduleStatus>,
}

/// MirroringInfoSpec is the status of the pool/radosnamespace mirroring
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolRadosNamespaceStatusMirroringInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastChanged")]
    pub last_changed: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastChecked")]
    pub last_checked: Option<String>,
    /// Mode is the mirroring mode
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Peers are the list of peer sites connected to that cluster
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peers: Option<Vec<CephBlockPoolRadosNamespaceStatusMirroringInfoPeers>>,
    /// SiteName is the current site name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,
}

/// PeersSpec contains peer details
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolRadosNamespaceStatusMirroringInfoPeers {
    /// ClientName is the CephX user used to connect to the peer
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    /// Direction is the peer mirroring direction
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    /// MirrorUUID is the mirror UUID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror_uuid: Option<String>,
    /// SiteName is the current site name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,
    /// UUID is the peer UUID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

/// MirroringStatusSpec is the status of the pool/radosNamespace mirroring
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolRadosNamespaceStatusMirroringStatus {
    /// Details contains potential status errors
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// LastChanged is the last time time the status last changed
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastChanged")]
    pub last_changed: Option<String>,
    /// LastChecked is the last time time the status was checked
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastChecked")]
    pub last_checked: Option<String>,
    /// Summary is the mirroring status summary
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<CephBlockPoolRadosNamespaceStatusMirroringStatusSummary>,
}

/// Summary is the mirroring status summary
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolRadosNamespaceStatusMirroringStatusSummary {
    /// DaemonHealth is the health of the mirroring daemon
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub daemon_health: Option<String>,
    /// Health is the mirroring health
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<String>,
    /// ImageHealth is the health of the mirrored image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_health: Option<String>,
    /// States is the various state for all mirrored images
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub states: Option<CephBlockPoolRadosNamespaceStatusMirroringStatusSummaryStates>,
}

/// States is the various state for all mirrored images
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolRadosNamespaceStatusMirroringStatusSummaryStates {
    /// Error is when the mirroring state is errored
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<i64>,
    /// Replaying is when the replay of the mirroring journal is on-going
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replaying: Option<i64>,
    /// StartingReplay is when the replay of the mirroring journal starts
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starting_replay: Option<i64>,
    /// Stopped is when the mirroring state is stopped
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopped: Option<i64>,
    /// StopReplaying is when the replay of the mirroring journal stops
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopping_replay: Option<i64>,
    /// Syncing is when the image is syncing
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub syncing: Option<i64>,
    /// Unknown is when the mirroring state is unknown
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unknown: Option<i64>,
}

/// SnapshotScheduleStatusSpec is the status of the snapshot schedule
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolRadosNamespaceStatusSnapshotScheduleStatus {
    /// Details contains potential status errors
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// LastChanged is the last time time the status last changed
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastChanged")]
    pub last_changed: Option<String>,
    /// LastChecked is the last time time the status was checked
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastChecked")]
    pub last_checked: Option<String>,
    /// SnapshotSchedules is the list of snapshots scheduled
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotSchedules")]
    pub snapshot_schedules: Option<Vec<CephBlockPoolRadosNamespaceStatusSnapshotScheduleStatusSnapshotSchedules>>,
}

/// SnapshotSchedulesSpec is the list of snapshot scheduled for images in a pool
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolRadosNamespaceStatusSnapshotScheduleStatusSnapshotSchedules {
    /// Image is the mirrored image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Items is the list schedules times for a given snapshot
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CephBlockPoolRadosNamespaceStatusSnapshotScheduleStatusSnapshotSchedulesItems>>,
    /// Namespace is the RADOS namespace the image is part of
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Pool is the pool name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,
}

/// SnapshotSchedule is a schedule
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolRadosNamespaceStatusSnapshotScheduleStatusSnapshotSchedulesItems {
    /// Interval is the interval in which snapshots will be taken
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// StartTime is the snapshot starting time
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

