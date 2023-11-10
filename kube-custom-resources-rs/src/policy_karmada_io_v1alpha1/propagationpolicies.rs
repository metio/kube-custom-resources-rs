// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/karmada-io/karmada/policy.karmada.io/v1alpha1/propagationpolicies.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// Spec represents the desired behavior of PropagationPolicy.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "policy.karmada.io", version = "v1alpha1", kind = "PropagationPolicy", plural = "propagationpolicies")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct PropagationPolicySpec {
    /// Association tells if relevant resources should be selected automatically. e.g. a ConfigMap referred by a Deployment. default false. Deprecated: in favor of PropagateDeps.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub association: Option<bool>,
    /// ConflictResolution declares how potential conflict should be handled when a resource that is being propagated already exists in the target cluster. 
    ///  It defaults to "Abort" which means stop propagating to avoid unexpected overwrites. The "Overwrite" might be useful when migrating legacy cluster resources to Karmada, in which case conflict is predictable and can be instructed to Karmada take over the resource by overwriting.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "conflictResolution")]
    pub conflict_resolution: Option<PropagationPolicyConflictResolution>,
    /// DependentOverrides represents the list of overrides(OverridePolicy) which must present before the current PropagationPolicy takes effect. 
    ///  It used to explicitly specify overrides which current PropagationPolicy rely on. A typical scenario is the users create OverridePolicy(ies) and resources at the same time, they want to ensure the new-created policies would be adopted. 
    ///  Note: For the overrides, OverridePolicy(ies) in current namespace and ClusterOverridePolicy(ies), which not present in this list will still be applied if they matches the resources.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dependentOverrides")]
    pub dependent_overrides: Option<Vec<String>>,
    /// Failover indicates how Karmada migrates applications in case of failures. If this value is nil, failover is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failover: Option<PropagationPolicyFailover>,
    /// Placement represents the rule for select clusters to propagate resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<PropagationPolicyPlacement>,
    /// Preemption declares the behaviors for preempting. Valid options are "Always" and "Never".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preemption: Option<PropagationPolicyPreemption>,
    /// Priority indicates the importance of a policy(PropagationPolicy or ClusterPropagationPolicy). A policy will be applied for the matched resource templates if there is no other policies with higher priority at the point of the resource template be processed. Once a resource template has been claimed by a policy, by default it will not be preempted by following policies even with a higher priority. See Preemption for more details. 
    ///  In case of two policies have the same priority, the one with a more precise matching rules in ResourceSelectors wins: - matching by name(resourceSelector.name) has higher priority than by selector(resourceSelector.labelSelector) - matching by selector(resourceSelector.labelSelector) has higher priority than by APIVersion(resourceSelector.apiVersion) and Kind(resourceSelector.kind). If there is still no winner at this point, the one with the lower alphabetic order wins, e.g. policy 'bar' has higher priority than 'foo'. 
    ///  The higher the value, the higher the priority. Defaults to zero.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// PropagateDeps tells if relevant resources should be propagated automatically. Take 'Deployment' which referencing 'ConfigMap' and 'Secret' as an example, when 'propagateDeps' is 'true', the referencing resources could be omitted(for saving config effort) from 'resourceSelectors' as they will be propagated along with the Deployment. In addition to the propagating process, the referencing resources will be migrated along with the Deployment in the fail-over scenario. 
    ///  Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "propagateDeps")]
    pub propagate_deps: Option<bool>,
    /// ResourceSelectors used to select resources. Nil or empty selector is not allowed and doesn't mean match all kinds of resources for security concerns that sensitive resources(like Secret) might be accidentally propagated.
    #[serde(rename = "resourceSelectors")]
    pub resource_selectors: Vec<PropagationPolicyResourceSelectors>,
    /// SchedulerName represents which scheduler to proceed the scheduling. If specified, the policy will be dispatched by specified scheduler. If not specified, the policy will be dispatched by default scheduler.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "schedulerName")]
    pub scheduler_name: Option<String>,
}

/// Spec represents the desired behavior of PropagationPolicy.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PropagationPolicyConflictResolution {
    Abort,
    Overwrite,
}

