// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubernetes-sigs/kueue/kueue.x-k8s.io/v1beta1/cohorts.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
}
use self::prelude::*;

/// CohortSpec defines the desired state of Cohort
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kueue.x-k8s.io", version = "v1beta1", kind = "Cohort", plural = "cohorts")]
#[kube(status = "CohortStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct CohortSpec {
    /// fairSharing defines the properties of the Cohort when
    /// participating in FairSharing. The values are only relevant
    /// if FairSharing is enabled in the Kueue configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fairSharing")]
    pub fair_sharing: Option<CohortFairSharing>,
    /// ParentName references the name of the Cohort's parent, if
    /// any. It satisfies one of three cases:
    /// 1) Unset. This Cohort is the root of its Cohort tree.
    /// 2) References a non-existent Cohort. We use default Cohort (no borrowing/lending limits).
    /// 3) References an existent Cohort.
    /// 
    /// If a cycle is created, we disable all members of the
    /// Cohort, including ClusterQueues, until the cycle is
    /// removed.  We prevent further admission while the cycle
    /// exists.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parentName")]
    pub parent_name: Option<String>,
    /// ResourceGroups describes groupings of Resources and
    /// Flavors.  Each ResourceGroup defines a list of Resources
    /// and a list of Flavors which provide quotas for these
    /// Resources. Each Resource and each Flavor may only form part
    /// of one ResourceGroup.  There may be up to 16 ResourceGroups
    /// within a Cohort.
    /// 
    /// BorrowingLimit limits how much members of this Cohort
    /// subtree can borrow from the parent subtree.
    /// 
    /// LendingLimit limits how much members of this Cohort subtree
    /// can lend to the parent subtree.
    /// 
    /// Borrowing and Lending limits must only be set when the
    /// Cohort has a parent.  Otherwise, the Cohort create/update
    /// will be rejected by the webhook.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceGroups")]
    pub resource_groups: Option<Vec<CohortResourceGroups>>,
}

/// fairSharing defines the properties of the Cohort when
/// participating in FairSharing. The values are only relevant
/// if FairSharing is enabled in the Kueue configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CohortFairSharing {
    /// weight gives a comparative advantage to this ClusterQueue
    /// or Cohort when competing for unused resources in the
    /// Cohort.  The share is based on the dominant resource usage
    /// above nominal quotas for each resource, divided by the
    /// weight.  Admission prioritizes scheduling workloads from
    /// ClusterQueues and Cohorts with the lowest share and
    /// preempting workloads from the ClusterQueues and Cohorts
    /// with the highest share.  A zero weight implies infinite
    /// share value, meaning that this Node will always be at
    /// disadvantage against other ClusterQueues and Cohorts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<IntOrString>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CohortResourceGroups {
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
    pub flavors: Vec<CohortResourceGroupsFlavors>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CohortResourceGroupsFlavors {
    /// name of this flavor. The name should match the .metadata.name of a
    /// ResourceFlavor. If a matching ResourceFlavor does not exist, the
    /// ClusterQueue will have an Active condition set to False.
    pub name: String,
    /// resources is the list of quotas for this flavor per resource.
    /// There could be up to 16 resources.
    pub resources: Vec<CohortResourceGroupsFlavorsResources>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CohortResourceGroupsFlavorsResources {
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
    /// This field is in beta stage and is enabled by default.
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
    /// If the ClusterQueue belongs to a cohort, the sum of the quotas for each
    /// (flavor, resource) combination defines the maximum quantity that can be
    /// allocated by a ClusterQueue in the cohort.
    #[serde(rename = "nominalQuota")]
    pub nominal_quota: IntOrString,
}

/// CohortStatus defines the observed state of Cohort.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CohortStatus {
    /// fairSharing contains the current state for this Cohort
    /// when participating in Fair Sharing.
    /// The is recorded only when Fair Sharing is enabled in the Kueue configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fairSharing")]
    pub fair_sharing: Option<CohortStatusFairSharing>,
}

/// fairSharing contains the current state for this Cohort
/// when participating in Fair Sharing.
/// The is recorded only when Fair Sharing is enabled in the Kueue configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CohortStatusFairSharing {
    /// admissionFairSharingStatus represents information relevant to the Admission Fair Sharing
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "admissionFairSharingStatus")]
    pub admission_fair_sharing_status: Option<CohortStatusFairSharingAdmissionFairSharingStatus>,
    /// WeightedShare represents the maximum of the ratios of usage
    /// above nominal quota to the lendable resources in the
    /// Cohort, among all the resources provided by the Node, and
    /// divided by the weight.  If zero, it means that the usage of
    /// the Node is below the nominal quota.  If the Node has a
    /// weight of zero and is borrowing, this will return
    /// 9223372036854775807, the maximum possible share value.
    #[serde(rename = "weightedShare")]
    pub weighted_share: i64,
}

/// admissionFairSharingStatus represents information relevant to the Admission Fair Sharing
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CohortStatusFairSharingAdmissionFairSharingStatus {
    /// ConsumedResources represents the aggregated usage of resources over time,
    /// with decaying function applied.
    /// The value is populated if usage consumption functionality is enabled in Kueue config.
    #[serde(rename = "consumedResources")]
    pub consumed_resources: BTreeMap<String, IntOrString>,
    /// LastUpdate is the time when share and consumed resources were updated.
    #[serde(rename = "lastUpdate")]
    pub last_update: String,
}

