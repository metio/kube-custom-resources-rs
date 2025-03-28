// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-csi/external-snapshotter/groupsnapshot.storage.k8s.io/v1alpha1/volumegroupsnapshots.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// Spec defines the desired characteristics of a group snapshot requested by a user.
/// Required.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "groupsnapshot.storage.k8s.io", version = "v1alpha1", kind = "VolumeGroupSnapshot", plural = "volumegroupsnapshots")]
#[kube(namespaced)]
#[kube(status = "VolumeGroupSnapshotStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct VolumeGroupSnapshotSpec {
    /// Source specifies where a group snapshot will be created from.
    /// This field is immutable after creation.
    /// Required.
    pub source: VolumeGroupSnapshotSource,
    /// VolumeGroupSnapshotClassName is the name of the VolumeGroupSnapshotClass
    /// requested by the VolumeGroupSnapshot.
    /// VolumeGroupSnapshotClassName may be left nil to indicate that the default
    /// class will be used.
    /// Empty string is not allowed for this field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeGroupSnapshotClassName")]
    pub volume_group_snapshot_class_name: Option<String>,
}

/// Source specifies where a group snapshot will be created from.
/// This field is immutable after creation.
/// Required.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeGroupSnapshotSource {
    /// Selector is a label query over persistent volume claims that are to be
    /// grouped together for snapshotting.
    /// This labelSelector will be used to match the label added to a PVC.
    /// If the label is added or removed to a volume after a group snapshot
    /// is created, the existing group snapshots won't be modified.
    /// Once a VolumeGroupSnapshotContent is created and the sidecar starts to process
    /// it, the volume list will not change with retries.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<VolumeGroupSnapshotSourceSelector>,
    /// VolumeGroupSnapshotContentName specifies the name of a pre-existing VolumeGroupSnapshotContent
    /// object representing an existing volume group snapshot.
    /// This field should be set if the volume group snapshot already exists and
    /// only needs a representation in Kubernetes.
    /// This field is immutable.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeGroupSnapshotContentName")]
    pub volume_group_snapshot_content_name: Option<String>,
}

/// Selector is a label query over persistent volume claims that are to be
/// grouped together for snapshotting.
/// This labelSelector will be used to match the label added to a PVC.
/// If the label is added or removed to a volume after a group snapshot
/// is created, the existing group snapshots won't be modified.
/// Once a VolumeGroupSnapshotContent is created and the sidecar starts to process
/// it, the volume list will not change with retries.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeGroupSnapshotSourceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<VolumeGroupSnapshotSourceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeGroupSnapshotSourceSelectorMatchExpressions {
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

/// Status represents the current information of a group snapshot.
/// Consumers must verify binding between VolumeGroupSnapshot and
/// VolumeGroupSnapshotContent objects is successful (by validating that both
/// VolumeGroupSnapshot and VolumeGroupSnapshotContent point to each other) before
/// using this object.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeGroupSnapshotStatus {
    /// BoundVolumeGroupSnapshotContentName is the name of the VolumeGroupSnapshotContent
    /// object to which this VolumeGroupSnapshot object intends to bind to.
    /// If not specified, it indicates that the VolumeGroupSnapshot object has not
    /// been successfully bound to a VolumeGroupSnapshotContent object yet.
    /// NOTE: To avoid possible security issues, consumers must verify binding between
    /// VolumeGroupSnapshot and VolumeGroupSnapshotContent objects is successful
    /// (by validating that both VolumeGroupSnapshot and VolumeGroupSnapshotContent
    /// point at each other) before using this object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "boundVolumeGroupSnapshotContentName")]
    pub bound_volume_group_snapshot_content_name: Option<String>,
    /// CreationTime is the timestamp when the point-in-time group snapshot is taken
    /// by the underlying storage system.
    /// If not specified, it may indicate that the creation time of the group snapshot
    /// is unknown.
    /// The format of this field is a Unix nanoseconds time encoded as an int64.
    /// On Unix, the command date +%s%N returns the current time in nanoseconds
    /// since 1970-01-01 00:00:00 UTC.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "creationTime")]
    pub creation_time: Option<String>,
    /// Error is the last observed error during group snapshot creation, if any.
    /// This field could be helpful to upper level controllers (i.e., application
    /// controller) to decide whether they should continue on waiting for the group
    /// snapshot to be created based on the type of error reported.
    /// The snapshot controller will keep retrying when an error occurs during the
    /// group snapshot creation. Upon success, this error field will be cleared.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<VolumeGroupSnapshotStatusError>,
    /// VolumeSnapshotRefList is the list of PVC and VolumeSnapshot pairs that
    /// is part of this group snapshot.
    /// The maximum number of allowed snapshots in the group is 100.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pvcVolumeSnapshotRefList")]
    pub pvc_volume_snapshot_ref_list: Option<Vec<VolumeGroupSnapshotStatusPvcVolumeSnapshotRefList>>,
    /// ReadyToUse indicates if all the individual snapshots in the group are ready
    /// to be used to restore a group of volumes.
    /// ReadyToUse becomes true when ReadyToUse of all individual snapshots become true.
    /// If not specified, it means the readiness of a group snapshot is unknown.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readyToUse")]
    pub ready_to_use: Option<bool>,
}

/// Error is the last observed error during group snapshot creation, if any.
/// This field could be helpful to upper level controllers (i.e., application
/// controller) to decide whether they should continue on waiting for the group
/// snapshot to be created based on the type of error reported.
/// The snapshot controller will keep retrying when an error occurs during the
/// group snapshot creation. Upon success, this error field will be cleared.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeGroupSnapshotStatusError {
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

/// PVCVolumeSnapshotPair defines a pair of a PVC reference and a Volume Snapshot Reference
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeGroupSnapshotStatusPvcVolumeSnapshotRefList {
    /// PersistentVolumeClaimRef is a reference to the PVC this pair is referring to
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "persistentVolumeClaimRef")]
    pub persistent_volume_claim_ref: Option<VolumeGroupSnapshotStatusPvcVolumeSnapshotRefListPersistentVolumeClaimRef>,
    /// VolumeSnapshotRef is a reference to the VolumeSnapshot this pair is referring to
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeSnapshotRef")]
    pub volume_snapshot_ref: Option<VolumeGroupSnapshotStatusPvcVolumeSnapshotRefListVolumeSnapshotRef>,
}

/// PersistentVolumeClaimRef is a reference to the PVC this pair is referring to
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeGroupSnapshotStatusPvcVolumeSnapshotRefListPersistentVolumeClaimRef {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// VolumeSnapshotRef is a reference to the VolumeSnapshot this pair is referring to
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeGroupSnapshotStatusPvcVolumeSnapshotRefListVolumeSnapshotRef {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

