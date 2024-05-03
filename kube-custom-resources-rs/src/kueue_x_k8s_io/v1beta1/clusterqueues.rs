// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/kueue/kueue.x-k8s.io/v1beta1/clusterqueues.yaml --derive=PartialEq
// kopium version: 0.19.0

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// ClusterQueueSpec defines the desired state of ClusterQueue
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "kueue.x-k8s.io", version = "v1beta1", kind = "ClusterQueue", plural = "clusterqueues")]
#[kube(status = "ClusterQueueStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct ClusterQueueSpec {
    /// admissionChecks lists the AdmissionChecks required by this ClusterQueue.
    /// Cannot be used along with AdmissionCheckStrategy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "admissionChecks")]
    pub admission_checks: Option<Vec<String>>,
    /// admissionCheckStrategy defines a list of strategies to determine which ResourceFlavors require AdmissionChecks.
    /// This property cannot be used in conjunction with the 'admissionChecks' property.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "admissionChecksStrategy")]
    pub admission_checks_strategy: Option<ClusterQueueAdmissionChecksStrategy>,
    /// cohort that this ClusterQueue belongs to. CQs that belong to the
    /// same cohort can borrow unused resources from each other.
    /// 
    /// 
    /// A CQ can be a member of a single borrowing cohort. A workload submitted
    /// to a queue referencing this CQ can borrow quota from any CQ in the cohort.
    /// Only quota for the [resource, flavor] pairs listed in the CQ can be
    /// borrowed.
    /// If empty, this ClusterQueue cannot borrow from any other ClusterQueue and
    /// vice versa.
    /// 
    /// 
    /// A cohort is a name that links CQs together, but it doesn't reference any
    /// object.
    /// 
    /// 
    /// Validation of a cohort name is equivalent to that of object names:
    /// subdomain in DNS (RFC 1123).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cohort: Option<String>,
    /// flavorFungibility defines whether a workload should try the next flavor
    /// before borrowing or preempting in the flavor being evaluated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "flavorFungibility")]
    pub flavor_fungibility: Option<ClusterQueueFlavorFungibility>,
    /// namespaceSelector defines which namespaces are allowed to submit workloads to
    /// this clusterQueue. Beyond this basic support for policy, a policy agent like
    /// Gatekeeper should be used to enforce more advanced policies.
    /// Defaults to null which is a nothing selector (no namespaces eligible).
    /// If set to an empty selector `{}`, then all namespaces are eligible.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceSelector")]
    pub namespace_selector: Option<ClusterQueueNamespaceSelector>,
    /// preemption describes policies to preempt Workloads from this ClusterQueue
    /// or the ClusterQueue's cohort.
    /// 
    /// 
    /// Preemption can happen in two scenarios:
    /// 
    /// 
    /// - When a Workload fits within the nominal quota of the ClusterQueue, but
    ///   the quota is currently borrowed by other ClusterQueues in the cohort.
    ///   Preempting Workloads in other ClusterQueues allows this ClusterQueue to
    ///   reclaim its nominal quota.
    /// - When a Workload doesn't fit within the nominal quota of the ClusterQueue
    ///   and there are admitted Workloads in the ClusterQueue with lower priority.
    /// 
    /// 
    /// The preemption algorithm tries to find a minimal set of Workloads to
    /// preempt to accomomdate the pending Workload, preempting Workloads with
    /// lower priority first.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preemption: Option<ClusterQueuePreemption>,
    /// QueueingStrategy indicates the queueing strategy of the workloads
    /// across the queues in this ClusterQueue.
    /// Current Supported Strategies:
    /// 
    /// 
    /// - StrictFIFO: workloads are ordered strictly by creation time.
    /// Older workloads that can't be admitted will block admitting newer
    /// workloads even if they fit available quota.
    /// - BestEffortFIFO: workloads are ordered by creation time,
    /// however older workloads that can't be admitted will not block
    /// admitting newer workloads that fit existing quota.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "queueingStrategy")]
    pub queueing_strategy: Option<ClusterQueueQueueingStrategy>,
    /// resourceGroups describes groups of resources.
    /// Each resource group defines the list of resources and a list of flavors
    /// that provide quotas for these resources.
    /// Each resource and each flavor can only form part of one resource group.
    /// resourceGroups can be up to 16.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceGroups")]
    pub resource_groups: Option<Vec<ClusterQueueResourceGroups>>,
    /// stopPolicy - if set to a value different from None, the ClusterQueue is considered Inactive, no new reservation being
    /// made.
    /// 
    /// 
    /// Depending on its value, its associated workloads will:
    /// 
    /// 
    /// - None - Workloads are admitted
    /// - HoldAndDrain - Admitted workloads are evicted and Reserving workloads will cancel the reservation.
    /// - Hold - Admitted workloads will run to completion and Reserving workloads will cancel the reservation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stopPolicy")]
    pub stop_policy: Option<ClusterQueueStopPolicy>,
}

