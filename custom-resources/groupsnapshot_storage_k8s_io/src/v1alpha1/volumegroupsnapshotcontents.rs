// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubernetes-csi/external-snapshotter/groupsnapshot.storage.k8s.io/v1alpha1/volumegroupsnapshotcontents.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::api::core::v1::ObjectReference;
}
use self::prelude::*;

/// Spec defines properties of a VolumeGroupSnapshotContent created by the underlying storage system.
/// Required.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "groupsnapshot.storage.k8s.io", version = "v1alpha1", kind = "VolumeGroupSnapshotContent", plural = "volumegroupsnapshotcontents")]
#[kube(status = "VolumeGroupSnapshotContentStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct VolumeGroupSnapshotContentSpec {
    /// DeletionPolicy determines whether this VolumeGroupSnapshotContent and the
    /// physical group snapshot on the underlying storage system should be deleted
    /// when the bound VolumeGroupSnapshot is deleted.
    /// Supported values are "Retain" and "Delete".
    /// "Retain" means that the VolumeGroupSnapshotContent and its physical group
    /// snapshot on underlying storage system are kept.
    /// "Delete" means that the VolumeGroupSnapshotContent and its physical group
    /// snapshot on underlying storage system are deleted.
    /// For dynamically provisioned group snapshots, this field will automatically
    /// be filled in by the CSI snapshotter sidecar with the "DeletionPolicy" field
    /// defined in the corresponding VolumeGroupSnapshotClass.
    /// For pre-existing snapshots, users MUST specify this field when creating the
    /// VolumeGroupSnapshotContent object.
    /// Required.
    #[serde(rename = "deletionPolicy")]
    pub deletion_policy: VolumeGroupSnapshotContentDeletionPolicy,
    /// Driver is the name of the CSI driver used to create the physical group snapshot on
    /// the underlying storage system.
    /// This MUST be the same as the name returned by the CSI GetPluginName() call for
    /// that driver.
    /// Required.
    pub driver: String,
    /// Source specifies whether the snapshot is (or should be) dynamically provisioned
    /// or already exists, and just requires a Kubernetes object representation.
    /// This field is immutable after creation.
    /// Required.
    pub source: VolumeGroupSnapshotContentSource,
    /// VolumeGroupSnapshotClassName is the name of the VolumeGroupSnapshotClass from
    /// which this group snapshot was (or will be) created.
    /// Note that after provisioning, the VolumeGroupSnapshotClass may be deleted or
    /// recreated with different set of values, and as such, should not be referenced
    /// post-snapshot creation.
    /// For dynamic provisioning, this field must be set.
    /// This field may be unset for pre-provisioned snapshots.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeGroupSnapshotClassName")]
    pub volume_group_snapshot_class_name: Option<String>,
    /// VolumeGroupSnapshotRef specifies the VolumeGroupSnapshot object to which this
    /// VolumeGroupSnapshotContent object is bound.
    /// VolumeGroupSnapshot.Spec.VolumeGroupSnapshotContentName field must reference to
    /// this VolumeGroupSnapshotContent's name for the bidirectional binding to be valid.
    /// For a pre-existing VolumeGroupSnapshotContent object, name and namespace of the
    /// VolumeGroupSnapshot object MUST be provided for binding to happen.
    /// This field is immutable after creation.
    /// Required.
    #[serde(rename = "volumeGroupSnapshotRef")]
    pub volume_group_snapshot_ref: ObjectReference,
}

/// Spec defines properties of a VolumeGroupSnapshotContent created by the underlying storage system.
/// Required.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VolumeGroupSnapshotContentDeletionPolicy {
    Delete,
    Retain,
}

/// Source specifies whether the snapshot is (or should be) dynamically provisioned
/// or already exists, and just requires a Kubernetes object representation.
/// This field is immutable after creation.
/// Required.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeGroupSnapshotContentSource {
    /// GroupSnapshotHandles specifies the CSI "group_snapshot_id" of a pre-existing
    /// group snapshot and a list of CSI "snapshot_id" of pre-existing snapshots
    /// on the underlying storage system for which a Kubernetes object
    /// representation was (or should be) created.
    /// This field is immutable.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "groupSnapshotHandles")]
    pub group_snapshot_handles: Option<VolumeGroupSnapshotContentSourceGroupSnapshotHandles>,
    /// VolumeHandles is a list of volume handles on the backend to be snapshotted
    /// together. It is specified for dynamic provisioning of the VolumeGroupSnapshot.
    /// This field is immutable.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeHandles")]
    pub volume_handles: Option<Vec<String>>,
}

/// GroupSnapshotHandles specifies the CSI "group_snapshot_id" of a pre-existing
/// group snapshot and a list of CSI "snapshot_id" of pre-existing snapshots
/// on the underlying storage system for which a Kubernetes object
/// representation was (or should be) created.
/// This field is immutable.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeGroupSnapshotContentSourceGroupSnapshotHandles {
    /// VolumeGroupSnapshotHandle specifies the CSI "group_snapshot_id" of a pre-existing
    /// group snapshot on the underlying storage system for which a Kubernetes object
    /// representation was (or should be) created.
    /// This field is immutable.
    /// Required.
    #[serde(rename = "volumeGroupSnapshotHandle")]
    pub volume_group_snapshot_handle: String,
    /// VolumeSnapshotHandles is a list of CSI "snapshot_id" of pre-existing
    /// snapshots on the underlying storage system for which Kubernetes objects
    /// representation were (or should be) created.
    /// This field is immutable.
    /// Required.
    #[serde(rename = "volumeSnapshotHandles")]
    pub volume_snapshot_handles: Vec<String>,
}

