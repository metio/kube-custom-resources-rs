// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/external-secrets/external-secrets/external-secrets.io/v1beta1/externalsecrets.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// ExternalSecretSpec defines the desired state of ExternalSecret.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "external-secrets.io", version = "v1beta1", kind = "ExternalSecret", plural = "externalsecrets")]
#[kube(namespaced)]
#[kube(status = "ExternalSecretStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ExternalSecretSpec {
    /// Data defines the connection between the Kubernetes Secret keys and the Provider data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ExternalSecretData>>,
    /// DataFrom is used to fetch all properties from a specific Provider data
    /// If multiple entries are specified, the Secret keys are merged in the specified order
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataFrom")]
    pub data_from: Option<Vec<ExternalSecretDataFrom>>,
    /// RefreshInterval is the amount of time before the values are read again from the SecretStore provider,
    /// specified as Golang Duration strings.
    /// Valid time units are "ns", "us" (or "µs"), "ms", "s", "m", "h"
    /// Example values: "1h", "2h30m", "5d", "10s"
    /// May be set to zero to fetch and create it once. Defaults to 1h.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refreshInterval")]
    pub refresh_interval: Option<String>,
    /// SecretStoreRef defines which SecretStore to fetch the ExternalSecret data.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretStoreRef")]
    pub secret_store_ref: Option<ExternalSecretSecretStoreRef>,
    /// ExternalSecretTarget defines the Kubernetes Secret to be created
    /// There can be only one target per ExternalSecret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<ExternalSecretTarget>,
}

/// ExternalSecretData defines the connection between the Kubernetes Secret key (spec.data.<key>) and the Provider data.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExternalSecretData {
    /// RemoteRef points to the remote secret and defines
    /// which secret (version/property/..) to fetch.
    #[serde(rename = "remoteRef")]
    pub remote_ref: ExternalSecretDataRemoteRef,
    /// The key in the Kubernetes Secret to store the value.
    #[serde(rename = "secretKey")]
    pub secret_key: String,
    /// SourceRef allows you to override the source
    /// from which the value will be pulled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceRef")]
    pub source_ref: Option<ExternalSecretDataSourceRef>,
}

/// RemoteRef points to the remote secret and defines
/// which secret (version/property/..) to fetch.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExternalSecretDataRemoteRef {
    /// Used to define a conversion Strategy
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "conversionStrategy")]
    pub conversion_strategy: Option<ExternalSecretDataRemoteRefConversionStrategy>,
    /// Used to define a decoding Strategy
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "decodingStrategy")]
    pub decoding_strategy: Option<ExternalSecretDataRemoteRefDecodingStrategy>,
    /// Key is the key used in the Provider, mandatory
    pub key: String,
    /// Policy for fetching tags/labels from provider secrets, possible options are Fetch, None. Defaults to None
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metadataPolicy")]
    pub metadata_policy: Option<ExternalSecretDataRemoteRefMetadataPolicy>,
    /// Used to select a specific property of the Provider value (if a map), if supported
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,
    /// Used to select a specific version of the Provider value, if supported
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// RemoteRef points to the remote secret and defines
/// which secret (version/property/..) to fetch.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExternalSecretDataRemoteRefConversionStrategy {
    Default,
    Unicode,
}

/// RemoteRef points to the remote secret and defines
/// which secret (version/property/..) to fetch.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExternalSecretDataRemoteRefDecodingStrategy {
    Auto,
    Base64,
    #[serde(rename = "Base64URL")]
    Base64Url,
    None,
}

/// RemoteRef points to the remote secret and defines
/// which secret (version/property/..) to fetch.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExternalSecretDataRemoteRefMetadataPolicy {
    None,
    Fetch,
}

/// SourceRef allows you to override the source
/// from which the value will be pulled.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExternalSecretDataSourceRef {
    /// GeneratorRef points to a generator custom resource.
    /// 
    /// Deprecated: The generatorRef is not implemented in .data[].
    /// this will be removed with v1.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "generatorRef")]
    pub generator_ref: Option<ExternalSecretDataSourceRefGeneratorRef>,
    /// SecretStoreRef defines which SecretStore to fetch the ExternalSecret data.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storeRef")]
    pub store_ref: Option<ExternalSecretDataSourceRefStoreRef>,
}

