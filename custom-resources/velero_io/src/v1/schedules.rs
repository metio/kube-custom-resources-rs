// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/vmware-tanzu/velero/velero.io/v1/schedules.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// ScheduleSpec defines the specification for a Velero schedule
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "velero.io", version = "v1", kind = "Schedule", plural = "schedules")]
#[kube(namespaced)]
#[kube(status = "ScheduleStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ScheduleSpec {
    /// Paused specifies whether the schedule is paused or not
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    /// Schedule is a Cron expression defining when to run
    /// the Backup.
    pub schedule: String,
    /// SkipImmediately specifies whether to skip backup if schedule is due immediately from `schedule.status.lastBackup` timestamp when schedule is unpaused or if schedule is new.
    /// If true, backup will be skipped immediately when schedule is unpaused if it is due based on .Status.LastBackupTimestamp or schedule is new, and will run at next schedule time.
    /// If false, backup will not be skipped immediately when schedule is unpaused, but will run at next schedule time.
    /// If empty, will follow server configuration (default: false).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "skipImmediately")]
    pub skip_immediately: Option<bool>,
    /// Template is the definition of the Backup to be run
    /// on the provided schedule
    pub template: ScheduleTemplate,
    /// UseOwnerReferencesBackup specifies whether to use
    /// OwnerReferences on backups created by this Schedule.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useOwnerReferencesInBackup")]
    pub use_owner_references_in_backup: Option<bool>,
}

/// Template is the definition of the Backup to be run
/// on the provided schedule
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ScheduleTemplate {
    /// CSISnapshotTimeout specifies the time used to wait for CSI VolumeSnapshot status turns to
    /// ReadyToUse during creation, before returning error as timeout.
    /// The default value is 10 minute.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "csiSnapshotTimeout")]
    pub csi_snapshot_timeout: Option<String>,
    /// DataMover specifies the data mover to be used by the backup.
    /// If DataMover is "" or "velero", the built-in data mover will be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datamover: Option<String>,
    /// DefaultVolumesToFsBackup specifies whether pod volume file system backup should be used
    /// for all volumes by default.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultVolumesToFsBackup")]
    pub default_volumes_to_fs_backup: Option<bool>,
    /// DefaultVolumesToRestic specifies whether restic should be used to take a
    /// backup of all pod volumes by default.
    /// 
    /// Deprecated: this field is no longer used and will be removed entirely in future. Use DefaultVolumesToFsBackup instead.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultVolumesToRestic")]
    pub default_volumes_to_restic: Option<bool>,
    /// ExcludedClusterScopedResources is a slice of cluster-scoped
    /// resource type names to exclude from the backup.
    /// If set to "*", all cluster-scoped resource types are excluded.
    /// The default value is empty.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "excludedClusterScopedResources")]
    pub excluded_cluster_scoped_resources: Option<Vec<String>>,
    /// ExcludedNamespaceScopedResources is a slice of namespace-scoped
    /// resource type names to exclude from the backup.
    /// If set to "*", all namespace-scoped resource types are excluded.
    /// The default value is empty.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "excludedNamespaceScopedResources")]
    pub excluded_namespace_scoped_resources: Option<Vec<String>>,
    /// ExcludedNamespaces contains a list of namespaces that are not
    /// included in the backup.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "excludedNamespaces")]
    pub excluded_namespaces: Option<Vec<String>>,
    /// ExcludedResources is a slice of resource names that are not
    /// included in the backup.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "excludedResources")]
    pub excluded_resources: Option<Vec<String>>,
    /// Hooks represent custom behaviors that should be executed at different phases of the backup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hooks: Option<ScheduleTemplateHooks>,
    /// IncludeClusterResources specifies whether cluster-scoped resources
    /// should be included for consideration in the backup.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "includeClusterResources")]
    pub include_cluster_resources: Option<bool>,
    /// IncludedClusterScopedResources is a slice of cluster-scoped
    /// resource type names to include in the backup.
    /// If set to "*", all cluster-scoped resource types are included.
    /// The default value is empty, which means only related
    /// cluster-scoped resources are included.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "includedClusterScopedResources")]
    pub included_cluster_scoped_resources: Option<Vec<String>>,
    /// IncludedNamespaceScopedResources is a slice of namespace-scoped
    /// resource type names to include in the backup.
    /// The default value is "*".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "includedNamespaceScopedResources")]
    pub included_namespace_scoped_resources: Option<Vec<String>>,
    /// IncludedNamespaces is a slice of namespace names to include objects
    /// from. If empty, all namespaces are included.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "includedNamespaces")]
    pub included_namespaces: Option<Vec<String>>,
    /// IncludedResources is a slice of resource names to include
    /// in the backup. If empty, all resources are included.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "includedResources")]
    pub included_resources: Option<Vec<String>>,
    /// ItemOperationTimeout specifies the time used to wait for asynchronous BackupItemAction operations
    /// The default value is 4 hour.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "itemOperationTimeout")]
    pub item_operation_timeout: Option<String>,
    /// LabelSelector is a metav1.LabelSelector to filter with
    /// when adding individual objects to the backup. If empty
    /// or nil, all objects are included. Optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<ScheduleTemplateLabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ScheduleTemplateMetadata>,
    /// OrLabelSelectors is list of metav1.LabelSelector to filter with
    /// when adding individual objects to the backup. If multiple provided
    /// they will be joined by the OR operator. LabelSelector as well as
    /// OrLabelSelectors cannot co-exist in backup request, only one of them
    /// can be used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "orLabelSelectors")]
    pub or_label_selectors: Option<Vec<ScheduleTemplateOrLabelSelectors>>,
    /// OrderedResources specifies the backup order of resources of specific Kind.
    /// The map key is the resource name and value is a list of object names separated by commas.
    /// Each resource name has format "namespace/objectname".  For cluster resources, simply use "objectname".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "orderedResources")]
    pub ordered_resources: Option<BTreeMap<String, String>>,
    /// ResourcePolicy specifies the referenced resource policies that backup should follow
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourcePolicy")]
    pub resource_policy: Option<ScheduleTemplateResourcePolicy>,
    /// SnapshotMoveData specifies whether snapshot data should be moved
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotMoveData")]
    pub snapshot_move_data: Option<bool>,
    /// SnapshotVolumes specifies whether to take snapshots
    /// of any PV's referenced in the set of objects included
    /// in the Backup.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotVolumes")]
    pub snapshot_volumes: Option<bool>,
    /// StorageLocation is a string containing the name of a BackupStorageLocation where the backup should be stored.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageLocation")]
    pub storage_location: Option<String>,
    /// TTL is a time.Duration-parseable string describing how long
    /// the Backup should be retained for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
    /// UploaderConfig specifies the configuration for the uploader.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "uploaderConfig")]
    pub uploader_config: Option<ScheduleTemplateUploaderConfig>,
    /// VolumeGroupSnapshotLabelKey specifies the label key to group PVCs under a VGS.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeGroupSnapshotLabelKey")]
    pub volume_group_snapshot_label_key: Option<String>,
    /// VolumeSnapshotLocations is a list containing names of VolumeSnapshotLocations associated with this backup.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeSnapshotLocations")]
    pub volume_snapshot_locations: Option<Vec<String>>,
}

