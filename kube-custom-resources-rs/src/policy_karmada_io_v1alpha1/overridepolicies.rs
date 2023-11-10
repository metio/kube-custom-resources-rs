// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/karmada-io/karmada/policy.karmada.io/v1alpha1/overridepolicies.yaml --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::collections::HashMap;

/// Spec represents the desired behavior of OverridePolicy.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "policy.karmada.io", version = "v1alpha1", kind = "OverridePolicy", plural = "overridepolicies")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct OverridePolicySpec {
    /// OverrideRules defines a collection of override rules on target clusters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "overrideRules")]
    pub override_rules: Option<Vec<OverridePolicyOverrideRules>>,
    /// Overriders represents the override rules that would apply on resources 
    ///  Deprecated: This filed is deprecated in v1.0 and please use the OverrideRules instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overriders: Option<OverridePolicyOverriders>,
    /// ResourceSelectors restricts resource types that this override policy applies to. nil means matching all resources.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceSelectors")]
    pub resource_selectors: Option<Vec<OverridePolicyResourceSelectors>>,
    /// TargetCluster defines restrictions on this override policy that only applies to resources propagated to the matching clusters. nil means matching all clusters. 
    ///  Deprecated: This filed is deprecated in v1.0 and please use the OverrideRules instead.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetCluster")]
    pub target_cluster: Option<OverridePolicyTargetCluster>,
}

/// RuleWithCluster defines the override rules on clusters.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRules {
    /// Overriders represents the override rules that would apply on resources
    pub overriders: OverridePolicyOverrideRulesOverriders,
    /// TargetCluster defines restrictions on this override policy that only applies to resources propagated to the matching clusters. nil means matching all clusters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetCluster")]
    pub target_cluster: Option<OverridePolicyOverrideRulesTargetCluster>,
}

/// Overriders represents the override rules that would apply on resources
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRulesOverriders {
    /// AnnotationsOverrider represents the rules dedicated to handling workload annotations
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "annotationsOverrider")]
    pub annotations_overrider: Option<Vec<OverridePolicyOverrideRulesOverridersAnnotationsOverrider>>,
    /// ArgsOverrider represents the rules dedicated to handling container args
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "argsOverrider")]
    pub args_overrider: Option<Vec<OverridePolicyOverrideRulesOverridersArgsOverrider>>,
    /// CommandOverrider represents the rules dedicated to handling container command
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "commandOverrider")]
    pub command_overrider: Option<Vec<OverridePolicyOverrideRulesOverridersCommandOverrider>>,
    /// ImageOverrider represents the rules dedicated to handling image overrides.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageOverrider")]
    pub image_overrider: Option<Vec<OverridePolicyOverrideRulesOverridersImageOverrider>>,
    /// LabelsOverrider represents the rules dedicated to handling workload labels
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelsOverrider")]
    pub labels_overrider: Option<Vec<OverridePolicyOverrideRulesOverridersLabelsOverrider>>,
    /// Plaintext represents override rules defined with plaintext overriders.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plaintext: Option<Vec<OverridePolicyOverrideRulesOverridersPlaintext>>,
}

/// LabelAnnotationOverrider represents the rules dedicated to handling workload labels/annotations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRulesOverridersAnnotationsOverrider {
    /// Operator represents the operator which will apply on the workload.
    pub operator: OverridePolicyOverrideRulesOverridersAnnotationsOverriderOperator,
    /// Value to be applied to annotations/labels of workload. Items in Value which will be appended after annotations/labels when Operator is 'add'. Items in Value which match in annotations/labels will be deleted when Operator is 'remove'. Items in Value which match in annotations/labels will be replaced when Operator is 'replace'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<BTreeMap<String, String>>,
}

/// LabelAnnotationOverrider represents the rules dedicated to handling workload labels/annotations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OverridePolicyOverrideRulesOverridersAnnotationsOverriderOperator {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "replace")]
    Replace,
}

/// CommandArgsOverrider represents the rules dedicated to handling command/args overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRulesOverridersArgsOverrider {
    /// The name of container
    #[serde(rename = "containerName")]
    pub container_name: String,
    /// Operator represents the operator which will apply on the command/args.
    pub operator: OverridePolicyOverrideRulesOverridersArgsOverriderOperator,
    /// Value to be applied to command/args. Items in Value which will be appended after command/args when Operator is 'add'. Items in Value which match in command/args will be deleted when Operator is 'remove'. If Value is empty, then the command/args will remain the same.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