/// GeneratorRef points to a generator custom resource.
/// 
/// Deprecated: The generatorRef is not implemented in .data[].
/// this will be removed with v1.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ExternalSecretDataSourceRefGeneratorRef {
    /// Specify the apiVersion of the generator resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Specify the Kind of the generator resource
    pub kind: ExternalSecretDataSourceRefGeneratorRefKind,
    /// Specify the name of the generator resource
    pub name: String,
}

/// GeneratorRef points to a generator custom resource.
/// 
/// Deprecated: The generatorRef is not implemented in .data[].
/// this will be removed with v1.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExternalSecretDataSourceRefGeneratorRefKind {
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
}

/// SecretStoreRef defines which SecretStore to fetch the ExternalSecret data.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExternalSecretDataSourceRefStoreRef {
    /// Kind of the SecretStore resource (SecretStore or ClusterSecretStore)
    /// Defaults to `SecretStore`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ExternalSecretDataSourceRefStoreRefKind>,
    /// Name of the SecretStore resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// SecretStoreRef defines which SecretStore to fetch the ExternalSecret data.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExternalSecretDataSourceRefStoreRefKind {
    SecretStore,
    ClusterSecretStore,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExternalSecretDataFrom {
    /// Used to extract multiple key/value pairs from one secret
    /// Note: Extract does not support sourceRef.Generator or sourceRef.GeneratorRef.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extract: Option<ExternalSecretDataFromExtract>,
    /// Used to find secrets based on tags or regular expressions
    /// Note: Find does not support sourceRef.Generator or sourceRef.GeneratorRef.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub find: Option<ExternalSecretDataFromFind>,
    /// Used to rewrite secret Keys after getting them from the secret Provider
    /// Multiple Rewrite operations can be provided. They are applied in a layered order (first to last)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rewrite: Option<Vec<ExternalSecretDataFromRewrite>>,
    /// SourceRef points to a store or generator
    /// which contains secret values ready to use.
    /// Use this in combination with Extract or Find pull values out of
    /// a specific SecretStore.
    /// When sourceRef points to a generator Extract or Find is not supported.
    /// The generator returns a static map of values
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceRef")]
    pub source_ref: Option<ExternalSecretDataFromSourceRef>,
}

/// Used to extract multiple key/value pairs from one secret
/// Note: Extract does not support sourceRef.Generator or sourceRef.GeneratorRef.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExternalSecretDataFromExtract {
    /// Used to define a conversion Strategy
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "conversionStrategy")]
    pub conversion_strategy: Option<ExternalSecretDataFromExtractConversionStrategy>,
    /// Used to define a decoding Strategy
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "decodingStrategy")]
    pub decoding_strategy: Option<ExternalSecretDataFromExtractDecodingStrategy>,
    /// Key is the key used in the Provider, mandatory
    pub key: String,
    /// Policy for fetching tags/labels from provider secrets, possible options are Fetch, None. Defaults to None
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metadataPolicy")]
    pub metadata_policy: Option<ExternalSecretDataFromExtractMetadataPolicy>,
    /// Used to select a specific property of the Provider value (if a map), if supported
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,
    /// Used to select a specific version of the Provider value, if supported
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Used to extract multiple key/value pairs from one secret
/// Note: Extract does not support sourceRef.Generator or sourceRef.GeneratorRef.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExternalSecretDataFromExtractConversionStrategy {
    Default,
    Unicode,
}

/// Used to extract multiple key/value pairs from one secret
/// Note: Extract does not support sourceRef.Generator or sourceRef.GeneratorRef.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExternalSecretDataFromExtractDecodingStrategy {
    Auto,
    Base64,
    #[serde(rename = "Base64URL")]
    Base64Url,
    None,
}

/// Used to extract multiple key/value pairs from one secret
/// Note: Extract does not support sourceRef.Generator or sourceRef.GeneratorRef.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExternalSecretDataFromExtractMetadataPolicy {
    None,
    Fetch,
}

/// Used to find secrets based on tags or regular expressions
/// Note: Find does not support sourceRef.Generator or sourceRef.GeneratorRef.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExternalSecretDataFromFind {
    /// Used to define a conversion Strategy
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "conversionStrategy")]
    pub conversion_strategy: Option<ExternalSecretDataFromFindConversionStrategy>,
    /// Used to define a decoding Strategy
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "decodingStrategy")]
    pub decoding_strategy: Option<ExternalSecretDataFromFindDecodingStrategy>,
    /// Finds secrets based on the name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ExternalSecretDataFromFindName>,
    /// A root path to start the find operations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Find secrets based on tags.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

