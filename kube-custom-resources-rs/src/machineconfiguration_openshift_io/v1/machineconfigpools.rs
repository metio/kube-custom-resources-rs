// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/openshift/api/machineconfiguration.openshift.io/v1/machineconfigpools.yaml --derive=Default --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;

/// MachineConfigPoolSpec is the spec for MachineConfigPool resource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "machineconfiguration.openshift.io", version = "v1", kind = "MachineConfigPool", plural = "machineconfigpools")]
#[kube(status = "MachineConfigPoolStatus")]
#[kube(schema = "disabled")]
pub struct MachineConfigPoolSpec {
    /// The targeted MachineConfig object for the machine config pool.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<MachineConfigPoolConfiguration>,
    /// machineConfigSelector specifies a label selector for MachineConfigs. Refer https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/ on how label and selectors work.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "machineConfigSelector")]
    pub machine_config_selector: Option<MachineConfigPoolMachineConfigSelector>,
    /// maxUnavailable defines either an integer number or percentage of nodes in the pool that can go Unavailable during an update. This includes nodes Unavailable for any reason, including user initiated cordons, failing nodes, etc. The default value is 1. 
    ///  A value larger than 1 will mean multiple nodes going unavailable during the update, which may affect your workload stress on the remaining nodes. You cannot set this value to 0 to stop updates (it will default back to 1); to stop updates, use the 'paused' property instead. Drain will respect Pod Disruption Budgets (PDBs) such as etcd quorum guards, even if maxUnavailable is greater than one.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxUnavailable")]
    pub max_unavailable: Option<IntOrString>,
    /// nodeSelector specifies a label selector for Machines
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<MachineConfigPoolNodeSelector>,
    /// paused specifies whether or not changes to this machine config pool should be stopped. This includes generating new desiredMachineConfig and update of machines.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
}

/// The targeted MachineConfig object for the machine config pool.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineConfigPoolConfiguration {
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
    /// source is the list of MachineConfig objects that were used to generate the single MachineConfig object specified in `content`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Vec<MachineConfigPoolConfigurationSource>>,
    /// UID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// ObjectReference contains enough information to let you inspect or modify the referred object. --- New uses of this type are discouraged because of difficulty describing its usage when embedded in APIs. 1. Ignored fields.  It includes many fields which are not generally honored.  For instance, ResourceVersion and FieldPath are both very rarely valid in actual usage. 2. Invalid usage help.  It is impossible to add specific help for individual usage.  In most embedded usages, there are particular restrictions like, "must refer only to types A and B" or "UID not honored" or "name must be restricted". Those cannot be well described when embedded. 3. Inconsistent validation.  Because the usages are different, the validation rules are different by usage, which makes it hard for users to predict what will happen. 4. The fields are both imprecise and overly precise.  Kind is not a precise mapping to a URL. This can produce ambiguity during interpretation and require a REST mapping.  In most cases, the dependency is on the group,resource tuple and the version of the actual struct is irrelevant. 5. We cannot easily change it.  Because this type is embedded in many locations, updates to this type will affect numerous schemas.  Don't make new APIs embed an underspecified API type they do not control. 
///  Instead of using this type, create a locally provided and used type that is well-focused on your reference. For example, ServiceReferences for admission registration: https://github.com/kubernetes/api/blob/release-1.17/admissionregistration/v1/types.go#L533 .
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineConfigPoolConfigurationSource {
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

/// machineConfigSelector specifies a label selector for MachineConfigs. Refer https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/ on how label and selectors work.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineConfigPoolMachineConfigSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<MachineConfigPoolMachineConfigSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineConfigPoolMachineConfigSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// nodeSelector specifies a label selector for Machines
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineConfigPoolNodeSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<MachineConfigPoolNodeSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineConfigPoolNodeSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// MachineConfigPoolStatus is the status for MachineConfigPool resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineConfigPoolStatus {
    /// certExpirys keeps track of important certificate expiration data
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certExpirys")]
    pub cert_expirys: Option<Vec<MachineConfigPoolStatusCertExpirys>>,
    /// conditions represents the latest available observations of current state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// configuration represents the current MachineConfig object for the machine config pool.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<MachineConfigPoolStatusConfiguration>,
    /// degradedMachineCount represents the total number of machines marked degraded (or unreconcilable). A node is marked degraded if applying a configuration failed..
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "degradedMachineCount")]
    pub degraded_machine_count: Option<i32>,
    /// machineCount represents the total number of machines in the machine config pool.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "machineCount")]
    pub machine_count: Option<i32>,
    /// observedGeneration represents the generation observed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// readyMachineCount represents the total number of ready machines targeted by the pool.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readyMachineCount")]
    pub ready_machine_count: Option<i32>,
    /// unavailableMachineCount represents the total number of unavailable (non-ready) machines targeted by the pool. A node is marked unavailable if it is in updating state or NodeReady condition is false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unavailableMachineCount")]
    pub unavailable_machine_count: Option<i32>,
    /// updatedMachineCount represents the total number of machines targeted by the pool that have the CurrentMachineConfig as their config.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updatedMachineCount")]
    pub updated_machine_count: Option<i32>,
}

/// ceryExpiry contains the bundle name and the expiry date
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineConfigPoolStatusCertExpirys {
    /// bundle is the name of the bundle in which the subject certificate resides
    pub bundle: String,
    /// expiry is the date after which the certificate will no longer be valid
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    /// subject is the subject of the certificate
    pub subject: String,
}

/// configuration represents the current MachineConfig object for the machine config pool.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineConfigPoolStatusConfiguration {
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
    /// source is the list of MachineConfig objects that were used to generate the single MachineConfig object specified in `content`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Vec<MachineConfigPoolStatusConfigurationSource>>,
    /// UID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// ObjectReference contains enough information to let you inspect or modify the referred object. --- New uses of this type are discouraged because of difficulty describing its usage when embedded in APIs. 1. Ignored fields.  It includes many fields which are not generally honored.  For instance, ResourceVersion and FieldPath are both very rarely valid in actual usage. 2. Invalid usage help.  It is impossible to add specific help for individual usage.  In most embedded usages, there are particular restrictions like, "must refer only to types A and B" or "UID not honored" or "name must be restricted". Those cannot be well described when embedded. 3. Inconsistent validation.  Because the usages are different, the validation rules are different by usage, which makes it hard for users to predict what will happen. 4. The fields are both imprecise and overly precise.  Kind is not a precise mapping to a URL. This can produce ambiguity during interpretation and require a REST mapping.  In most cases, the dependency is on the group,resource tuple and the version of the actual struct is irrelevant. 5. We cannot easily change it.  Because this type is embedded in many locations, updates to this type will affect numerous schemas.  Don't make new APIs embed an underspecified API type they do not control. 
///  Instead of using this type, create a locally provided and used type that is well-focused on your reference. For example, ServiceReferences for admission registration: https://github.com/kubernetes/api/blob/release-1.17/admissionregistration/v1/types.go#L533 .
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineConfigPoolStatusConfigurationSource {
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

