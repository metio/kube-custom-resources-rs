// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-csi/external-snapshotter/groupsnapshot.storage.k8s.io/v1alpha1/volumegroupsnapshots.yaml --derive=Default --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// Spec defines the desired characteristics of a group snapshot requested by a user. Required.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "groupsnapshot.storage.k8s.io", version = "v1alpha1", kind = "VolumeGroupSnapshot", plural = "volumegroupsnapshots")]
#[kube(namespaced)]
#[kube(status = "VolumeGroupSnapshotStatus")]
#[kube(schema = "disabled")]
pub struct VolumeGroupSnapshotSpec {
    /// Source specifies where a group snapshot will be created from. This field is immutable after creation. Required.
    pub source: VolumeGroupSnapshotSource,
    /// VolumeGroupSnapshotClassName is the name of the VolumeGroupSnapshotClass requested by the VolumeGroupSnapshot. VolumeGroupSnapshotClassName may be left nil to indicate that the default class will be used. Empty string is not allowed for this field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeGroupSnapshotClassName")]
    pub volume_group_snapshot_class_name: Option<String>,
}

/// Source specifies where a group snapshot will be created from. This field is immutable after creation. Required.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeGroupSnapshotSource {
    /// Selector is a label query over persistent volume claims that are to be grouped together for snapshotting. This labelSelector will be used to match the label added to a PVC. If the label is added or removed to a volume after a group snapshot is created, the existing group snapshots won't be modified. Once a VolumeGroupSnapshotContent is created and the sidecar starts to process it, the volume list will not change with retries.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<VolumeGroupSnapshotSourceSelector>,
    /// VolumeGroupSnapshotContentName specifies the name of a pre-existing VolumeGroupSnapshotContent object representing an existing volume group snapshot. This field should be set if the volume group snapshot already exists and only needs a representation in Kubernetes. This field is immutable.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeGroupSnapshotContentName")]
    pub volume_group_snapshot_content_name: Option<String>,
}

/// Selector is a label query over persistent volume claims that are to be grouped together for snapshotting. This labelSelector will be used to match the label added to a PVC. If the label is added or removed to a volume after a group snapshot is created, the existing group snapshots won't be modified. Once a VolumeGroupSnapshotContent is created and the sidecar starts to process it, the volume list will not change with retries.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeGroupSnapshotSourceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<VolumeGroupSnapshotSourceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeGroupSnapshotSourceSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Status represents the current information of a group snapshot. Consumers must verify binding between VolumeGroupSnapshot and VolumeGroupSnapshotContent objects is successful (by validating that both VolumeGroupSnapshot and VolumeGroupSnapshotContent point to each other) before using this object.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeGroupSnapshotStatus {
    /// BoundVolumeGroupSnapshotContentName is the name of the VolumeGroupSnapshotContent object to which this VolumeGroupSnapshot object intends to bind to. If not specified, it indicates that the VolumeGroupSnapshot object has not been successfully bound to a VolumeGroupSnapshotContent object yet. NOTE: To avoid possible security issues, consumers must verify binding between VolumeGroupSnapshot and VolumeGroupSnapshotContent objects is successful (by validating that both VolumeGroupSnapshot and VolumeGroupSnapshotContent point at each other) before using this object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "boundVolumeGroupSnapshotContentName")]
    pub bound_volume_group_snapshot_content_name: Option<String>,
    /// CreationTime is the timestamp when the point-in-time group snapshot is taken by the underlying storage system. If not specified, it may indicate that the creation time of the group snapshot is unknown. The format of this field is a Unix nanoseconds time encoded as an int64. On Unix, the command date +%s%N returns the current time in nanoseconds since 1970-01-01 00:00:00 UTC.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "creationTime")]
    pub creation_time: Option<String>,
    /// Error is the last observed error during group snapshot creation, if any. This field could be helpful to upper level controllers (i.e., application controller) to decide whether they should continue on waiting for the group snapshot to be created based on the type of error reported. The snapshot controller will keep retrying when an error occurs during the group snapshot creation. Upon success, this error field will be cleared.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<VolumeGroupSnapshotStatusError>,
    /// ReadyToUse indicates if all the individual snapshots in the group are ready to be used to restore a group of volumes. ReadyToUse becomes true when ReadyToUse of all individual snapshots become true. If not specified, it means the readiness of a group snapshot is unknown.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readyToUse")]
    pub ready_to_use: Option<bool>,
    /// VolumeSnapshotRefList is the list of volume snapshot references for this group snapshot. The maximum number of allowed snapshots in the group is 100.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeSnapshotRefList")]
    pub volume_snapshot_ref_list: Option<Vec<VolumeGroupSnapshotStatusVolumeSnapshotRefList>>,
}

/// Error is the last observed error during group snapshot creation, if any. This field could be helpful to upper level controllers (i.e., application controller) to decide whether they should continue on waiting for the group snapshot to be created based on the type of error reported. The snapshot controller will keep retrying when an error occurs during the group snapshot creation. Upon success, this error field will be cleared.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeGroupSnapshotStatusError {
    /// message is a string detailing the encountered error during snapshot creation if specified. NOTE: message may be logged, and it should not contain sensitive information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// time is the timestamp when the error was encountered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

/// ObjectReference contains enough information to let you inspect or modify the referred object. --- New uses of this type are discouraged because of difficulty describing its usage when embedded in APIs. 1. Ignored fields.  It includes many fields which are not generally honored.  For instance, ResourceVersion and FieldPath are both very rarely valid in actual usage. 2. Invalid usage help.  It is impossible to add specific help for individual usage.  In most embedded usages, there are particular restrictions like, "must refer only to types A and B" or "UID not honored" or "name must be restricted". Those cannot be well described when embedded. 3. Inconsistent validation.  Because the usages are different, the validation rules are different by usage, which makes it hard for users to predict what will happen. 4. The fields are both imprecise and overly precise.  Kind is not a precise mapping to a URL. This can produce ambiguity during interpretation and require a REST mapping.  In most cases, the dependency is on the group,resource tuple and the version of the actual struct is irrelevant. 5. We cannot easily change it.  Because this type is embedded in many locations, updates to this type will affect numerous schemas.  Don't make new APIs embed an underspecified API type they do not control. 
///  Instead of using this type, create a locally provided and used type that is well-focused on your reference. For example, ServiceReferences for admission registration: https://github.com/kubernetes/api/blob/release-1.17/admissionregistration/v1/types.go#L533 .
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeGroupSnapshotStatusVolumeSnapshotRefList {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2]. For example, if the object reference is to a container within a pod, this would take on a value like: "spec.containers{name}" (where "name" refers to the name of the container that triggered the event) or if no container name is specified "spec.containers[2]" (container with index 2 in this pod). This syntax is chosen only to have some well-defined way of referencing a part of an object. TODO: this design is not final and this field is subject to change in the future.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