/// Hooks represent custom behaviors that should be executed at different phases of the backup.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ScheduleTemplateHooks {
    /// Resources are hooks that should be executed when backing up individual instances of a resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<ScheduleTemplateHooksResources>>,
}

/// BackupResourceHookSpec defines one or more BackupResourceHooks that should be executed based on
/// the rules defined for namespaces, resources, and label selector.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ScheduleTemplateHooksResources {
    /// ExcludedNamespaces specifies the namespaces to which this hook spec does not apply.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "excludedNamespaces")]
    pub excluded_namespaces: Option<Vec<String>>,
    /// ExcludedResources specifies the resources to which this hook spec does not apply.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "excludedResources")]
    pub excluded_resources: Option<Vec<String>>,
    /// IncludedNamespaces specifies the namespaces to which this hook spec applies. If empty, it applies
    /// to all namespaces.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "includedNamespaces")]
    pub included_namespaces: Option<Vec<String>>,
    /// IncludedResources specifies the resources to which this hook spec applies. If empty, it applies
    /// to all resources.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "includedResources")]
    pub included_resources: Option<Vec<String>>,
    /// LabelSelector, if specified, filters the resources to which this hook spec applies.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<ScheduleTemplateHooksResourcesLabelSelector>,
    /// Name is the name of this hook.
    pub name: String,
    /// PostHooks is a list of BackupResourceHooks to execute after storing the item in the backup.
    /// These are executed after all "additional items" from item actions are processed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post: Option<Vec<ScheduleTemplateHooksResourcesPost>>,
    /// PreHooks is a list of BackupResourceHooks to execute prior to storing the item in the backup.
    /// These are executed before any "additional items" from item actions are processed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre: Option<Vec<ScheduleTemplateHooksResourcesPre>>,
}