/// Failover indicates how Karmada migrates applications in case of failures. If this value is nil, failover is disabled.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyFailover {
    /// Application indicates failover behaviors in case of application failure. If this value is nil, failover is disabled. If set, the PropagateDeps should be true so that the dependencies could be migrated along with the application.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<PropagationPolicyFailoverApplication>,
}

/// Application indicates failover behaviors in case of application failure. If this value is nil, failover is disabled. If set, the PropagateDeps should be true so that the dependencies could be migrated along with the application.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyFailoverApplication {
    /// DecisionConditions indicates the decision conditions of performing the failover process. Only when all conditions are met can the failover process be performed. Currently, DecisionConditions includes several conditions: - TolerationSeconds (optional)
    #[serde(rename = "decisionConditions")]
    pub decision_conditions: PropagationPolicyFailoverApplicationDecisionConditions,
    /// GracePeriodSeconds is the maximum waiting duration in seconds before application on the migrated cluster should be deleted. Required only when PurgeMode is "Graciously" and defaults to 600s. If the application on the new cluster cannot reach a Healthy state, Karmada will delete the application after GracePeriodSeconds is reached. Value must be positive integer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gracePeriodSeconds")]
    pub grace_period_seconds: Option<i32>,
    /// PurgeMode represents how to deal with the legacy applications on the cluster from which the application is migrated. Valid options are "Immediately", "Graciously" and "Never". Defaults to "Graciously".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "purgeMode")]
    pub purge_mode: Option<String>,
}

/// DecisionConditions indicates the decision conditions of performing the failover process. Only when all conditions are met can the failover process be performed. Currently, DecisionConditions includes several conditions: - TolerationSeconds (optional)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyFailoverApplicationDecisionConditions {
    /// TolerationSeconds represents the period of time Karmada should wait after reaching the desired state before performing failover process. If not specified, Karmada will immediately perform failover process. Defaults to 300s.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tolerationSeconds")]
    pub toleration_seconds: Option<i32>,
}

/// Placement represents the rule for select clusters to propagate resources.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyPlacement {
    /// ClusterAffinities represents scheduling restrictions to multiple cluster groups that indicated by ClusterAffinityTerm. 
    ///  The scheduler will evaluate these groups one by one in the order they appear in the spec, the group that does not satisfy scheduling restrictions will be ignored which means all clusters in this group will not be selected unless it also belongs to the next group(a cluster could belong to multiple groups). 
    ///  If none of the groups satisfy the scheduling restrictions, then scheduling fails, which means no cluster will be selected. 
    ///  Note: 1. ClusterAffinities can not co-exist with ClusterAffinity. 2. If both ClusterAffinity and ClusterAffinities are not set, any cluster can be scheduling candidates. 
    ///  Potential use case 1: The private clusters in the local data center could be the main group, and the managed clusters provided by cluster providers could be the secondary group. So that the Karmada scheduler would prefer to schedule workloads to the main group and the second group will only be considered in case of the main group does not satisfy restrictions(like, lack of resources). 
    ///  Potential use case 2: For the disaster recovery scenario, the clusters could be organized to primary and backup groups, the workloads would be scheduled to primary clusters firstly, and when primary cluster fails(like data center power off), Karmada scheduler could migrate workloads to the backup clusters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterAffinities")]
    pub cluster_affinities: Option<Vec<PropagationPolicyPlacementClusterAffinities>>,
    /// ClusterAffinity represents scheduling restrictions to a certain set of clusters. Note: 1. ClusterAffinity can not co-exist with ClusterAffinities. 2. If both ClusterAffinity and ClusterAffinities are not set, any cluster can be scheduling candidates.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterAffinity")]
    pub cluster_affinity: Option<PropagationPolicyPlacementClusterAffinity>,
    /// ClusterTolerations represents the tolerations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterTolerations")]
    pub cluster_tolerations: Option<Vec<PropagationPolicyPlacementClusterTolerations>>,
    /// ReplicaScheduling represents the scheduling policy on dealing with the number of replicas when propagating resources that have replicas in spec (e.g. deployments, statefulsets) to member clusters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaScheduling")]
    pub replica_scheduling: Option<PropagationPolicyPlacementReplicaScheduling>,
    /// SpreadConstraints represents a list of the scheduling constraints.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "spreadConstraints")]
    pub spread_constraints: Option<Vec<PropagationPolicyPlacementSpreadConstraints>>,
}