/// CommandArgsOverrider represents the rules dedicated to handling command/args overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OverridePolicyOverrideRulesOverridersArgsOverriderOperator {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
}

/// CommandArgsOverrider represents the rules dedicated to handling command/args overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRulesOverridersCommandOverrider {
    /// The name of container
    #[serde(rename = "containerName")]
    pub container_name: String,
    /// Operator represents the operator which will apply on the command/args.
    pub operator: OverridePolicyOverrideRulesOverridersCommandOverriderOperator,
    /// Value to be applied to command/args. Items in Value which will be appended after command/args when Operator is 'add'. Items in Value which match in command/args will be deleted when Operator is 'remove'. If Value is empty, then the command/args will remain the same.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

/// CommandArgsOverrider represents the rules dedicated to handling command/args overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OverridePolicyOverrideRulesOverridersCommandOverriderOperator {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
}

/// ImageOverrider represents the rules dedicated to handling image overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRulesOverridersImageOverrider {
    /// Component is part of image name. Basically we presume an image can be made of '[registry/]repository[:tag]'. The registry could be: - registry.k8s.io - fictional.registry.example:10443 The repository could be: - kube-apiserver - fictional/nginx The tag cloud be: - latest - v1.19.1 - @sha256:dbcc1c35ac38df41fd2f5e4130b32ffdb93ebae8b3dbe638c23575912276fc9c
    pub component: OverridePolicyOverrideRulesOverridersImageOverriderComponent,
    /// Operator represents the operator which will apply on the image.
    pub operator: OverridePolicyOverrideRulesOverridersImageOverriderOperator,
    /// Predicate filters images before applying the rule. 
    ///  Defaults to nil, in that case, the system will automatically detect image fields if the resource type is Pod, ReplicaSet, Deployment, StatefulSet, DaemonSet or Job by following rule: - Pod: /spec/containers/<N>/image - ReplicaSet: /spec/template/spec/containers/<N>/image - Deployment: /spec/template/spec/containers/<N>/image - DaemonSet: /spec/template/spec/containers/<N>/image - StatefulSet: /spec/template/spec/containers/<N>/image - Job: /spec/template/spec/containers/<N>/image In addition, all images will be processed if the resource object has more than one container. 
    ///  If not nil, only images matches the filters will be processed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub predicate: Option<OverridePolicyOverrideRulesOverridersImageOverriderPredicate>,
    /// Value to be applied to image. Must not be empty when operator is 'add' or 'replace'. Defaults to empty and ignored when operator is 'remove'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// ImageOverrider represents the rules dedicated to handling image overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OverridePolicyOverrideRulesOverridersImageOverriderComponent {
    Registry,
    Repository,
    Tag,
}

/// ImageOverrider represents the rules dedicated to handling image overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OverridePolicyOverrideRulesOverridersImageOverriderOperator {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "replace")]
    Replace,
}

/// Predicate filters images before applying the rule. 
///  Defaults to nil, in that case, the system will automatically detect image fields if the resource type is Pod, ReplicaSet, Deployment, StatefulSet, DaemonSet or Job by following rule: - Pod: /spec/containers/<N>/image - ReplicaSet: /spec/template/spec/containers/<N>/image - Deployment: /spec/template/spec/containers/<N>/image - DaemonSet: /spec/template/spec/containers/<N>/image - StatefulSet: /spec/template/spec/containers/<N>/image - Job: /spec/template/spec/containers/<N>/image In addition, all images will be processed if the resource object has more than one container. 
///  If not nil, only images matches the filters will be processed.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRulesOverridersImageOverriderPredicate {
    /// Path indicates the path of target field
    pub path: String,
}