/// Used to find secrets based on tags or regular expressions
/// Note: Find does not support sourceRef.Generator or sourceRef.GeneratorRef.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExternalSecretDataFromFindConversionStrategy {
    Default,
    Unicode,
}

/// Used to find secrets based on tags or regular expressions
/// Note: Find does not support sourceRef.Generator or sourceRef.GeneratorRef.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExternalSecretDataFromFindDecodingStrategy {
    Auto,
    Base64,
    #[serde(rename = "Base64URL")]
    Base64Url,
    None,
}

/// Finds secrets based on the name.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExternalSecretDataFromFindName {
    /// Finds secrets base
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regexp: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExternalSecretDataFromRewrite {
    /// Used to rewrite with regular expressions.
    /// The resulting key will be the output of a regexp.ReplaceAll operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regexp: Option<ExternalSecretDataFromRewriteRegexp>,
    /// Used to apply string transformation on the secrets.
    /// The resulting key will be the output of the template applied by the operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform: Option<ExternalSecretDataFromRewriteTransform>,
}

/// Used to rewrite with regular expressions.
/// The resulting key will be the output of a regexp.ReplaceAll operation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExternalSecretDataFromRewriteRegexp {
    /// Used to define the regular expression of a re.Compiler.
    pub source: String,
    /// Used to define the target pattern of a ReplaceAll operation.
    pub target: String,
}

/// Used to apply string transformation on the secrets.
/// The resulting key will be the output of the template applied by the operation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExternalSecretDataFromRewriteTransform {
    /// Used to define the template to apply on the secret name.
    /// `.value ` will specify the secret name in the template.
    pub template: String,
}

/// SourceRef points to a store or generator
/// which contains secret values ready to use.
/// Use this in combination with Extract or Find pull values out of
/// a specific SecretStore.
/// When sourceRef points to a generator Extract or Find is not supported.
/// The generator returns a static map of values
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExternalSecretDataFromSourceRef {
    /// GeneratorRef points to a generator custom resource.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "generatorRef")]
    pub generator_ref: Option<ExternalSecretDataFromSourceRefGeneratorRef>,
    /// SecretStoreRef defines which SecretStore to fetch the ExternalSecret data.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storeRef")]
    pub store_ref: Option<ExternalSecretDataFromSourceRefStoreRef>,
}

/// GeneratorRef points to a generator custom resource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ExternalSecretDataFromSourceRefGeneratorRef {
    /// Specify the apiVersion of the generator resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Specify the Kind of the generator resource
    pub kind: ExternalSecretDataFromSourceRefGeneratorRefKind,
    /// Specify the name of the generator resource
    pub name: String,
}

/// GeneratorRef points to a generator custom resource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExternalSecretDataFromSourceRefGeneratorRefKind {
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
}

/// SecretStoreRef defines which SecretStore to fetch the ExternalSecret data.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExternalSecretDataFromSourceRefStoreRef {
    /// Kind of the SecretStore resource (SecretStore or ClusterSecretStore)
    /// Defaults to `SecretStore`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ExternalSecretDataFromSourceRefStoreRefKind>,
    /// Name of the SecretStore resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// SecretStoreRef defines which SecretStore to fetch the ExternalSecret data.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExternalSecretDataFromSourceRefStoreRefKind {
    SecretStore,
    ClusterSecretStore,
}

/// SecretStoreRef defines which SecretStore to fetch the ExternalSecret data.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExternalSecretSecretStoreRef {
    /// Kind of the SecretStore resource (SecretStore or ClusterSecretStore)
    /// Defaults to `SecretStore`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ExternalSecretSecretStoreRefKind>,
    /// Name of the SecretStore resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// SecretStoreRef defines which SecretStore to fetch the ExternalSecret data.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExternalSecretSecretStoreRefKind {
    SecretStore,
    ClusterSecretStore,
}