/// ClusterAffinityTerm selects a set of cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyPlacementClusterAffinities {
    /// AffinityName is the name of the cluster group.
    #[serde(rename = "affinityName")]
    pub affinity_name: String,
    /// ClusterNames is the list of clusters to be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterNames")]
    pub cluster_names: Option<Vec<String>>,
    /// ExcludedClusters is the list of clusters to be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
    /// FieldSelector is a filter to select member clusters by fields. The key(field) of the match expression should be 'provider', 'region', or 'zone', and the operator of the match expression should be 'In' or 'NotIn'. If non-nil and non-empty, only the clusters match this filter will be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldSelector")]
    pub field_selector: Option<PropagationPolicyPlacementClusterAffinitiesFieldSelector>,
    /// LabelSelector is a filter to select member clusters by labels. If non-nil and non-empty, only the clusters match this filter will be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<PropagationPolicyPlacementClusterAffinitiesLabelSelector>,
}

/// FieldSelector is a filter to select member clusters by fields. The key(field) of the match expression should be 'provider', 'region', or 'zone', and the operator of the match expression should be 'In' or 'NotIn'. If non-nil and non-empty, only the clusters match this filter will be selected.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyPlacementClusterAffinitiesFieldSelector {
    /// A list of field selector requirements.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<PropagationPolicyPlacementClusterAffinitiesFieldSelectorMatchExpressions>>,
}

/// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyPlacementClusterAffinitiesFieldSelectorMatchExpressions {
    /// The label key that the selector applies to.
    pub key: String,
    /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    pub operator: String,
    /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// LabelSelector is a filter to select member clusters by labels. If non-nil and non-empty, only the clusters match this filter will be selected.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyPlacementClusterAffinitiesLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<PropagationPolicyPlacementClusterAffinitiesLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyPlacementClusterAffinitiesLabelSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// ClusterAffinity represents scheduling restrictions to a certain set of clusters. Note: 1. ClusterAffinity can not co-exist with ClusterAffinities. 2. If both ClusterAffinity and ClusterAffinities are not set, any cluster can be scheduling candidates.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyPlacementClusterAffinity {
    /// ClusterNames is the list of clusters to be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterNames")]
    pub cluster_names: Option<Vec<String>>,
    /// ExcludedClusters is the list of clusters to be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
    /// FieldSelector is a filter to select member clusters by fields. The key(field) of the match expression should be 'provider', 'region', or 'zone', and the operator of the match expression should be 'In' or 'NotIn'. If non-nil and non-empty, only the clusters match this filter will be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldSelector")]
    pub field_selector: Option<PropagationPolicyPlacementClusterAffinityFieldSelector>,
    /// LabelSelector is a filter to select member clusters by labels. If non-nil and non-empty, only the clusters match this filter will be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<PropagationPolicyPlacementClusterAffinityLabelSelector>,
}

/// FieldSelector is a filter to select member clusters by fields. The key(field) of the match expression should be 'provider', 'region', or 'zone', and the operator of the match expression should be 'In' or 'NotIn'. If non-nil and non-empty, only the clusters match this filter will be selected.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyPlacementClusterAffinityFieldSelector {
    /// A list of field selector requirements.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<PropagationPolicyPlacementClusterAffinityFieldSelectorMatchExpressions>>,
}

/// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyPlacementClusterAffinityFieldSelectorMatchExpressions {
    /// The label key that the selector applies to.
    pub key: String,
    /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    pub operator: String,
    /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// LabelSelector is a filter to select member clusters by labels. If non-nil and non-empty, only the clusters match this filter will be selected.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyPlacementClusterAffinityLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<PropagationPolicyPlacementClusterAffinityLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyPlacementClusterAffinityLabelSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// The pod this Toleration is attached to tolerates any taint that matches the triple <key,value,effect> using the matching operator <operator>.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyPlacementClusterTolerations {
    /// Effect indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// Key is the taint key that the toleration applies to. Empty means match all taint keys. If the key is empty, operator must be Exists; this combination means to match all values and all keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Operator represents a key's relationship to the value. Valid operators are Exists and Equal. Defaults to Equal. Exists is equivalent to wildcard for value, so that a pod can tolerate all taints of a particular category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// TolerationSeconds represents the period of time the toleration (which must be of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default, it is not set, which means tolerate the taint forever (do not evict). Zero and negative values will be treated as 0 (evict immediately) by the system.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tolerationSeconds")]
    pub toleration_seconds: Option<i64>,
    /// Value is the taint value the toleration matches to. If the operator is Exists, the value should be empty, otherwise just a regular string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// ReplicaScheduling represents the scheduling policy on dealing with the number of replicas when propagating resources that have replicas in spec (e.g. deployments, statefulsets) to member clusters.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyPlacementReplicaScheduling {
    /// ReplicaDivisionPreference determines how the replicas is divided when ReplicaSchedulingType is "Divided". Valid options are Aggregated and Weighted. "Aggregated" divides replicas into clusters as few as possible, while respecting clusters' resource availabilities during the division. "Weighted" divides replicas by weight according to WeightPreference.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaDivisionPreference")]
    pub replica_division_preference: Option<PropagationPolicyPlacementReplicaSchedulingReplicaDivisionPreference>,
    /// ReplicaSchedulingType determines how the replicas is scheduled when karmada propagating a resource. Valid options are Duplicated and Divided. "Duplicated" duplicates the same replicas to each candidate member cluster from resource. "Divided" divides replicas into parts according to number of valid candidate member clusters, and exact replicas for each cluster are determined by ReplicaDivisionPreference.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaSchedulingType")]
    pub replica_scheduling_type: Option<PropagationPolicyPlacementReplicaSchedulingReplicaSchedulingType>,
    /// WeightPreference describes weight for each cluster or for each group of cluster If ReplicaDivisionPreference is set to "Weighted", and WeightPreference is not set, scheduler will weight all clusters the same.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "weightPreference")]
    pub weight_preference: Option<PropagationPolicyPlacementReplicaSchedulingWeightPreference>,
}

/// ReplicaScheduling represents the scheduling policy on dealing with the number of replicas when propagating resources that have replicas in spec (e.g. deployments, statefulsets) to member clusters.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PropagationPolicyPlacementReplicaSchedulingReplicaDivisionPreference {
    Aggregated,
    Weighted,
}

/// ReplicaScheduling represents the scheduling policy on dealing with the number of replicas when propagating resources that have replicas in spec (e.g. deployments, statefulsets) to member clusters.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PropagationPolicyPlacementReplicaSchedulingReplicaSchedulingType {
    Duplicated,
    Divided,
}

/// WeightPreference describes weight for each cluster or for each group of cluster If ReplicaDivisionPreference is set to "Weighted", and WeightPreference is not set, scheduler will weight all clusters the same.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyPlacementReplicaSchedulingWeightPreference {
    /// DynamicWeight specifies the factor to generates dynamic weight list. If specified, StaticWeightList will be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dynamicWeight")]
    pub dynamic_weight: Option<PropagationPolicyPlacementReplicaSchedulingWeightPreferenceDynamicWeight>,
    /// StaticWeightList defines the static cluster weight.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "staticWeightList")]
    pub static_weight_list: Option<Vec<PropagationPolicyPlacementReplicaSchedulingWeightPreferenceStaticWeightList>>,
}

/// WeightPreference describes weight for each cluster or for each group of cluster If ReplicaDivisionPreference is set to "Weighted", and WeightPreference is not set, scheduler will weight all clusters the same.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PropagationPolicyPlacementReplicaSchedulingWeightPreferenceDynamicWeight {
    AvailableReplicas,
}

/// StaticClusterWeight defines the static cluster weight.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyPlacementReplicaSchedulingWeightPreferenceStaticWeightList {
    /// TargetCluster describes the filter to select clusters.
    #[serde(rename = "targetCluster")]
    pub target_cluster: PropagationPolicyPlacementReplicaSchedulingWeightPreferenceStaticWeightListTargetCluster,
    /// Weight expressing the preference to the cluster(s) specified by 'TargetCluster'.
    pub weight: i64,
}