/// LabelAnnotationOverrider represents the rules dedicated to handling workload labels/annotations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRulesOverridersLabelsOverrider {
    /// Operator represents the operator which will apply on the workload.
    pub operator: OverridePolicyOverrideRulesOverridersLabelsOverriderOperator,
    /// Value to be applied to annotations/labels of workload. Items in Value which will be appended after annotations/labels when Operator is 'add'. Items in Value which match in annotations/labels will be deleted when Operator is 'remove'. Items in Value which match in annotations/labels will be replaced when Operator is 'replace'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<BTreeMap<String, String>>,
}

/// LabelAnnotationOverrider represents the rules dedicated to handling workload labels/annotations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OverridePolicyOverrideRulesOverridersLabelsOverriderOperator {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "replace")]
    Replace,
}

/// PlaintextOverrider is a simple overrider that overrides target fields according to path, operator and value.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRulesOverridersPlaintext {
    /// Operator indicates the operation on target field. Available operators are: add, replace and remove.
    pub operator: OverridePolicyOverrideRulesOverridersPlaintextOperator,
    /// Path indicates the path of target field
    pub path: String,
    /// Value to be applied to target field. Must be empty when operator is Remove.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<HashMap<String, serde_json::Value>>,
}

/// PlaintextOverrider is a simple overrider that overrides target fields according to path, operator and value.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OverridePolicyOverrideRulesOverridersPlaintextOperator {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "replace")]
    Replace,
}

/// TargetCluster defines restrictions on this override policy that only applies to resources propagated to the matching clusters. nil means matching all clusters.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRulesTargetCluster {
    /// ClusterNames is the list of clusters to be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterNames")]
    pub cluster_names: Option<Vec<String>>,
    /// ExcludedClusters is the list of clusters to be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
    /// FieldSelector is a filter to select member clusters by fields. The key(field) of the match expression should be 'provider', 'region', or 'zone', and the operator of the match expression should be 'In' or 'NotIn'. If non-nil and non-empty, only the clusters match this filter will be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldSelector")]
    pub field_selector: Option<OverridePolicyOverrideRulesTargetClusterFieldSelector>,
    /// LabelSelector is a filter to select member clusters by labels. If non-nil and non-empty, only the clusters match this filter will be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<OverridePolicyOverrideRulesTargetClusterLabelSelector>,
}

/// FieldSelector is a filter to select member clusters by fields. The key(field) of the match expression should be 'provider', 'region', or 'zone', and the operator of the match expression should be 'In' or 'NotIn'. If non-nil and non-empty, only the clusters match this filter will be selected.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRulesTargetClusterFieldSelector {
    /// A list of field selector requirements.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<OverridePolicyOverrideRulesTargetClusterFieldSelectorMatchExpressions>>,
}

/// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRulesTargetClusterFieldSelectorMatchExpressions {
    /// The label key that the selector applies to.
    pub key: String,
    /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    pub operator: String,
    /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// LabelSelector is a filter to select member clusters by labels. If non-nil and non-empty, only the clusters match this filter will be selected.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRulesTargetClusterLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<OverridePolicyOverrideRulesTargetClusterLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRulesTargetClusterLabelSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Overriders represents the override rules that would apply on resources 
///  Deprecated: This filed is deprecated in v1.0 and please use the OverrideRules instead.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverriders {
    /// AnnotationsOverrider represents the rules dedicated to handling workload annotations
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "annotationsOverrider")]
    pub annotations_overrider: Option<Vec<OverridePolicyOverridersAnnotationsOverrider>>,
    /// ArgsOverrider represents the rules dedicated to handling container args
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "argsOverrider")]
    pub args_overrider: Option<Vec<OverridePolicyOverridersArgsOverrider>>,
    /// CommandOverrider represents the rules dedicated to handling container command
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "commandOverrider")]
    pub command_overrider: Option<Vec<OverridePolicyOverridersCommandOverrider>>,
    /// ImageOverrider represents the rules dedicated to handling image overrides.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageOverrider")]
    pub image_overrider: Option<Vec<OverridePolicyOverridersImageOverrider>>,
    /// LabelsOverrider represents the rules dedicated to handling workload labels
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelsOverrider")]
    pub labels_overrider: Option<Vec<OverridePolicyOverridersLabelsOverrider>>,
    /// Plaintext represents override rules defined with plaintext overriders.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plaintext: Option<Vec<OverridePolicyOverridersPlaintext>>,
}

