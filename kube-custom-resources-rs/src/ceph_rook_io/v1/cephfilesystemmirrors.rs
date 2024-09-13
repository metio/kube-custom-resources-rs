// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/rook/rook/ceph.rook.io/v1/cephfilesystemmirrors.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// FilesystemMirroringSpec is the filesystem mirroring specification
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "ceph.rook.io", version = "v1", kind = "CephFilesystemMirror", plural = "cephfilesystemmirrors")]
#[kube(namespaced)]
#[kube(status = "CephFilesystemMirrorStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct CephFilesystemMirrorSpec {
    /// The annotations-related configuration to add/set on each Pod related object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// The labels-related configuration to add/set on each Pod related object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<CephFilesystemMirrorPlacement>,
    /// PriorityClassName sets priority class on the cephfs-mirror pods
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "priorityClassName")]
    pub priority_class_name: Option<String>,
    /// The resource requirements for the cephfs-mirror pods
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<CephFilesystemMirrorResources>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacement {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeAffinity")]
    pub node_affinity: Option<CephFilesystemMirrorPlacementNodeAffinity>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podAffinity")]
    pub pod_affinity: Option<CephFilesystemMirrorPlacementPodAffinity>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podAntiAffinity")]
    pub pod_anti_affinity: Option<CephFilesystemMirrorPlacementPodAntiAffinity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<Vec<CephFilesystemMirrorPlacementTolerations>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "topologySpreadConstraints")]
    pub topology_spread_constraints: Option<Vec<CephFilesystemMirrorPlacementTopologySpreadConstraints>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementNodeAffinity {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredDuringSchedulingIgnoredDuringExecution")]
    pub preferred_during_scheduling_ignored_during_execution: Option<Vec<CephFilesystemMirrorPlacementNodeAffinityPreferredDuringSchedulingIgnoredDuringExecution>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requiredDuringSchedulingIgnoredDuringExecution")]
    pub required_during_scheduling_ignored_during_execution: Option<CephFilesystemMirrorPlacementNodeAffinityRequiredDuringSchedulingIgnoredDuringExecution>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementNodeAffinityPreferredDuringSchedulingIgnoredDuringExecution {
    pub preference: CephFilesystemMirrorPlacementNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreference,
    pub weight: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreference {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CephFilesystemMirrorPlacementNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreferenceMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchFields")]
    pub match_fields: Option<Vec<CephFilesystemMirrorPlacementNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreferenceMatchFields>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreferenceMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreferenceMatchFields {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementNodeAffinityRequiredDuringSchedulingIgnoredDuringExecution {
    #[serde(rename = "nodeSelectorTerms")]
    pub node_selector_terms: Vec<CephFilesystemMirrorPlacementNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTerms>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTerms {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CephFilesystemMirrorPlacementNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTermsMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchFields")]
    pub match_fields: Option<Vec<CephFilesystemMirrorPlacementNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTermsMatchFields>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTermsMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTermsMatchFields {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementPodAffinity {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredDuringSchedulingIgnoredDuringExecution")]
    pub preferred_during_scheduling_ignored_during_execution: Option<Vec<CephFilesystemMirrorPlacementPodAffinityPreferredDuringSchedulingIgnoredDuringExecution>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requiredDuringSchedulingIgnoredDuringExecution")]
    pub required_during_scheduling_ignored_during_execution: Option<Vec<CephFilesystemMirrorPlacementPodAffinityRequiredDuringSchedulingIgnoredDuringExecution>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementPodAffinityPreferredDuringSchedulingIgnoredDuringExecution {
    #[serde(rename = "podAffinityTerm")]
    pub pod_affinity_term: CephFilesystemMirrorPlacementPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTerm,
    pub weight: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTerm {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<CephFilesystemMirrorPlacementPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabelKeys")]
    pub match_label_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mismatchLabelKeys")]
    pub mismatch_label_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceSelector")]
    pub namespace_selector: Option<CephFilesystemMirrorPlacementPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    #[serde(rename = "topologyKey")]
    pub topology_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CephFilesystemMirrorPlacementPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CephFilesystemMirrorPlacementPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementPodAffinityRequiredDuringSchedulingIgnoredDuringExecution {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<CephFilesystemMirrorPlacementPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabelKeys")]
    pub match_label_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mismatchLabelKeys")]
    pub mismatch_label_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceSelector")]
    pub namespace_selector: Option<CephFilesystemMirrorPlacementPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    #[serde(rename = "topologyKey")]
    pub topology_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CephFilesystemMirrorPlacementPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CephFilesystemMirrorPlacementPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementPodAntiAffinity {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredDuringSchedulingIgnoredDuringExecution")]
    pub preferred_during_scheduling_ignored_during_execution: Option<Vec<CephFilesystemMirrorPlacementPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecution>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requiredDuringSchedulingIgnoredDuringExecution")]
    pub required_during_scheduling_ignored_during_execution: Option<Vec<CephFilesystemMirrorPlacementPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecution>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecution {
    #[serde(rename = "podAffinityTerm")]
    pub pod_affinity_term: CephFilesystemMirrorPlacementPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTerm,
    pub weight: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTerm {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<CephFilesystemMirrorPlacementPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabelKeys")]
    pub match_label_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mismatchLabelKeys")]
    pub mismatch_label_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceSelector")]
    pub namespace_selector: Option<CephFilesystemMirrorPlacementPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    #[serde(rename = "topologyKey")]
    pub topology_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CephFilesystemMirrorPlacementPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CephFilesystemMirrorPlacementPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecution {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<CephFilesystemMirrorPlacementPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabelKeys")]
    pub match_label_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mismatchLabelKeys")]
    pub mismatch_label_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceSelector")]
    pub namespace_selector: Option<CephFilesystemMirrorPlacementPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    #[serde(rename = "topologyKey")]
    pub topology_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CephFilesystemMirrorPlacementPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CephFilesystemMirrorPlacementPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementTolerations {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tolerationSeconds")]
    pub toleration_seconds: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementTopologySpreadConstraints {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<CephFilesystemMirrorPlacementTopologySpreadConstraintsLabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabelKeys")]
    pub match_label_keys: Option<Vec<String>>,
    #[serde(rename = "maxSkew")]
    pub max_skew: i32,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minDomains")]
    pub min_domains: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeAffinityPolicy")]
    pub node_affinity_policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeTaintsPolicy")]
    pub node_taints_policy: Option<String>,
    #[serde(rename = "topologyKey")]
    pub topology_key: String,
    #[serde(rename = "whenUnsatisfiable")]
    pub when_unsatisfiable: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementTopologySpreadConstraintsLabelSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CephFilesystemMirrorPlacementTopologySpreadConstraintsLabelSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorPlacementTopologySpreadConstraintsLabelSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// The resource requirements for the cephfs-mirror pods
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorResources {
    /// Claims lists the names of resources, defined in spec.resourceClaims,
    /// that are used by this container.
    /// 
    /// This is an alpha field and requires enabling the
    /// DynamicResourceAllocation feature gate.
    /// 
    /// This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<CephFilesystemMirrorResourcesClaims>>,
    /// Limits describes the maximum amount of compute resources allowed.
    /// More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required.
    /// If Requests is omitted for a container, it defaults to Limits if that is explicitly specified,
    /// otherwise to an implementation-defined value. Requests cannot exceed Limits.
    /// More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// ResourceClaim references one entry in PodSpec.ResourceClaims.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorResourcesClaims {
    /// Name must match the name of one entry in pod.spec.resourceClaims of
    /// the Pod where this field is used. It makes that resource available
    /// inside a container.
    pub name: String,
    /// Request is the name chosen for a request in the referenced claim.
    /// If empty, everything from the claim is made available, otherwise
    /// only the result of this request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
}

/// Status represents the status of an object
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemMirrorStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// ObservedGeneration is the latest generation observed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}