/// LabelSelector, if specified, filters the resources to which this hook spec applies.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ScheduleTemplateHooksResourcesLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ScheduleTemplateHooksResourcesLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ScheduleTemplateHooksResourcesLabelSelectorMatchExpressions {
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

/// BackupResourceHook defines a hook for a resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ScheduleTemplateHooksResourcesPost {
    /// Exec defines an exec hook.
    pub exec: ScheduleTemplateHooksResourcesPostExec,
}

/// Exec defines an exec hook.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ScheduleTemplateHooksResourcesPostExec {
    /// Command is the command and arguments to execute.
    pub command: Vec<String>,
    /// Container is the container in the pod where the command should be executed. If not specified,
    /// the pod's first container is used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    /// OnError specifies how Velero should behave if it encounters an error executing this hook.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onError")]
    pub on_error: Option<ScheduleTemplateHooksResourcesPostExecOnError>,
    /// Timeout defines the maximum amount of time Velero should wait for the hook to complete before
    /// considering the execution a failure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// Exec defines an exec hook.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ScheduleTemplateHooksResourcesPostExecOnError {
    Continue,
    Fail,
}

/// BackupResourceHook defines a hook for a resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ScheduleTemplateHooksResourcesPre {
    /// Exec defines an exec hook.
    pub exec: ScheduleTemplateHooksResourcesPreExec,
}

/// Exec defines an exec hook.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ScheduleTemplateHooksResourcesPreExec {
    /// Command is the command and arguments to execute.
    pub command: Vec<String>,
    /// Container is the container in the pod where the command should be executed. If not specified,
    /// the pod's first container is used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    /// OnError specifies how Velero should behave if it encounters an error executing this hook.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onError")]
    pub on_error: Option<ScheduleTemplateHooksResourcesPreExecOnError>,
    /// Timeout defines the maximum amount of time Velero should wait for the hook to complete before
    /// considering the execution a failure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// Exec defines an exec hook.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ScheduleTemplateHooksResourcesPreExecOnError {
    Continue,
    Fail,
}

/// LabelSelector is a metav1.LabelSelector to filter with
/// when adding individual objects to the backup. If empty
/// or nil, all objects are included. Optional.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ScheduleTemplateLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ScheduleTemplateLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ScheduleTemplateLabelSelectorMatchExpressions {
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

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ScheduleTemplateMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

/// A label selector is a label query over a set of resources. The result of matchLabels and
/// matchExpressions are ANDed. An empty label selector matches all objects. A null
/// label selector matches no objects.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ScheduleTemplateOrLabelSelectors {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ScheduleTemplateOrLabelSelectorsMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ScheduleTemplateOrLabelSelectorsMatchExpressions {
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

/// ResourcePolicy specifies the referenced resource policies that backup should follow
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ScheduleTemplateResourcePolicy {
    /// APIGroup is the group for the resource being referenced.
    /// If APIGroup is not specified, the specified Kind must be in the core API group.
    /// For any other third-party types, APIGroup is required.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiGroup")]
    pub api_group: Option<String>,
    /// Kind is the type of resource being referenced
    pub kind: String,
    /// Name is the name of resource being referenced
    pub name: String,
}

/// UploaderConfig specifies the configuration for the uploader.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ScheduleTemplateUploaderConfig {
    /// ParallelFilesUpload is the number of files parallel uploads to perform when using the uploader.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parallelFilesUpload")]
    pub parallel_files_upload: Option<i64>,
}

/// ScheduleStatus captures the current state of a Velero schedule
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ScheduleStatus {
    /// LastBackup is the last time a Backup was run for this
    /// Schedule schedule
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastBackup")]
    pub last_backup: Option<String>,
    /// LastSkipped is the last time a Schedule was skipped
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastSkipped")]
    pub last_skipped: Option<String>,
    /// Phase is the current phase of the Schedule
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<ScheduleStatusPhase>,
    /// ValidationErrors is a slice of all validation errors (if
    /// applicable)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "validationErrors")]
    pub validation_errors: Option<Vec<String>>,
}

/// ScheduleStatus captures the current state of a Velero schedule
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ScheduleStatusPhase {
    New,
    Enabled,
    FailedValidation,
}

