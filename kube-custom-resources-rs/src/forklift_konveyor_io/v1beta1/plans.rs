// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubev2v/forklift/forklift.konveyor.io/v1beta1/plans.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// PlanSpec defines the desired state of Plan.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "forklift.konveyor.io", version = "v1beta1", kind = "Plan", plural = "plans")]
#[kube(namespaced)]
#[kube(status = "PlanStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct PlanSpec {
    /// Whether this plan should be archived.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// Description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Resource mapping.
    pub map: PlanMap,
    /// Preserve the CPU model and flags the VM runs with in its oVirt cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preserveClusterCpuModel")]
    pub preserve_cluster_cpu_model: Option<bool>,
    /// Preserve static IPs of VMs in vSphere (Windows only)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preserveStaticIPs")]
    pub preserve_static_i_ps: Option<bool>,
    /// Providers.
    pub provider: PlanProvider,
    /// Target namespace.
    #[serde(rename = "targetNamespace")]
    pub target_namespace: String,
    /// The network attachment definition that should be used for disk transfer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "transferNetwork")]
    pub transfer_network: Option<PlanTransferNetwork>,
    /// List of VMs.
    pub vms: Vec<PlanVms>,
    /// Whether this is a warm migration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warm: Option<bool>,
}

/// Resource mapping.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanMap {
    /// Network.
    pub network: PlanMapNetwork,
    /// Storage.
    pub storage: PlanMapStorage,
}

/// Network.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanMapNetwork {
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

/// Storage.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanMapStorage {
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

/// Providers.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanProvider {
    /// Destination.
    pub destination: PlanProviderDestination,
    /// Source.
    pub source: PlanProviderSource,
}

/// Destination.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanProviderDestination {
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

/// Source.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanProviderSource {
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

/// The network attachment definition that should be used for disk transfer.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanTransferNetwork {
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

/// A VM listed on the plan.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanVms {
    /// Enable hooks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hooks: Option<Vec<PlanVmsHooks>>,
    /// The object ID.
    /// vsphere:
    ///   The managed object ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Disk decryption LUKS keys
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub luks: Option<PlanVmsLuks>,
    /// An object Name.
    /// vsphere:
    ///   A qualified name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The VM Namespace
    /// Only relevant for an openshift source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Type used to qualify the name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// Plan hook.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanVmsHooks {
    /// Hook reference.
    pub hook: PlanVmsHooksHook,
    /// Pipeline step.
    pub step: String,
}

/// Hook reference.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanVmsHooksHook {
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

/// Disk decryption LUKS keys
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanVmsLuks {
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

/// PlanStatus defines the observed state of Plan.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanStatus {
    /// List of conditions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Migration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migration: Option<PlanStatusMigration>,
    /// The most recent generation observed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
}

/// Migration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanStatusMigration {
    /// Completed timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    /// History
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<PlanStatusMigrationHistory>>,
    /// Started timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started: Option<String>,
    /// VM status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vms: Option<Vec<PlanStatusMigrationVms>>,
}

/// Snapshot
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanStatusMigrationHistory {
    /// List of conditions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Map.
    pub map: PlanStatusMigrationHistoryMap,
    /// Migration
    pub migration: PlanStatusMigrationHistoryMigration,
    /// Plan
    pub plan: PlanStatusMigrationHistoryPlan,
    /// Provider
    pub provider: PlanStatusMigrationHistoryProvider,
}

/// Map.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanStatusMigrationHistoryMap {
    /// Snapshot object reference.
    pub network: PlanStatusMigrationHistoryMapNetwork,
    /// Snapshot object reference.
    pub storage: PlanStatusMigrationHistoryMapStorage,
}

/// Snapshot object reference.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanStatusMigrationHistoryMapNetwork {
    pub generation: i64,
    pub name: String,
    pub namespace: String,
    /// UID is a type that holds unique ID values, including UUIDs.  Because we
    /// don't ONLY use UUIDs, this is an alias to string.  Being a type captures
    /// intent and helps make sure that UIDs and names do not get conflated.
    pub uid: String,
}

/// Snapshot object reference.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanStatusMigrationHistoryMapStorage {
    pub generation: i64,
    pub name: String,
    pub namespace: String,
    /// UID is a type that holds unique ID values, including UUIDs.  Because we
    /// don't ONLY use UUIDs, this is an alias to string.  Being a type captures
    /// intent and helps make sure that UIDs and names do not get conflated.
    pub uid: String,
}

/// Migration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanStatusMigrationHistoryMigration {
    pub generation: i64,
    pub name: String,
    pub namespace: String,
    /// UID is a type that holds unique ID values, including UUIDs.  Because we
    /// don't ONLY use UUIDs, this is an alias to string.  Being a type captures
    /// intent and helps make sure that UIDs and names do not get conflated.
    pub uid: String,
}