/// LabelAnnotationOverrider represents the rules dedicated to handling workload labels/annotations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverridersAnnotationsOverrider {
    /// Operator represents the operator which will apply on the workload.
    pub operator: OverridePolicyOverridersAnnotationsOverriderOperator,
    /// Value to be applied to annotations/labels of workload. Items in Value which will be appended after annotations/labels when Operator is 'add'. Items in Value which match in annotations/labels will be deleted when Operator is 'remove'. Items in Value which match in annotations/labels will be replaced when Operator is 'replace'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<BTreeMap<String, String>>,
}

/// LabelAnnotationOverrider represents the rules dedicated to handling workload labels/annotations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OverridePolicyOverridersAnnotationsOverriderOperator {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "replace")]
    Replace,
}

/// CommandArgsOverrider represents the rules dedicated to handling command/args overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverridersArgsOverrider {
    /// The name of container
    #[serde(rename = "containerName")]
    pub container_name: String,
    /// Operator represents the operator which will apply on the command/args.
    pub operator: OverridePolicyOverridersArgsOverriderOperator,
    /// Value to be applied to command/args. Items in Value which will be appended after command/args when Operator is 'add'. Items in Value which match in command/args will be deleted when Operator is 'remove'. If Value is empty, then the command/args will remain the same.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

/// CommandArgsOverrider represents the rules dedicated to handling command/args overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OverridePolicyOverridersArgsOverriderOperator {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
}

/// CommandArgsOverrider represents the rules dedicated to handling command/args overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverridersCommandOverrider {
    /// The name of container
    #[serde(rename = "containerName")]
    pub container_name: String,
    /// Operator represents the operator which will apply on the command/args.
    pub operator: OverridePolicyOverridersCommandOverriderOperator,
    /// Value to be applied to command/args. Items in Value which will be appended after command/args when Operator is 'add'. Items in Value which match in command/args will be deleted when Operator is 'remove'. If Value is empty, then the command/args will remain the same.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

/// CommandArgsOverrider represents the rules dedicated to handling command/args overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OverridePolicyOverridersCommandOverriderOperator {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
}

/// ImageOverrider represents the rules dedicated to handling image overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverridersImageOverrider {
    /// Component is part of image name. Basically we presume an image can be made of '[registry/]repository[:tag]'. The registry could be: - registry.k8s.io - fictional.registry.example:10443 The repository could be: - kube-apiserver - fictional/nginx The tag cloud be: - latest - v1.19.1 - @sha256:dbcc1c35ac38df41fd2f5e4130b32ffdb93ebae8b3dbe638c23575912276fc9c
    pub component: OverridePolicyOverridersImageOverriderComponent,
    /// Operator represents the operator which will apply on the image.
    pub operator: OverridePolicyOverridersImageOverriderOperator,
    /// Predicate filters images before applying the rule. 
    ///  Defaults to nil, in that case, the system will automatically detect image fields if the resource type is Pod, ReplicaSet, Deployment, StatefulSet, DaemonSet or Job by following rule: - Pod: /spec/containers/<N>/image - ReplicaSet: /spec/template/spec/containers/<N>/image - Deployment: /spec/template/spec/containers/<N>/image - DaemonSet: /spec/template/spec/containers/<N>/image - StatefulSet: /spec/template/spec/containers/<N>/image - Job: /spec/template/spec/containers/<N>/image In addition, all images will be processed if the resource object has more than one container. 
    ///  If not nil, only images matches the filters will be processed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub predicate: Option<OverridePolicyOverridersImageOverriderPredicate>,
    /// Value to be applied to image. Must not be empty when operator is 'add' or 'replace'. Defaults to empty and ignored when operator is 'remove'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// ImageOverrider represents the rules dedicated to handling image overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OverridePolicyOverridersImageOverriderComponent {
    Registry,
    Repository,
    Tag,
}

/// ImageOverrider represents the rules dedicated to handling image overrides.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OverridePolicyOverridersImageOverriderOperator {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "replace")]
    Replace,
}

