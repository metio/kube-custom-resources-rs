// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/volcano-sh/volcano/scheduling.volcano.sh/v1beta1/podgroups.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// Specification of the desired behavior of the pod group. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "scheduling.volcano.sh", version = "v1beta1", kind = "PodGroup", plural = "podgroups")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct PodGroupSpec {
    /// MinMember defines the minimal number of members/tasks to run the pod group; if there's not enough resources to start all tasks, the scheduler will not start anyone.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minMember")]
    pub min_member: Option<i32>,
    /// MinResources defines the minimal resource of members/tasks to run the pod group; if there's not enough resources to start all tasks, the scheduler will not start anyone.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minResources")]
    pub min_resources: Option<BTreeMap<String, IntOrString>>,
    /// MinTaskMember defines the minimal number of pods to run each task in the pod group; if there's not enough resources to start each task, the scheduler will not start anyone.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minTaskMember")]
    pub min_task_member: Option<BTreeMap<String, i32>>,
    /// If specified, indicates the PodGroup's priority. "system-node-critical" and "system-cluster-critical" are two special keywords which indicate the highest priorities with the former being the highest priority. Any other name must be defined by creating a PriorityClass object with that name. If not specified, the PodGroup priority will be default or zero if there is no default.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "priorityClassName")]
    pub priority_class_name: Option<String>,
    /// Queue defines the queue to allocate resource for PodGroup; if queue does not exist, the PodGroup will not be scheduled. Defaults to `default` Queue with the lowest weight.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
}

/// Status represents the current information about a pod group. This data may not be up to date.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PodGroupStatus {
    /// The conditions of PodGroup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<PodGroupStatusConditions>>,
    /// The number of pods which reached phase Failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<i32>,
    /// Current phase of PodGroup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// The number of actively running pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub running: Option<i32>,
    /// The number of pods which reached phase Succeeded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<i32>,
}

/// PodGroupCondition contains details for the current state of this pod group.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PodGroupStatusConditions {
    /// Last time the phase transitioned from another to current phase.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// Human-readable message indicating details about last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Unique, one-word, CamelCase reason for the phase's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Status is the status of the condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The ID of condition transition.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "transitionID")]
    pub transition_id: Option<String>,
    /// Type is the type of the condition
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