/// TargetCluster describes the filter to select clusters.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyPlacementReplicaSchedulingWeightPreferenceStaticWeightListTargetCluster {
    /// ClusterNames is the list of clusters to be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterNames")]
    pub cluster_names: Option<Vec<String>>,
    /// ExcludedClusters is the list of clusters to be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
    /// FieldSelector is a filter to select member clusters by fields. The key(field) of the match expression should be 'provider', 'region', or 'zone', and the operator of the match expression should be 'In' or 'NotIn'. If non-nil and non-empty, only the clusters match this filter will be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldSelector")]
    pub field_selector: Option<PropagationPolicyPlacementReplicaSchedulingWeightPreferenceStaticWeightListTargetClusterFieldSelector>,
    /// LabelSelector is a filter to select member clusters by labels. If non-nil and non-empty, only the clusters match this filter will be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<PropagationPolicyPlacementReplicaSchedulingWeightPreferenceStaticWeightListTargetClusterLabelSelector>,
}

/// FieldSelector is a filter to select member clusters by fields. The key(field) of the match expression should be 'provider', 'region', or 'zone', and the operator of the match expression should be 'In' or 'NotIn'. If non-nil and non-empty, only the clusters match this filter will be selected.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyPlacementReplicaSchedulingWeightPreferenceStaticWeightListTargetClusterFieldSelector {
    /// A list of field selector requirements.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<PropagationPolicyPlacementReplicaSchedulingWeightPreferenceStaticWeightListTargetClusterFieldSelectorMatchExpressions>>,
}

/// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyPlacementReplicaSchedulingWeightPreferenceStaticWeightListTargetClusterFieldSelectorMatchExpressions {
    /// The label key that the selector applies to.
    pub key: String,
    /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    pub operator: String,
    /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// LabelSelector is a filter to select member clusters by labels. If non-nil and non-empty, only the clusters match this filter will be selected.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyPlacementReplicaSchedulingWeightPreferenceStaticWeightListTargetClusterLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<PropagationPolicyPlacementReplicaSchedulingWeightPreferenceStaticWeightListTargetClusterLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyPlacementReplicaSchedulingWeightPreferenceStaticWeightListTargetClusterLabelSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// SpreadConstraint represents the spread constraints on resources.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyPlacementSpreadConstraints {
    /// MaxGroups restricts the maximum number of cluster groups to be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxGroups")]
    pub max_groups: Option<i64>,
    /// MinGroups restricts the minimum number of cluster groups to be selected. Defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minGroups")]
    pub min_groups: Option<i64>,
    /// SpreadByField represents the fields on Karmada cluster API used for dynamically grouping member clusters into different groups. Resources will be spread among different cluster groups. Available fields for spreading are: cluster, region, zone, and provider. SpreadByField should not co-exist with SpreadByLabel. If both SpreadByField and SpreadByLabel are empty, SpreadByField will be set to "cluster" by system.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "spreadByField")]
    pub spread_by_field: Option<PropagationPolicyPlacementSpreadConstraintsSpreadByField>,
    /// SpreadByLabel represents the label key used for grouping member clusters into different groups. Resources will be spread among different cluster groups. SpreadByLabel should not co-exist with SpreadByField.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "spreadByLabel")]
    pub spread_by_label: Option<String>,
}

/// SpreadConstraint represents the spread constraints on resources.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PropagationPolicyPlacementSpreadConstraintsSpreadByField {
    #[serde(rename = "cluster")]
    Cluster,
    #[serde(rename = "region")]
    Region,
    #[serde(rename = "zone")]
    Zone,
    #[serde(rename = "provider")]
    Provider,
}

/// Spec represents the desired behavior of PropagationPolicy.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PropagationPolicyPreemption {
    Always,
    Never,
}

/// ResourceSelector the resources will be selected.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyResourceSelectors {
    /// APIVersion represents the API version of the target resources.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Kind represents the Kind of the target resources.
    pub kind: String,
    /// A label query over a set of resources. If name is not empty, labelSelector will be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<PropagationPolicyResourceSelectorsLabelSelector>,
    /// Name of the target resource. Default is empty, which means selecting all resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the target resource. Default is empty, which means inherit from the parent object scope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// A label query over a set of resources. If name is not empty, labelSelector will be ignored.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyResourceSelectorsLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<PropagationPolicyResourceSelectorsLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagationPolicyResourceSelectorsLabelSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