/// admissionCheckStrategy defines a list of strategies to determine which ResourceFlavors require AdmissionChecks.
/// This property cannot be used in conjunction with the 'admissionChecks' property.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterQueueAdmissionChecksStrategy {
    /// admissionChecks is a list of strategies for AdmissionChecks
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "admissionChecks")]
    pub admission_checks: Option<Vec<ClusterQueueAdmissionChecksStrategyAdmissionChecks>>,
}

/// AdmissionCheckStrategyRule defines rules for a single AdmissionCheck
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterQueueAdmissionChecksStrategyAdmissionChecks {
    /// name is an AdmissionCheck's name.
    pub name: String,
    /// onFlavors is a list of ResourceFlavors' names that this AdmissionCheck should run for.
    /// If empty, the AdmissionCheck will run for all workloads submitted to the ClusterQueue.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onFlavors")]
    pub on_flavors: Option<Vec<String>>,
}

/// flavorFungibility defines whether a workload should try the next flavor
/// before borrowing or preempting in the flavor being evaluated.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterQueueFlavorFungibility {
    /// whenCanBorrow determines whether a workload should try the next flavor
    /// before borrowing in current flavor. The possible values are:
    /// 
    /// 
    /// - `Borrow` (default): allocate in current flavor if borrowing
    ///   is possible.
    /// - `TryNextFlavor`: try next flavor even if the current
    ///   flavor has enough resources to borrow.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "whenCanBorrow")]
    pub when_can_borrow: Option<ClusterQueueFlavorFungibilityWhenCanBorrow>,
    /// whenCanPreempt determines whether a workload should try the next flavor
    /// before borrowing in current flavor. The possible values are:
    /// 
    /// 
    /// - `Preempt`: allocate in current flavor if it's possible to preempt some workloads.
    /// - `TryNextFlavor` (default): try next flavor even if there are enough
    ///   candidates for preemption in the current flavor.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "whenCanPreempt")]
    pub when_can_preempt: Option<ClusterQueueFlavorFungibilityWhenCanPreempt>,
}

/// flavorFungibility defines whether a workload should try the next flavor
/// before borrowing or preempting in the flavor being evaluated.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterQueueFlavorFungibilityWhenCanBorrow {
    Borrow,
    TryNextFlavor,
}

/// flavorFungibility defines whether a workload should try the next flavor
/// before borrowing or preempting in the flavor being evaluated.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterQueueFlavorFungibilityWhenCanPreempt {
    Preempt,
    TryNextFlavor,
}

/// namespaceSelector defines which namespaces are allowed to submit workloads to
/// this clusterQueue. Beyond this basic support for policy, a policy agent like
/// Gatekeeper should be used to enforce more advanced policies.
/// Defaults to null which is a nothing selector (no namespaces eligible).
/// If set to an empty selector `{}`, then all namespaces are eligible.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterQueueNamespaceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ClusterQueueNamespaceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterQueueNamespaceSelectorMatchExpressions {
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

