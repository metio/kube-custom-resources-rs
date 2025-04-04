// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubewharf/kubeadmiral/core.kubeadmiral.io/v1alpha1/clusteroverridepolicies.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "core.kubeadmiral.io", version = "v1alpha1", kind = "ClusterOverridePolicy", plural = "clusteroverridepolicies")]
#[kube(status = "ClusterOverridePolicyStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ClusterOverridePolicySpec {
    /// OverrideRules specify the override rules.
    /// Each rule specifies the overriders and the clusters these overriders should be applied to.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "overrideRules")]
    pub override_rules: Option<Vec<ClusterOverridePolicyOverrideRules>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOverridePolicyOverrideRules {
    /// Overriders specify the overriders to be applied in the target clusters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overriders: Option<ClusterOverridePolicyOverrideRulesOverriders>,
    /// TargetClusters selects the clusters in which the overriders in this rule should be applied.
    /// If multiple types of selectors are specified, the overall result is the intersection of all of them.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetClusters")]
    pub target_clusters: Option<ClusterOverridePolicyOverrideRulesTargetClusters>,
}

/// Overriders specify the overriders to be applied in the target clusters.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesOverriders {
    /// Annotation specifies overriders that apply to the resource annotations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<ClusterOverridePolicyOverrideRulesOverridersAnnotations>>,
    /// Args specifies overriders that apply to the container arguments.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<ClusterOverridePolicyOverrideRulesOverridersArgs>>,
    /// Command specifies overriders that apply to the container commands.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<ClusterOverridePolicyOverrideRulesOverridersCommand>>,
    /// Envs specifies overriders that apply to the container envs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub envs: Option<Vec<ClusterOverridePolicyOverrideRulesOverridersEnvs>>,
    /// Image specifies the overriders that apply to the image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<Vec<ClusterOverridePolicyOverrideRulesOverridersImage>>,
    /// JsonPatch specifies overriders in a syntax similar to RFC6902 JSON Patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jsonpatch: Option<Vec<ClusterOverridePolicyOverrideRulesOverridersJsonpatch>>,
    /// Label specifies overriders that apply to the resource labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<ClusterOverridePolicyOverrideRulesOverridersLabels>>,
}

/// StringMapOverrider represents the rules dedicated to handling resource labels/annotations
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesOverridersAnnotations {
    /// Operator specifies the operation.
    /// If omitted, defaults to "overwrite".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<ClusterOverridePolicyOverrideRulesOverridersAnnotationsOperator>,
    /// Value is the value(s) that will be applied to annotations/labels of resource.
    /// If Operator is 'addIfAbsent', items in Value (empty is not allowed) will be added in annotations/labels.
    ///   - For 'addIfAbsent' Operator, the keys in Value cannot conflict with annotations/labels.
    /// If Operator is 'overwrite', items in Value which match in annotations/labels will be replaced.
    /// If Operator is 'delete', items in Value which match in annotations/labels will be deleted.
    pub value: BTreeMap<String, String>,
}