/// Predicate filters images before applying the rule. 
///  Defaults to nil, in that case, the system will automatically detect image fields if the resource type is Pod, ReplicaSet, Deployment, StatefulSet, DaemonSet or Job by following rule: - Pod: /spec/containers/<N>/image - ReplicaSet: /spec/template/spec/containers/<N>/image - Deployment: /spec/template/spec/containers/<N>/image - DaemonSet: /spec/template/spec/containers/<N>/image - StatefulSet: /spec/template/spec/containers/<N>/image - Job: /spec/template/spec/containers/<N>/image In addition, all images will be processed if the resource object has more than one container. 
///  If not nil, only images matches the filters will be processed.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverridersImageOverriderPredicate {
    /// Path indicates the path of target field
    pub path: String,
}

/// LabelAnnotationOverrider represents the rules dedicated to handling workload labels/annotations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverridersLabelsOverrider {
    /// Operator represents the operator which will apply on the workload.
    pub operator: OverridePolicyOverridersLabelsOverriderOperator,
    /// Value to be applied to annotations/labels of workload. Items in Value which will be appended after annotations/labels when Operator is 'add'. Items in Value which match in annotations/labels will be deleted when Operator is 'remove'. Items in Value which match in annotations/labels will be replaced when Operator is 'replace'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<BTreeMap<String, String>>,
}

/// LabelAnnotationOverrider represents the rules dedicated to handling workload labels/annotations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OverridePolicyOverridersLabelsOverriderOperator {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "replace")]
    Replace,
}

/// PlaintextOverrider is a simple overrider that overrides target fields according to path, operator and value.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverridersPlaintext {
    /// Operator indicates the operation on target field. Available operators are: add, replace and remove.
    pub operator: OverridePolicyOverridersPlaintextOperator,
    /// Path indicates the path of target field
    pub path: String,
    /// Value to be applied to target field. Must be empty when operator is Remove.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<HashMap<String, serde_json::Value>>,
}

/// PlaintextOverrider is a simple overrider that overrides target fields according to path, operator and value.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OverridePolicyOverridersPlaintextOperator {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "replace")]
    Replace,
}

/// ResourceSelector the resources will be selected.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyResourceSelectors {
    /// APIVersion represents the API version of the target resources.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Kind represents the Kind of the target resources.
    pub kind: String,
    /// A label query over a set of resources. If name is not empty, labelSelector will be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<OverridePolicyResourceSelectorsLabelSelector>,
    /// Name of the target resource. Default is empty, which means selecting all resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the target resource. Default is empty, which means inherit from the parent object scope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// A label query over a set of resources. If name is not empty, labelSelector will be ignored.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyResourceSelectorsLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<OverridePolicyResourceSelectorsLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyResourceSelectorsLabelSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// TargetCluster defines restrictions on this override policy that only applies to resources propagated to the matching clusters. nil means matching all clusters. 
///  Deprecated: This filed is deprecated in v1.0 and please use the OverrideRules instead.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyTargetCluster {
    /// ClusterNames is the list of clusters to be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterNames")]
    pub cluster_names: Option<Vec<String>>,
    /// ExcludedClusters is the list of clusters to be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
    /// FieldSelector is a filter to select member clusters by fields. The key(field) of the match expression should be 'provider', 'region', or 'zone', and the operator of the match expression should be 'In' or 'NotIn'. If non-nil and non-empty, only the clusters match this filter will be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldSelector")]
    pub field_selector: Option<OverridePolicyTargetClusterFieldSelector>,
    /// LabelSelector is a filter to select member clusters by labels. If non-nil and non-empty, only the clusters match this filter will be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<OverridePolicyTargetClusterLabelSelector>,
}

/// FieldSelector is a filter to select member clusters by fields. The key(field) of the match expression should be 'provider', 'region', or 'zone', and the operator of the match expression should be 'In' or 'NotIn'. If non-nil and non-empty, only the clusters match this filter will be selected.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyTargetClusterFieldSelector {
    /// A list of field selector requirements.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<OverridePolicyTargetClusterFieldSelectorMatchExpressions>>,
}

/// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyTargetClusterFieldSelectorMatchExpressions {
    /// The label key that the selector applies to.
    pub key: String,
    /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    pub operator: String,
    /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// LabelSelector is a filter to select member clusters by labels. If non-nil and non-empty, only the clusters match this filter will be selected.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyTargetClusterLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<OverridePolicyTargetClusterLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyTargetClusterLabelSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