/// Plan
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanStatusMigrationHistoryPlan {
    pub generation: i64,
    pub name: String,
    pub namespace: String,
    /// UID is a type that holds unique ID values, including UUIDs.  Because we
    /// don't ONLY use UUIDs, this is an alias to string.  Being a type captures
    /// intent and helps make sure that UIDs and names do not get conflated.
    pub uid: String,
}

/// Provider
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanStatusMigrationHistoryProvider {
    /// Snapshot object reference.
    pub destination: PlanStatusMigrationHistoryProviderDestination,
    /// Snapshot object reference.
    pub source: PlanStatusMigrationHistoryProviderSource,
}

/// Snapshot object reference.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanStatusMigrationHistoryProviderDestination {
    pub generation: i64,
    pub name: String,
    pub namespace: String,
    /// UID is a type that holds unique ID values, including UUIDs.  Because we
    /// don't ONLY use UUIDs, this is an alias to string.  Being a type captures
    /// intent and helps make sure that UIDs and names do not get conflated.
    pub uid: String,
}

/// Snapshot object reference.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanStatusMigrationHistoryProviderSource {
    pub generation: i64,
    pub name: String,
    pub namespace: String,
    /// UID is a type that holds unique ID values, including UUIDs.  Because we
    /// don't ONLY use UUIDs, this is an alias to string.  Being a type captures
    /// intent and helps make sure that UIDs and names do not get conflated.
    pub uid: String,
}

/// VM Status
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanStatusMigrationVms {
    /// Completed timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    /// List of conditions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Errors
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PlanStatusMigrationVmsError>,
    /// The firmware type detected from the OVF file produced by virt-v2v.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub firmware: Option<String>,
    /// Enable hooks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hooks: Option<Vec<PlanStatusMigrationVmsHooks>>,
    /// The object ID.
    /// vsphere:
    ///   The managed object ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Disk decryption LUKS keys
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub luks: Option<PlanStatusMigrationVmsLuks>,
    /// An object Name.
    /// vsphere:
    ///   A qualified name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The VM Namespace
    /// Only relevant for an openshift source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Phase
    pub phase: String,
    /// Migration pipeline.
    pub pipeline: Vec<PlanStatusMigrationVmsPipeline>,
    /// Source VM power state before migration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "restorePowerState")]
    pub restore_power_state: Option<String>,
    /// Started timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started: Option<String>,
    /// Type used to qualify the name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// Warm migration status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warm: Option<PlanStatusMigrationVmsWarm>,
}

/// Errors
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanStatusMigrationVmsError {
    pub phase: String,
    pub reasons: Vec<String>,
}

/// Plan hook.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanStatusMigrationVmsHooks {
    /// Hook reference.
    pub hook: PlanStatusMigrationVmsHooksHook,
    /// Pipeline step.
    pub step: String,
}

/// Hook reference.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanStatusMigrationVmsHooksHook {
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

/// Disk decryption LUKS keys
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanStatusMigrationVmsLuks {
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

/// Pipeline step.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanStatusMigrationVmsPipeline {
    /// Annotations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Completed timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    /// Name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PlanStatusMigrationVmsPipelineError>,
    /// Name.
    pub name: String,
    /// Phase
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// Progress.
    pub progress: PlanStatusMigrationVmsPipelineProgress,
    /// Reason
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Started timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started: Option<String>,
    /// Nested tasks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<PlanStatusMigrationVmsPipelineTasks>>,
}

/// Error.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanStatusMigrationVmsPipelineError {
    pub phase: String,
    pub reasons: Vec<String>,
}

/// Progress.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanStatusMigrationVmsPipelineProgress {
    /// Completed units.
    pub completed: i64,
    /// Total units.
    pub total: i64,
}

/// Migration task.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanStatusMigrationVmsPipelineTasks {
    /// Annotations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Completed timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    /// Name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PlanStatusMigrationVmsPipelineTasksError>,
    /// Name.
    pub name: String,
    /// Phase
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// Progress.
    pub progress: PlanStatusMigrationVmsPipelineTasksProgress,
    /// Reason
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Started timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started: Option<String>,
}

/// Error.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanStatusMigrationVmsPipelineTasksError {
    pub phase: String,
    pub reasons: Vec<String>,
}

/// Progress.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanStatusMigrationVmsPipelineTasksProgress {
    /// Completed units.
    pub completed: i64,
    /// Total units.
    pub total: i64,
}

/// Warm migration status
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanStatusMigrationVmsWarm {
    #[serde(rename = "consecutiveFailures")]
    pub consecutive_failures: i64,
    pub failures: i64,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nextPrecopyAt")]
    pub next_precopy_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precopies: Option<Vec<PlanStatusMigrationVmsWarmPrecopies>>,
    pub successes: i64,
}

/// Precopy durations
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlanStatusMigrationVmsWarmPrecopies {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
}