/// StringMapOverrider represents the rules dedicated to handling resource labels/annotations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterOverridePolicyOverrideRulesOverridersAnnotationsOperator {
    #[serde(rename = "addIfAbsent")]
    AddIfAbsent,
    #[serde(rename = "overwrite")]
    Overwrite,
    #[serde(rename = "delete")]
    Delete,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesOverridersArgs {
    /// ContainerName targets the specified container or init container in the pod template.
    #[serde(rename = "containerName")]
    pub container_name: String,
    /// Operator specifies the operation.
    /// If omitted, defaults to "overwrite".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<ClusterOverridePolicyOverrideRulesOverridersArgsOperator>,
    /// Value is the value(s) that will be applied to command/args of ContainerName.
    /// If Operator is 'append', items in Value (empty is not allowed) will be appended to command/args.
    /// If Operator is 'overwrite', current command/args of ContainerName will be completely replaced by Value.
    /// If Operator is 'delete', items in Value that match in command/args will be deleted.
    pub value: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterOverridePolicyOverrideRulesOverridersArgsOperator {
    #[serde(rename = "append")]
    Append,
    #[serde(rename = "overwrite")]
    Overwrite,
    #[serde(rename = "delete")]
    Delete,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesOverridersCommand {
    /// ContainerName targets the specified container or init container in the pod template.
    #[serde(rename = "containerName")]
    pub container_name: String,
    /// Operator specifies the operation.
    /// If omitted, defaults to "overwrite".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<ClusterOverridePolicyOverrideRulesOverridersCommandOperator>,
    /// Value is the value(s) that will be applied to command/args of ContainerName.
    /// If Operator is 'append', items in Value (empty is not allowed) will be appended to command/args.
    /// If Operator is 'overwrite', current command/args of ContainerName will be completely replaced by Value.
    /// If Operator is 'delete', items in Value that match in command/args will be deleted.
    pub value: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterOverridePolicyOverrideRulesOverridersCommandOperator {
    #[serde(rename = "append")]
    Append,
    #[serde(rename = "overwrite")]
    Overwrite,
    #[serde(rename = "delete")]
    Delete,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesOverridersEnvs {
    /// ContainerName targets the specified container or init container in the pod template.
    #[serde(rename = "containerName")]
    pub container_name: String,
    /// Operator specifies the operation.
    /// If omitted, defaults to "overwrite".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<ClusterOverridePolicyOverrideRulesOverridersEnvsOperator>,
    /// List of environment variables to set in the container.
    pub value: Vec<ClusterOverridePolicyOverrideRulesOverridersEnvsValue>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterOverridePolicyOverrideRulesOverridersEnvsOperator {
    #[serde(rename = "addIfAbsent")]
    AddIfAbsent,
    #[serde(rename = "overwrite")]
    Overwrite,
    #[serde(rename = "delete")]
    Delete,
}

/// EnvVar represents an environment variable present in a Container.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesOverridersEnvsValue {
    /// Name of the environment variable. Must be a C_IDENTIFIER.
    pub name: String,
    /// Variable references $(VAR_NAME) are expanded
    /// using the previously defined environment variables in the container and
    /// any service environment variables. If a variable cannot be resolved,
    /// the reference in the input string will be unchanged. Double $$ are reduced
    /// to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e.
    /// "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)".
    /// Escaped references will never be expanded, regardless of whether the variable
    /// exists or not.
    /// Defaults to "".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Source for the environment variable's value. Cannot be used if value is not empty.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<ClusterOverridePolicyOverrideRulesOverridersEnvsValueValueFrom>,
}

/// Source for the environment variable's value. Cannot be used if value is not empty.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesOverridersEnvsValueValueFrom {
    /// Selects a key of a ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapKeyRef")]
    pub config_map_key_ref: Option<ClusterOverridePolicyOverrideRulesOverridersEnvsValueValueFromConfigMapKeyRef>,
    /// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`,
    /// spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldRef")]
    pub field_ref: Option<ClusterOverridePolicyOverrideRulesOverridersEnvsValueValueFromFieldRef>,
    /// Selects a resource of the container: only resources limits and requests
    /// (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceFieldRef")]
    pub resource_field_ref: Option<ClusterOverridePolicyOverrideRulesOverridersEnvsValueValueFromResourceFieldRef>,
    /// Selects a key of a secret in the pod's namespace
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<ClusterOverridePolicyOverrideRulesOverridersEnvsValueValueFromSecretKeyRef>,
}

/// Selects a key of a ConfigMap.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesOverridersEnvsValueValueFromConfigMapKeyRef {
    /// The key to select.
    pub key: String,
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`,
/// spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesOverridersEnvsValueValueFromFieldRef {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Path of the field to select in the specified API version.
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

/// Selects a resource of the container: only resources limits and requests
/// (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesOverridersEnvsValueValueFromResourceFieldRef {
    /// Container name: required for volumes, optional for env vars
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerName")]
    pub container_name: Option<String>,
    /// Specifies the output format of the exposed resources, defaults to "1"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<IntOrString>,
    /// Required: resource to select
    pub resource: String,
}

/// Selects a key of a secret in the pod's namespace
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesOverridersEnvsValueValueFromSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesOverridersImage {
    /// ContainerNames are ignored when ImagePath is set.
    /// If empty, the image override rule applies to all containers.
    /// Otherwise, this override targets the specified container(s) or init container(s) in the pod template.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerNames")]
    pub container_names: Option<Vec<String>>,
    /// ImagePath indicates the image path to target.
    /// For Example: /spec/template/spec/containers/0/image
    /// 
    /// If empty, the system will automatically resolve the image path if the resource type is
    /// Pod, CronJob, Deployment, StatefulSet, DaemonSet or Job.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePath")]
    pub image_path: Option<String>,
    /// Operations are the specific operations to be performed on ContainerNames or ImagePath.
    pub operations: Vec<ClusterOverridePolicyOverrideRulesOverridersImageOperations>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesOverridersImageOperations {
    /// ImageComponent is the part of the image to override.
    #[serde(rename = "imageComponent")]
    pub image_component: ClusterOverridePolicyOverrideRulesOverridersImageOperationsImageComponent,
    /// Operator specifies the operation.
    /// If omitted, defaults to "overwrite".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<ClusterOverridePolicyOverrideRulesOverridersImageOperationsOperator>,
    /// Value is the value required by the operation.
    /// For 'addIfAbsent' Operator, the old value of ImageComponent should be empty, and the Value shouldn't be empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterOverridePolicyOverrideRulesOverridersImageOperationsImageComponent {
    Registry,
    Repository,
    Tag,
    Digest,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterOverridePolicyOverrideRulesOverridersImageOperationsOperator {
    #[serde(rename = "addIfAbsent")]
    AddIfAbsent,
    #[serde(rename = "overwrite")]
    Overwrite,
    #[serde(rename = "delete")]
    Delete,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesOverridersJsonpatch {
    /// Operator specifies the operation.
    /// If omitted, defaults to "replace".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// Path is a JSON pointer (RFC 6901) specifying the location within the resource document where the
    /// operation is performed.
    /// Each key in the path should be prefixed with "/",
    /// while "~" and "/" should be escaped as "~0" and "~1" respectively.
    /// For example, to add a label "kubeadmiral.io/label",
    /// the path should be "/metadata/labels/kubeadmiral.io~1label".
    pub path: String,
    /// Value is the value(s) required by the operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

/// StringMapOverrider represents the rules dedicated to handling resource labels/annotations
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesOverridersLabels {
    /// Operator specifies the operation.
    /// If omitted, defaults to "overwrite".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<ClusterOverridePolicyOverrideRulesOverridersLabelsOperator>,
    /// Value is the value(s) that will be applied to annotations/labels of resource.
    /// If Operator is 'addIfAbsent', items in Value (empty is not allowed) will be added in annotations/labels.
    ///   - For 'addIfAbsent' Operator, the keys in Value cannot conflict with annotations/labels.
    /// If Operator is 'overwrite', items in Value which match in annotations/labels will be replaced.
    /// If Operator is 'delete', items in Value which match in annotations/labels will be deleted.
    pub value: BTreeMap<String, String>,
}

/// StringMapOverrider represents the rules dedicated to handling resource labels/annotations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterOverridePolicyOverrideRulesOverridersLabelsOperator {
    #[serde(rename = "addIfAbsent")]
    AddIfAbsent,
    #[serde(rename = "overwrite")]
    Overwrite,
    #[serde(rename = "delete")]
    Delete,
}

/// TargetClusters selects the clusters in which the overriders in this rule should be applied.
/// If multiple types of selectors are specified, the overall result is the intersection of all of them.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesTargetClusters {
    /// ClusterAffinity selects FederatedClusters by matching their labels and fields against expressions.
    /// If multiple terms are specified, their results are ORed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterAffinity")]
    pub cluster_affinity: Option<Vec<ClusterOverridePolicyOverrideRulesTargetClustersClusterAffinity>>,
    /// ClusterSelector selects FederatedClusters by their labels.
    /// Empty labels selects all FederatedClusters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterSelector")]
    pub cluster_selector: Option<BTreeMap<String, String>>,
    /// Clusters selects FederatedClusters by their names.
    /// Empty Clusters selects all FederatedClusters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesTargetClustersClusterAffinity {
    /// A list of cluster selector requirements by cluster labels.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ClusterOverridePolicyOverrideRulesTargetClustersClusterAffinityMatchExpressions>>,
    /// A list of cluster selector requirements by cluster fields.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchFields")]
    pub match_fields: Option<Vec<ClusterOverridePolicyOverrideRulesTargetClustersClusterAffinityMatchFields>>,
}

/// ClusterSelectorRequirement is a selector that contains values, a key, and an operator that relates the values and keys
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesTargetClustersClusterAffinityMatchExpressions {
    pub key: String,
    /// ClusterSelectorOperator is the set of operators that can be used in a cluster selector requirement.
    pub operator: ClusterOverridePolicyOverrideRulesTargetClustersClusterAffinityMatchExpressionsOperator,
    pub values: Vec<String>,
}

/// ClusterSelectorRequirement is a selector that contains values, a key, and an operator that relates the values and keys
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterOverridePolicyOverrideRulesTargetClustersClusterAffinityMatchExpressionsOperator {
    In,
    NotIn,
    Exists,
    DoesNotExist,
    Gt,
    Lt,
}

/// ClusterSelectorRequirement is a selector that contains values, a key, and an operator that relates the values and keys
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterOverridePolicyOverrideRulesTargetClustersClusterAffinityMatchFields {
    pub key: String,
    /// ClusterSelectorOperator is the set of operators that can be used in a cluster selector requirement.
    pub operator: ClusterOverridePolicyOverrideRulesTargetClustersClusterAffinityMatchFieldsOperator,
    pub values: Vec<String>,
}

/// ClusterSelectorRequirement is a selector that contains values, a key, and an operator that relates the values and keys
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterOverridePolicyOverrideRulesTargetClustersClusterAffinityMatchFieldsOperator {
    In,
    NotIn,
    Exists,
    DoesNotExist,
    Gt,
    Lt,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOverridePolicyStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refCount")]
    pub ref_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "typedRefCount")]
    pub typed_ref_count: Option<Vec<ClusterOverridePolicyStatusTypedRefCount>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOverridePolicyStatusTypedRefCount {
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    pub resource: String,
}

