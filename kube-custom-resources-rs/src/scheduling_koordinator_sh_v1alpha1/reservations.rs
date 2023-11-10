// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/koordinator-sh/koordinator/scheduling.koordinator.sh/v1alpha1/reservations.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::collections::HashMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "scheduling.koordinator.sh", version = "v1alpha1", kind = "Reservation", plural = "reservations")]
#[kube(status = "ReservationStatus")]
#[kube(schema = "disabled")]
pub struct ReservationSpec {
    /// When `AllocateOnce` is set, the reserved resources are only available for the first owner who allocates successfully and are not allocatable to other owners anymore. Defaults to true.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allocateOnce")]
    pub allocate_once: Option<bool>,
    /// AllocatePolicy represents the allocation policy of reserved resources that Reservation expects.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allocatePolicy")]
    pub allocate_policy: Option<ReservationAllocatePolicy>,
    /// Expired timestamp when the reservation is expected to expire. If both `expires` and `ttl` are set, `expires` is checked first. `expires` and `ttl` are mutually exclusive. Defaults to being set dynamically at runtime based on the `ttl`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    /// Specify the owners who can allocate the reserved resources. Multiple owner selectors and ORed.
    pub owners: Vec<ReservationOwners>,
    /// By default, the resources requirements of reservation (specified in `template.spec`) is filtered by whether the node has sufficient free resources (i.e. Reservation Request <  Node Free). When `preAllocation` is set, the scheduler will skip this validation and allow overcommitment. The scheduled reservation would be waiting to be available until free resources are sufficient.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preAllocation")]
    pub pre_allocation: Option<bool>,
    /// Template defines the scheduling requirements (resources, affinities, images, ...) processed by the scheduler just like a normal pod. If the `template.spec.nodeName` is specified, the scheduler will not choose another node but reserve resources on the specified node.
    pub template: HashMap<String, serde_json::Value>,
    /// Time-to-Live period for the reservation. `expires` and `ttl` are mutually exclusive. Defaults to 24h. Set 0 to disable expiration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
    /// Unschedulable controls reservation schedulability of new pods. By default, reservation is schedulable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unschedulable: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ReservationAllocatePolicy {
    Aligned,
    Restricted,
}

/// ReservationOwner indicates the owner specification which can allocate reserved resources.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReservationOwners {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controller: Option<ReservationOwnersController>,
    /// A label selector is a label query over a set of resources. The result of matchLabels and matchExpressions are ANDed. An empty label selector matches all objects. A null label selector matches no objects.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<ReservationOwnersLabelSelector>,
    /// Multiple field selectors are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<ReservationOwnersObject>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReservationOwnersController {
    /// API version of the referent.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// If true, AND if the owner has the "foregroundDeletion" finalizer, then the owner cannot be deleted from the key-value store until this reference is removed. See https://kubernetes.io/docs/concepts/architecture/garbage-collection/#foreground-deletion for how the garbage collector interacts with this field and enforces the foreground deletion. Defaults to false. To set this field, a user needs "delete" permission of the owner, otherwise 422 (Unprocessable Entity) will be returned.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "blockOwnerDeletion")]
    pub block_owner_deletion: Option<bool>,
    /// If true, this reference points to the managing controller.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controller: Option<bool>,
    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub kind: String,
    /// Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// UID of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#uids
    pub uid: String,
}

/// A label selector is a label query over a set of resources. The result of matchLabels and matchExpressions are ANDed. An empty label selector matches all objects. A null label selector matches no objects.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReservationOwnersLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ReservationOwnersLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReservationOwnersLabelSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Multiple field selectors are ANDed.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReservationOwnersObject {
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

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReservationStatus {
    /// Resource reserved and allocatable for owners.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocatable: Option<BTreeMap<String, IntOrString>>,
    /// Resource allocated by current owners.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocated: Option<BTreeMap<String, IntOrString>>,
    /// The `conditions` indicate the messages of reason why the reservation is still pending.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ReservationStatusConditions>>,
    /// Current resource owners which allocated the reservation resources.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentOwners")]
    pub current_owners: Option<Vec<ReservationStatusCurrentOwners>>,
    /// Name of node the reservation is scheduled on.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeName")]
    pub node_name: Option<String>,
    /// The `phase` indicates whether is reservation is waiting for process, available to allocate or failed/expired to get cleanup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReservationStatusConditions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastProbeTime")]
    pub last_probe_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// ObjectReference contains enough information to let you inspect or modify the referred object. --- New uses of this type are discouraged because of difficulty describing its usage when embedded in APIs. 1. Ignored fields.  It includes many fields which are not generally honored.  For instance, ResourceVersion and FieldPath are both very rarely valid in actual usage. 2. Invalid usage help.  It is impossible to add specific help for individual usage.  In most embedded usages, there are particular restrictions like, "must refer only to types A and B" or "UID not honored" or "name must be restricted". Those cannot be well described when embedded. 3. Inconsistent validation.  Because the usages are different, the validation rules are different by usage, which makes it hard for users to predict what will happen. 4. The fields are both imprecise and overly precise.  Kind is not a precise mapping to a URL. This can produce ambiguity during interpretation and require a REST mapping.  In most cases, the dependency is on the group,resource tuple and the version of the actual struct is irrelevant. 5. We cannot easily change it.  Because this type is embedded in many locations, updates to this type will affect numerous schemas.  Don't make new APIs embed an underspecified API type they do not control. 
///  Instead of using this type, create a locally provided and used type that is well-focused on your reference. For example, ServiceReferences for admission registration: https://github.com/kubernetes/api/blob/release-1.17/admissionregistration/v1/types.go#L533 .
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReservationStatusCurrentOwners {
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