/// VolumeGroupSnapshotRef specifies the VolumeGroupSnapshot object to which this
/// VolumeGroupSnapshotContent object is bound.
/// VolumeGroupSnapshot.Spec.VolumeGroupSnapshotContentName field must reference to
/// this VolumeGroupSnapshotContent's name for the bidirectional binding to be valid.
/// For a pre-existing VolumeGroupSnapshotContent object, name and namespace of the
/// VolumeGroupSnapshot object MUST be provided for binding to happen.
/// This field is immutable after creation.
/// Required.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeGroupSnapshotContentVolumeGroupSnapshotRef {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string
    /// should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2].
    /// For example, if the object reference is to a container within a pod, this would take on a value like:
    /// "spec.containers{name}" (where "name" refers to the name of the container that triggered
    /// the event) or if no container name is specified "spec.containers[2]" (container with
    /// index 2 in this pod). This syntax is chosen only to have some well-defined way of
    /// referencing a part of an object.
    /// TODO: this design is not final and this field is subject to change in the future.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// status represents the current information of a group snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeGroupSnapshotContentStatus {
    /// CreationTime is the timestamp when the point-in-time group snapshot is taken
    /// by the underlying storage system.
    /// If not specified, it indicates the creation time is unknown.
    /// If not specified, it means the readiness of a group snapshot is unknown.
    /// The format of this field is a Unix nanoseconds time encoded as an int64.
    /// On Unix, the command date +%s%N returns the current time in nanoseconds
    /// since 1970-01-01 00:00:00 UTC.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "creationTime")]
    pub creation_time: Option<i64>,
    /// Error is the last observed error during group snapshot creation, if any.
    /// Upon success after retry, this error field will be cleared.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<VolumeGroupSnapshotContentStatusError>,
    /// PVVolumeSnapshotContentList is the list of pairs of PV and
    /// VolumeSnapshotContent for this group snapshot
    /// The maximum number of allowed snapshots in the group is 100.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pvVolumeSnapshotContentList")]
    pub pv_volume_snapshot_content_list: Option<Vec<VolumeGroupSnapshotContentStatusPvVolumeSnapshotContentList>>,
    /// ReadyToUse indicates if all the individual snapshots in the group are ready to be
    /// used to restore a group of volumes.
    /// ReadyToUse becomes true when ReadyToUse of all individual snapshots become true.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readyToUse")]
    pub ready_to_use: Option<bool>,
    /// VolumeGroupSnapshotHandle is a unique id returned by the CSI driver
    /// to identify the VolumeGroupSnapshot on the storage system.
    /// If a storage system does not provide such an id, the
    /// CSI driver can choose to return the VolumeGroupSnapshot name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeGroupSnapshotHandle")]
    pub volume_group_snapshot_handle: Option<String>,
    /// VolumeSnapshotHandlePairList is a list of CSI "volume_id" and "snapshot_id"
    /// pair returned by the CSI driver to identify snapshots and their source volumes
    /// on the storage system.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeSnapshotHandlePairList")]
    pub volume_snapshot_handle_pair_list: Option<Vec<VolumeGroupSnapshotContentStatusVolumeSnapshotHandlePairList>>,
}

/// Error is the last observed error during group snapshot creation, if any.
/// Upon success after retry, this error field will be cleared.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeGroupSnapshotContentStatusError {
    /// message is a string detailing the encountered error during snapshot
    /// creation if specified.
    /// NOTE: message may be logged, and it should not contain sensitive
    /// information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// time is the timestamp when the error was encountered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

/// PVVolumeSnapshotContentPair represent a pair of PV names and
/// VolumeSnapshotContent names
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeGroupSnapshotContentStatusPvVolumeSnapshotContentList {
    /// PersistentVolumeRef is a reference to the persistent volume resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "persistentVolumeRef")]
    pub persistent_volume_ref: Option<VolumeGroupSnapshotContentStatusPvVolumeSnapshotContentListPersistentVolumeRef>,
    /// VolumeSnapshotContentRef is a reference to the volume snapshot content resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeSnapshotContentRef")]
    pub volume_snapshot_content_ref: Option<VolumeGroupSnapshotContentStatusPvVolumeSnapshotContentListVolumeSnapshotContentRef>,
}

/// PersistentVolumeRef is a reference to the persistent volume resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeGroupSnapshotContentStatusPvVolumeSnapshotContentListPersistentVolumeRef {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// VolumeSnapshotContentRef is a reference to the volume snapshot content resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeGroupSnapshotContentStatusPvVolumeSnapshotContentListVolumeSnapshotContentRef {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// VolumeSnapshotHandlePair defines a pair of a source volume handle and a snapshot handle
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeGroupSnapshotContentStatusVolumeSnapshotHandlePairList {
    /// SnapshotHandle is a unique id returned by the CSI driver to identify a volume
    /// snapshot on the storage system
    #[serde(rename = "snapshotHandle")]
    pub snapshot_handle: String,
    /// VolumeHandle is a unique id returned by the CSI driver to identify a volume
    /// on the storage system
    #[serde(rename = "volumeHandle")]
    pub volume_handle: String,
}

