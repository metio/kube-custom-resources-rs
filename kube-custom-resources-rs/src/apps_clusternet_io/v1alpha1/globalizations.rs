// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/clusternet/clusternet/apps.clusternet.io/v1alpha1/globalizations.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// GlobalizationSpec defines the desired state of Globalization
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "apps.clusternet.io", version = "v1alpha1", kind = "Globalization", plural = "globalizations")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct GlobalizationSpec {
    /// ClusterAffinity is a label query over managed clusters by labels.
    /// If no labels are specified, all clusters will be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterAffinity")]
    pub cluster_affinity: Option<GlobalizationClusterAffinity>,
    /// Feed holds references to the objects the Globalization applies to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feed: Option<GlobalizationFeed>,
    /// OverridePolicy specifies the override policy for this Globalization.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "overridePolicy")]
    pub override_policy: Option<GlobalizationOverridePolicy>,
    /// Overrides holds all the OverrideConfig.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<Vec<GlobalizationOverrides>>,
    /// Priority is an integer defining the relative importance of this Globalization compared to others.
    /// Lower numbers are considered lower priority.
    /// And these Globalization(s) will be applied by order from lower priority to higher.
    /// That means override values in lower Globalization will be overridden by those in higher Globalization.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

/// ClusterAffinity is a label query over managed clusters by labels.
/// If no labels are specified, all clusters will be selected.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GlobalizationClusterAffinity {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<GlobalizationClusterAffinityMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GlobalizationClusterAffinityMatchExpressions {
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

/// Feed holds references to the objects the Globalization applies to.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GlobalizationFeed {
    /// APIVersion defines the versioned schema of this representation of an object.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Kind is a string value representing the REST resource this object represents.
    /// In CamelCase.
    pub kind: String,
    /// Name of the target resource.
    pub name: String,
    /// Namespace of the target resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// GlobalizationSpec defines the desired state of Globalization
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum GlobalizationOverridePolicy {
    ApplyNow,
    ApplyLater,
}

/// OverrideConfig holds information that describes a override config.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GlobalizationOverrides {
    /// Name indicate the OverrideConfig name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// OverrideChart indicates whether the override value for the HelmChart CR.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "overrideChart")]
    pub override_chart: Option<bool>,
    /// Type specifies the override type for override value.
    #[serde(rename = "type")]
    pub r#type: GlobalizationOverridesType,
    /// Value represents override value.
    pub value: String,
}

/// OverrideConfig holds information that describes a override config.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum GlobalizationOverridesType {
    Helm,
    #[serde(rename = "JSONPatch")]
    JsonPatch,
    MergePatch,
}

