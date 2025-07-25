// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/external-secrets/external-secrets/external-secrets.io/v1alpha1/clusterpushsecrets.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "external-secrets.io", version = "v1alpha1", kind = "ClusterPushSecret", plural = "clusterpushsecrets")]
#[kube(status = "ClusterPushSecretStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ClusterPushSecretSpec {
    /// A list of labels to select by to find the Namespaces to create the ExternalSecrets in. The selectors are ORed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceSelectors")]
    pub namespace_selectors: Option<Vec<ClusterPushSecretNamespaceSelectors>>,
    /// The metadata of the external secrets to be created
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pushSecretMetadata")]
    pub push_secret_metadata: Option<ClusterPushSecretPushSecretMetadata>,
    /// The name of the push secrets to be created.
    /// Defaults to the name of the ClusterPushSecret
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pushSecretName")]
    pub push_secret_name: Option<String>,
    /// PushSecretSpec defines what to do with the secrets.
    #[serde(rename = "pushSecretSpec")]
    pub push_secret_spec: ClusterPushSecretPushSecretSpec,
    /// The time in which the controller should reconcile its objects and recheck namespaces for labels.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refreshTime")]
    pub refresh_time: Option<String>,
}

/// A label selector is a label query over a set of resources. The result of matchLabels and
/// matchExpressions are ANDed. An empty label selector matches all objects. A null
/// label selector matches no objects.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPushSecretNamespaceSelectors {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ClusterPushSecretNamespaceSelectorsMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPushSecretNamespaceSelectorsMatchExpressions {
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

/// The metadata of the external secrets to be created
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPushSecretPushSecretMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

/// PushSecretSpec defines what to do with the secrets.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPushSecretPushSecretSpec {
    /// Secret Data that should be pushed to providers
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ClusterPushSecretPushSecretSpecData>>,
    /// Deletion Policy to handle Secrets in the provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deletionPolicy")]
    pub deletion_policy: Option<ClusterPushSecretPushSecretSpecDeletionPolicy>,
    /// The Interval to which External Secrets will try to push a secret definition
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refreshInterval")]
    pub refresh_interval: Option<String>,
    #[serde(rename = "secretStoreRefs")]
    pub secret_store_refs: Vec<ClusterPushSecretPushSecretSpecSecretStoreRefs>,
    /// The Secret Selector (k8s source) for the Push Secret
    pub selector: ClusterPushSecretPushSecretSpecSelector,
    /// Template defines a blueprint for the created Secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<ClusterPushSecretPushSecretSpecTemplate>,
    /// UpdatePolicy to handle Secrets in the provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updatePolicy")]
    pub update_policy: Option<ClusterPushSecretPushSecretSpecUpdatePolicy>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPushSecretPushSecretSpecData {
    /// Used to define a conversion Strategy for the secret keys
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "conversionStrategy")]
    pub conversion_strategy: Option<ClusterPushSecretPushSecretSpecDataConversionStrategy>,
    /// Match a given Secret Key to be pushed to the provider.
    #[serde(rename = "match")]
    pub r#match: ClusterPushSecretPushSecretSpecDataMatch,
    /// Metadata is metadata attached to the secret.
    /// The structure of metadata is provider specific, please look it up in the provider documentation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterPushSecretPushSecretSpecDataConversionStrategy {
    None,
    ReverseUnicode,
}

/// Match a given Secret Key to be pushed to the provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPushSecretPushSecretSpecDataMatch {
    /// Remote Refs to push to providers.
    #[serde(rename = "remoteRef")]
    pub remote_ref: ClusterPushSecretPushSecretSpecDataMatchRemoteRef,
    /// Secret Key to be pushed
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKey")]
    pub secret_key: Option<String>,
}

/// Remote Refs to push to providers.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPushSecretPushSecretSpecDataMatchRemoteRef {
    /// Name of the property in the resulting secret
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,
    /// Name of the resulting provider secret.
    #[serde(rename = "remoteKey")]
    pub remote_key: String,
}

/// PushSecretSpec defines what to do with the secrets.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterPushSecretPushSecretSpecDeletionPolicy {
    Delete,
    None,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPushSecretPushSecretSpecSecretStoreRefs {
    /// Kind of the SecretStore resource (SecretStore or ClusterSecretStore)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ClusterPushSecretPushSecretSpecSecretStoreRefsKind>,
    /// Optionally, sync to secret stores with label selector
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<ClusterPushSecretPushSecretSpecSecretStoreRefsLabelSelector>,
    /// Optionally, sync to the SecretStore of the given name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterPushSecretPushSecretSpecSecretStoreRefsKind {
    SecretStore,
    ClusterSecretStore,
}

/// Optionally, sync to secret stores with label selector
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPushSecretPushSecretSpecSecretStoreRefsLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ClusterPushSecretPushSecretSpecSecretStoreRefsLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPushSecretPushSecretSpecSecretStoreRefsLabelSelectorMatchExpressions {
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

/// The Secret Selector (k8s source) for the Push Secret
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPushSecretPushSecretSpecSelector {
    /// Point to a generator to create a Secret.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "generatorRef")]
    pub generator_ref: Option<ClusterPushSecretPushSecretSpecSelectorGeneratorRef>,
    /// Select a Secret to Push.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<ClusterPushSecretPushSecretSpecSelectorSecret>,
}

/// Point to a generator to create a Secret.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterPushSecretPushSecretSpecSelectorGeneratorRef {
    /// Specify the apiVersion of the generator resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Specify the Kind of the generator resource
    pub kind: ClusterPushSecretPushSecretSpecSelectorGeneratorRefKind,
    /// Specify the name of the generator resource
    pub name: String,
}