/// preemption describes policies to preempt Workloads from this ClusterQueue
/// or the ClusterQueue's cohort.
/// 
/// 
/// Preemption can happen in two scenarios:
/// 
/// 
/// - When a Workload fits within the nominal quota of the ClusterQueue, but
///   the quota is currently borrowed by other ClusterQueues in the cohort.
///   Preempting Workloads in other ClusterQueues allows this ClusterQueue to
///   reclaim its nominal quota.
/// - When a Workload doesn't fit within the nominal quota of the ClusterQueue
///   and there are admitted Workloads in the ClusterQueue with lower priority.
/// 
/// 
/// The preemption algorithm tries to find a minimal set of Workloads to
/// preempt to accomomdate the pending Workload, preempting Workloads with
/// lower priority first.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterQueuePreemption {
    /// borrowWithinCohort provides configuration to allow preemption within
    /// cohort while borrowing.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "borrowWithinCohort")]
    pub borrow_within_cohort: Option<ClusterQueuePreemptionBorrowWithinCohort>,
    /// reclaimWithinCohort determines whether a pending Workload can preempt
    /// Workloads from other ClusterQueues in the cohort that are using more than
    /// their nominal quota. The possible values are:
    /// 
    /// 
    /// - `Never` (default): do not preempt Workloads in the cohort.
    /// - `LowerPriority`: if the pending Workload fits within the nominal
    ///   quota of its ClusterQueue, only preempt Workloads in the cohort that have
    ///   lower priority than the pending Workload.
    /// - `Any`: if the pending Workload fits within the nominal quota of its
    ///   ClusterQueue, preempt any Workload in the cohort, irrespective of
    ///   priority.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reclaimWithinCohort")]
    pub reclaim_within_cohort: Option<ClusterQueuePreemptionReclaimWithinCohort>,
    /// withinClusterQueue determines whether a pending Workload that doesn't fit
    /// within the nominal quota for its ClusterQueue, can preempt active Workloads in
    /// the ClusterQueue. The possible values are:
    /// 
    /// 
    /// - `Never` (default): do not preempt Workloads in the ClusterQueue.
    /// - `LowerPriority`: only preempt Workloads in the ClusterQueue that have
    ///   lower priority than the pending Workload.
    /// - `LowerOrNewerEqualPriority`: only preempt Workloads in the ClusterQueue that
    ///   either have a lower priority than the pending workload or equal priority
    ///   and are newer than the pending workload.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "withinClusterQueue")]
    pub within_cluster_queue: Option<ClusterQueuePreemptionWithinClusterQueue>,
}

/// borrowWithinCohort provides configuration to allow preemption within
/// cohort while borrowing.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterQueuePreemptionBorrowWithinCohort {
    /// maxPriorityThreshold allows to restrict the set of workloads which
    /// might be preempted by a borrowing workload, to only workloads with
    /// priority less than or equal to the specified threshold priority.
    /// When the threshold is not specified, then any workload satisfying the
    /// policy can be preempted by the borrowing workload.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxPriorityThreshold")]
    pub max_priority_threshold: Option<i32>,
    /// policy determines the policy for preemption to reclaim quota within cohort while borrowing.
    /// Possible values are:
    /// - `Never` (default): do not allow for preemption, in other
    ///    ClusterQueues within the cohort, for a borrowing workload.
    /// - `LowerPriority`: allow preemption, in other ClusterQueues
    ///    within the cohort, for a borrowing workload, but only if
    ///    the preempted workloads are of lower priority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<ClusterQueuePreemptionBorrowWithinCohortPolicy>,
}

/// borrowWithinCohort provides configuration to allow preemption within
/// cohort while borrowing.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterQueuePreemptionBorrowWithinCohortPolicy {
    Never,
    LowerPriority,
}

/// preemption describes policies to preempt Workloads from this ClusterQueue
/// or the ClusterQueue's cohort.
/// 
/// 
/// Preemption can happen in two scenarios:
/// 
/// 
/// - When a Workload fits within the nominal quota of the ClusterQueue, but
///   the quota is currently borrowed by other ClusterQueues in the cohort.
///   Preempting Workloads in other ClusterQueues allows this ClusterQueue to
///   reclaim its nominal quota.
/// - When a Workload doesn't fit within the nominal quota of the ClusterQueue
///   and there are admitted Workloads in the ClusterQueue with lower priority.
/// 
/// 
/// The preemption algorithm tries to find a minimal set of Workloads to
/// preempt to accomomdate the pending Workload, preempting Workloads with
/// lower priority first.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterQueuePreemptionReclaimWithinCohort {
    Never,
    LowerPriority,
    Any,
}

