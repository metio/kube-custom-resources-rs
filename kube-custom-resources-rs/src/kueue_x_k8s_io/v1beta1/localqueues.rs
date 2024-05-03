// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/kueue/kueue.x-k8s.io/v1beta1/localqueues.yaml --derive=PartialEq
// kopium version: 0.19.0

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// LocalQueueSpec defines the desired state of LocalQueue
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "kueue.x-k8s.io", version = "v1beta1", kind = "LocalQueue", plural = "localqueues")]
#[kube(namespaced)]
#[kube(status = "LocalQueueStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct LocalQueueSpec {
    /// clusterQueue is a reference to a clusterQueue that backs this localQueue.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterQueue")]
    pub cluster_queue: Option<String>,
}

/// LocalQueueStatus defines the observed state of LocalQueue
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LocalQueueStatus {
    /// admittedWorkloads is the number of workloads in this LocalQueue
    /// admitted to a ClusterQueue and that haven't finished yet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "admittedWorkloads")]
    pub admitted_workloads: Option<i32>,
    /// Conditions hold the latest available observations of the LocalQueue
    /// current state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// flavorsUsage are the used quotas, by flavor currently in use by the
    /// workloads assigned to this LocalQueue.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "flavorUsage")]
    pub flavor_usage: Option<Vec<LocalQueueStatusFlavorUsage>>,
    /// flavorsReservation are the reserved quotas, by flavor currently in use by the
    /// workloads assigned to this LocalQueue.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "flavorsReservation")]
    pub flavors_reservation: Option<Vec<LocalQueueStatusFlavorsReservation>>,
    /// PendingWorkloads is the number of Workloads in the LocalQueue not yet admitted to a ClusterQueue
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pendingWorkloads")]
    pub pending_workloads: Option<i32>,
    /// reservingWorkloads is the number of workloads in this LocalQueue
    /// reserving quota in a ClusterQueue and that haven't finished yet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reservingWorkloads")]
    pub reserving_workloads: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LocalQueueStatusFlavorUsage {
    /// name of the flavor.
    pub name: String,
    /// resources lists the quota usage for the resources in this flavor.
    pub resources: Vec<LocalQueueStatusFlavorUsageResources>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LocalQueueStatusFlavorUsageResources {
    /// name of the resource.
    pub name: String,
    /// total is the total quantity of used quota.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<IntOrString>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LocalQueueStatusFlavorsReservation {
    /// name of the flavor.
    pub name: String,
    /// resources lists the quota usage for the resources in this flavor.
    pub resources: Vec<LocalQueueStatusFlavorsReservationResources>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LocalQueueStatusFlavorsReservationResources {
    /// name of the resource.
    pub name: String,
    /// total is the total quantity of used quota.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<IntOrString>,
}

