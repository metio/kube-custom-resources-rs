// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/koordinator-sh/koordinator/scheduling.sigs.k8s.io/v1alpha1/podgroups.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// Specification of the desired behavior of the pod group.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "scheduling.sigs.k8s.io", version = "v1alpha1", kind = "PodGroup", plural = "podgroups")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct PodGroupSpec {
    /// MinMember defines the minimal number of members/tasks to run the pod group; if there's not enough resources to start all tasks, the scheduler will not start anyone.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minMember")]
    pub min_member: Option<i32>,
    /// MinResources defines the minimal resource of members/tasks to run the pod group; if there's not enough resources to start all tasks, the scheduler will not start anyone.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minResources")]
    pub min_resources: Option<BTreeMap<String, IntOrString>>,
    /// ScheduleTimeoutSeconds defines the maximal time of members/tasks to wait before run the pod group;
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scheduleTimeoutSeconds")]
    pub schedule_timeout_seconds: Option<i32>,
}

/// Status represents the current information about a pod group. This data may not be up to date.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PodGroupStatus {
    /// The number of pods which reached phase Failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<i32>,
    /// OccupiedBy marks the workload (e.g., deployment, statefulset) UID that occupy the podgroup. It is empty if not initialized.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "occupiedBy")]
    pub occupied_by: Option<String>,
    /// Current phase of PodGroup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// The number of actively running pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub running: Option<i32>,
    /// ScheduleStartTime of the group
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scheduleStartTime")]
    pub schedule_start_time: Option<String>,
    /// The number of actively running pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduled: Option<i32>,
    /// The number of pods which reached phase Succeeded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<i32>,
}