/// preemption describes policies to preempt Workloads from this ClusterQueue
/// or the ClusterQueue's cohort.
/// 
/// 
/// Preemption can happen in two scenarios:
/// 
/// 
/// - When a Workload fits within the nominal quota of the ClusterQueue, but
///   the quota is currently borrowed by other ClusterQueues in the cohort.
///   Preempting Workloads in other ClusterQueues allows this ClusterQueue to
///   reclaim its nominal quota.
/// - When a Workload doesn't fit within the nominal quota of the ClusterQueue
///   and there are admitted Workloads in the ClusterQueue with lower priority.
/// 
/// 
/// The preemption algorithm tries to find a minimal set of Workloads to
/// preempt to accomomdate the pending Workload, preempting Workloads with
/// lower priority first.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterQueuePreemptionWithinClusterQueue {
    Never,
    LowerPriority,
    LowerOrNewerEqualPriority,
}

/// ClusterQueueSpec defines the desired state of ClusterQueue
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterQueueQueueingStrategy {
    #[serde(rename = "StrictFIFO")]
    StrictFifo,
    #[serde(rename = "BestEffortFIFO")]
    BestEffortFifo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterQueueResourceGroups {
    /// coveredResources is the list of resources covered by the flavors in this
    /// group.
    /// Examples: cpu, memory, vendor.com/gpu.
    /// The list cannot be empty and it can contain up to 16 resources.
    #[serde(rename = "coveredResources")]
    pub covered_resources: Vec<String>,
    /// flavors is the list of flavors that provide the resources of this group.
    /// Typically, different flavors represent different hardware models
    /// (e.g., gpu models, cpu architectures) or pricing models (on-demand vs spot
    /// cpus).
    /// Each flavor MUST list all the resources listed for this group in the same
    /// order as the .resources field.
    /// The list cannot be empty and it can contain up to 16 flavors.
    pub flavors: Vec<ClusterQueueResourceGroupsFlavors>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterQueueResourceGroupsFlavors {
    /// name of this flavor. The name should match the .metadata.name of a
    /// ResourceFlavor. If a matching ResourceFlavor does not exist, the
    /// ClusterQueue will have an Active condition set to False.
    pub name: String,
    /// resources is the list of quotas for this flavor per resource.
    /// There could be up to 16 resources.
    pub resources: Vec<ClusterQueueResourceGroupsFlavorsResources>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterQueueResourceGroupsFlavorsResources {
    /// borrowingLimit is the maximum amount of quota for the [flavor, resource]
    /// combination that this ClusterQueue is allowed to borrow from the unused
    /// quota of other ClusterQueues in the same cohort.
    /// In total, at a given time, Workloads in a ClusterQueue can consume a
    /// quantity of quota equal to nominalQuota+borrowingLimit, assuming the other
    /// ClusterQueues in the cohort have enough unused quota.
    /// If null, it means that there is no borrowing limit.
    /// If not null, it must be non-negative.
    /// borrowingLimit must be null if spec.cohort is empty.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "borrowingLimit")]
    pub borrowing_limit: Option<IntOrString>,
    /// lendingLimit is the maximum amount of unused quota for the [flavor, resource]
    /// combination that this ClusterQueue can lend to other ClusterQueues in the same cohort.
    /// In total, at a given time, ClusterQueue reserves for its exclusive use
    /// a quantity of quota equals to nominalQuota - lendingLimit.
    /// If null, it means that there is no lending limit, meaning that
    /// all the nominalQuota can be borrowed by other clusterQueues in the cohort.
    /// If not null, it must be non-negative.
    /// lendingLimit must be null if spec.cohort is empty.
    /// This field is in alpha stage. To be able to use this field,
    /// enable the feature gate LendingLimit, which is disabled by default.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lendingLimit")]
    pub lending_limit: Option<IntOrString>,
    /// name of this resource.
    pub name: String,
    /// nominalQuota is the quantity of this resource that is available for
    /// Workloads admitted by this ClusterQueue at a point in time.
    /// The nominalQuota must be non-negative.
    /// nominalQuota should represent the resources in the cluster available for
    /// running jobs (after discounting resources consumed by system components
    /// and pods not managed by kueue). In an autoscaled cluster, nominalQuota
    /// should account for resources that can be provided by a component such as
    /// Kubernetes cluster-autoscaler.
    /// 
    /// 
    /// If the ClusterQueue belongs to a cohort, the sum of the quotas for each
    /// (flavor, resource) combination defines the maximum quantity that can be
    /// allocated by a ClusterQueue in the cohort.
    #[serde(rename = "nominalQuota")]
    pub nominal_quota: IntOrString,
}

/// ClusterQueueSpec defines the desired state of ClusterQueue
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterQueueStopPolicy {
    None,
    Hold,
    HoldAndDrain,
}

/// ClusterQueueStatus defines the observed state of ClusterQueue
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterQueueStatus {
    /// admittedWorkloads is the number of workloads currently admitted to this
    /// clusterQueue and haven't finished yet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "admittedWorkloads")]
    pub admitted_workloads: Option<i32>,
    /// conditions hold the latest available observations of the ClusterQueue
    /// current state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// flavorsReservation are the reserved quotas, by flavor, currently in use by the
    /// workloads assigned to this ClusterQueue.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "flavorsReservation")]
    pub flavors_reservation: Option<Vec<ClusterQueueStatusFlavorsReservation>>,
    /// flavorsUsage are the used quotas, by flavor, currently in use by the
    /// workloads admitted in this ClusterQueue.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "flavorsUsage")]
    pub flavors_usage: Option<Vec<ClusterQueueStatusFlavorsUsage>>,
    /// pendingWorkloads is the number of workloads currently waiting to be
    /// admitted to this clusterQueue.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pendingWorkloads")]
    pub pending_workloads: Option<i32>,
    /// PendingWorkloadsStatus contains the information exposed about the current
    /// status of the pending workloads in the cluster queue.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pendingWorkloadsStatus")]
    pub pending_workloads_status: Option<ClusterQueueStatusPendingWorkloadsStatus>,
    /// reservingWorkloads is the number of workloads currently reserving quota in this
    /// clusterQueue.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reservingWorkloads")]
    pub reserving_workloads: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterQueueStatusFlavorsReservation {
    /// name of the flavor.
    pub name: String,
    /// resources lists the quota usage for the resources in this flavor.
    pub resources: Vec<ClusterQueueStatusFlavorsReservationResources>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterQueueStatusFlavorsReservationResources {
    /// Borrowed is quantity of quota that is borrowed from the cohort. In other
    /// words, it's the used quota that is over the nominalQuota.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub borrowed: Option<IntOrString>,
    /// name of the resource
    pub name: String,
    /// total is the total quantity of used quota, including the amount borrowed
    /// from the cohort.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<IntOrString>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterQueueStatusFlavorsUsage {
    /// name of the flavor.
    pub name: String,
    /// resources lists the quota usage for the resources in this flavor.
    pub resources: Vec<ClusterQueueStatusFlavorsUsageResources>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterQueueStatusFlavorsUsageResources {
    /// Borrowed is quantity of quota that is borrowed from the cohort. In other
    /// words, it's the used quota that is over the nominalQuota.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub borrowed: Option<IntOrString>,
    /// name of the resource
    pub name: String,
    /// total is the total quantity of used quota, including the amount borrowed
    /// from the cohort.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<IntOrString>,
}

/// PendingWorkloadsStatus contains the information exposed about the current
/// status of the pending workloads in the cluster queue.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterQueueStatusPendingWorkloadsStatus {
    /// Head contains the list of top pending workloads.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterQueuePendingWorkload")]
    pub cluster_queue_pending_workload: Option<Vec<ClusterQueueStatusPendingWorkloadsStatusClusterQueuePendingWorkload>>,
    /// LastChangeTime indicates the time of the last change of the structure.
    #[serde(rename = "lastChangeTime")]
    pub last_change_time: String,
}

/// ClusterQueuePendingWorkload contains the information identifying a pending workload
/// in the cluster queue.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterQueueStatusPendingWorkloadsStatusClusterQueuePendingWorkload {
    /// Name indicates the name of the pending workload.
    pub name: String,
    /// Namespace indicates the name of the pending workload.
    pub namespace: String,
}