/// ExternalSecretTarget defines the Kubernetes Secret to be created
/// There can be only one target per ExternalSecret.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExternalSecretTarget {
    /// CreationPolicy defines rules on how to create the resulting Secret.
    /// Defaults to "Owner"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "creationPolicy")]
    pub creation_policy: Option<ExternalSecretTargetCreationPolicy>,
    /// DeletionPolicy defines rules on how to delete the resulting Secret.
    /// Defaults to "Retain"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deletionPolicy")]
    pub deletion_policy: Option<ExternalSecretTargetDeletionPolicy>,
    /// Immutable defines if the final secret will be immutable
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub immutable: Option<bool>,
    /// The name of the Secret resource to be managed.
    /// Defaults to the .metadata.name of the ExternalSecret resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Template defines a blueprint for the created Secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<ExternalSecretTargetTemplate>,
}

/// ExternalSecretTarget defines the Kubernetes Secret to be created
/// There can be only one target per ExternalSecret.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExternalSecretTargetCreationPolicy {
    Owner,
    Orphan,
    Merge,
    None,
}

/// ExternalSecretTarget defines the Kubernetes Secret to be created
/// There can be only one target per ExternalSecret.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExternalSecretTargetDeletionPolicy {
    Delete,
    Merge,
    Retain,
}

/// Template defines a blueprint for the created Secret resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExternalSecretTargetTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<BTreeMap<String, String>>,
    /// EngineVersion specifies the template engine version
    /// that should be used to compile/execute the
    /// template specified in .data and .templateFrom[].
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "engineVersion")]
    pub engine_version: Option<ExternalSecretTargetTemplateEngineVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mergePolicy")]
    pub merge_policy: Option<ExternalSecretTargetTemplateMergePolicy>,
    /// ExternalSecretTemplateMetadata defines metadata fields for the Secret blueprint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ExternalSecretTargetTemplateMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "templateFrom")]
    pub template_from: Option<Vec<ExternalSecretTargetTemplateTemplateFrom>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// Template defines a blueprint for the created Secret resource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExternalSecretTargetTemplateEngineVersion {
    #[serde(rename = "v1")]
    V1,
    #[serde(rename = "v2")]
    V2,
}

/// Template defines a blueprint for the created Secret resource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExternalSecretTargetTemplateMergePolicy {
    Replace,
    Merge,
}

/// ExternalSecretTemplateMetadata defines metadata fields for the Secret blueprint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExternalSecretTargetTemplateMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExternalSecretTargetTemplateTemplateFrom {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMap")]
    pub config_map: Option<ExternalSecretTargetTemplateTemplateFromConfigMap>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub literal: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<ExternalSecretTargetTemplateTemplateFromSecret>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<ExternalSecretTargetTemplateTemplateFromTarget>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExternalSecretTargetTemplateTemplateFromConfigMap {
    /// A list of keys in the ConfigMap/Secret to use as templates for Secret data
    pub items: Vec<ExternalSecretTargetTemplateTemplateFromConfigMapItems>,
    /// The name of the ConfigMap/Secret resource
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExternalSecretTargetTemplateTemplateFromConfigMapItems {
    /// A key in the ConfigMap/Secret
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "templateAs")]
    pub template_as: Option<ExternalSecretTargetTemplateTemplateFromConfigMapItemsTemplateAs>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExternalSecretTargetTemplateTemplateFromConfigMapItemsTemplateAs {
    Values,
    KeysAndValues,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExternalSecretTargetTemplateTemplateFromSecret {
    /// A list of keys in the ConfigMap/Secret to use as templates for Secret data
    pub items: Vec<ExternalSecretTargetTemplateTemplateFromSecretItems>,
    /// The name of the ConfigMap/Secret resource
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExternalSecretTargetTemplateTemplateFromSecretItems {
    /// A key in the ConfigMap/Secret
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "templateAs")]
    pub template_as: Option<ExternalSecretTargetTemplateTemplateFromSecretItemsTemplateAs>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExternalSecretTargetTemplateTemplateFromSecretItemsTemplateAs {
    Values,
    KeysAndValues,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExternalSecretTargetTemplateTemplateFromTarget {
    Data,
    Annotations,
    Labels,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExternalSecretStatus {
    /// Binding represents a servicebinding.io Provisioned Service reference to the secret
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding: Option<ExternalSecretStatusBinding>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// refreshTime is the time and date the external secret was fetched and
    /// the target secret updated
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refreshTime")]
    pub refresh_time: Option<String>,
    /// SyncedResourceVersion keeps track of the last synced version
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "syncedResourceVersion")]
    pub synced_resource_version: Option<String>,
}

/// Binding represents a servicebinding.io Provisioned Service reference to the secret
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExternalSecretStatusBinding {
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