/// Point to a generator to create a Secret.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterPushSecretPushSecretSpecSelectorGeneratorRefKind {
    #[serde(rename = "ACRAccessToken")]
    AcrAccessToken,
    ClusterGenerator,
    #[serde(rename = "ECRAuthorizationToken")]
    EcrAuthorizationToken,
    Fake,
    #[serde(rename = "GCRAccessToken")]
    GcrAccessToken,
    GithubAccessToken,
    QuayAccessToken,
    Password,
    #[serde(rename = "STSSessionToken")]
    StsSessionToken,
    #[serde(rename = "UUID")]
    Uuid,
    VaultDynamicSecret,
    Webhook,
    Grafana,
    #[serde(rename = "MFA")]
    Mfa,
}

/// Select a Secret to Push.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPushSecretPushSecretSpecSelectorSecret {
    /// Name of the Secret.
    /// The Secret must exist in the same namespace as the PushSecret manifest.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Selector chooses secrets using a labelSelector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<ClusterPushSecretPushSecretSpecSelectorSecretSelector>,
}

/// Selector chooses secrets using a labelSelector.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPushSecretPushSecretSpecSelectorSecretSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ClusterPushSecretPushSecretSpecSelectorSecretSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPushSecretPushSecretSpecSelectorSecretSelectorMatchExpressions {
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

/// Template defines a blueprint for the created Secret resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPushSecretPushSecretSpecTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<BTreeMap<String, String>>,
    /// EngineVersion specifies the template engine version
    /// that should be used to compile/execute the
    /// template specified in .data and .templateFrom[].
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "engineVersion")]
    pub engine_version: Option<ClusterPushSecretPushSecretSpecTemplateEngineVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mergePolicy")]
    pub merge_policy: Option<ClusterPushSecretPushSecretSpecTemplateMergePolicy>,
    /// ExternalSecretTemplateMetadata defines metadata fields for the Secret blueprint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ClusterPushSecretPushSecretSpecTemplateMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "templateFrom")]
    pub template_from: Option<Vec<ClusterPushSecretPushSecretSpecTemplateTemplateFrom>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// Template defines a blueprint for the created Secret resource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterPushSecretPushSecretSpecTemplateEngineVersion {
    #[serde(rename = "v2")]
    V2,
}

/// Template defines a blueprint for the created Secret resource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterPushSecretPushSecretSpecTemplateMergePolicy {
    Replace,
    Merge,
}

/// ExternalSecretTemplateMetadata defines metadata fields for the Secret blueprint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPushSecretPushSecretSpecTemplateMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPushSecretPushSecretSpecTemplateTemplateFrom {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMap")]
    pub config_map: Option<ClusterPushSecretPushSecretSpecTemplateTemplateFromConfigMap>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub literal: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<ClusterPushSecretPushSecretSpecTemplateTemplateFromSecret>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<ClusterPushSecretPushSecretSpecTemplateTemplateFromTarget>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPushSecretPushSecretSpecTemplateTemplateFromConfigMap {
    /// A list of keys in the ConfigMap/Secret to use as templates for Secret data
    pub items: Vec<ClusterPushSecretPushSecretSpecTemplateTemplateFromConfigMapItems>,
    /// The name of the ConfigMap/Secret resource
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPushSecretPushSecretSpecTemplateTemplateFromConfigMapItems {
    /// A key in the ConfigMap/Secret
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "templateAs")]
    pub template_as: Option<ClusterPushSecretPushSecretSpecTemplateTemplateFromConfigMapItemsTemplateAs>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterPushSecretPushSecretSpecTemplateTemplateFromConfigMapItemsTemplateAs {
    Values,
    KeysAndValues,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPushSecretPushSecretSpecTemplateTemplateFromSecret {
    /// A list of keys in the ConfigMap/Secret to use as templates for Secret data
    pub items: Vec<ClusterPushSecretPushSecretSpecTemplateTemplateFromSecretItems>,
    /// The name of the ConfigMap/Secret resource
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPushSecretPushSecretSpecTemplateTemplateFromSecretItems {
    /// A key in the ConfigMap/Secret
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "templateAs")]
    pub template_as: Option<ClusterPushSecretPushSecretSpecTemplateTemplateFromSecretItemsTemplateAs>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterPushSecretPushSecretSpecTemplateTemplateFromSecretItemsTemplateAs {
    Values,
    KeysAndValues,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterPushSecretPushSecretSpecTemplateTemplateFromTarget {
    Data,
    Annotations,
    Labels,
}

/// PushSecretSpec defines what to do with the secrets.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterPushSecretPushSecretSpecUpdatePolicy {
    Replace,
    IfNotExists,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPushSecretStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Failed namespaces are the namespaces that failed to apply an PushSecret
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failedNamespaces")]
    pub failed_namespaces: Option<Vec<ClusterPushSecretStatusFailedNamespaces>>,
    /// ProvisionedNamespaces are the namespaces where the ClusterPushSecret has secrets
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "provisionedNamespaces")]
    pub provisioned_namespaces: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pushSecretName")]
    pub push_secret_name: Option<String>,
}

/// ClusterPushSecretNamespaceFailure represents a failed namespace deployment and it's reason.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPushSecretStatusFailedNamespaces {
    /// Namespace is the namespace that failed when trying to apply an PushSecret
    pub namespace: String,
    /// Reason is why the PushSecret failed to apply to the namespace
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

