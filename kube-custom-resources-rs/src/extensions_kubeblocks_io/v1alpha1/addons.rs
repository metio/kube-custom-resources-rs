// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/apecloud/kubeblocks/extensions.kubeblocks.io/v1alpha1/addons.yaml --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// AddonSpec defines the desired state of an add-on.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "extensions.kubeblocks.io", version = "v1alpha1", kind = "Addon", plural = "addons")]
#[kube(status = "AddonStatus")]
#[kube(schema = "disabled")]
pub struct AddonSpec {
    /// Plugin installation spec.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cliPlugins")]
    pub cli_plugins: Option<Vec<AddonCliPlugins>>,
    /// Default installation parameters.
    #[serde(rename = "defaultInstallValues")]
    pub default_install_values: Vec<AddonDefaultInstallValues>,
    /// Addon description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Helm installation spec. It's processed only when type=helm.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub helm: Option<AddonHelm>,
    /// Installation parameters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub install: Option<AddonInstall>,
    /// Addon installable spec. It provides selector and auto-install settings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installable: Option<AddonInstallable>,
    /// Add-on type. The valid value is helm.
    #[serde(rename = "type")]
    pub r#type: AddonType,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonCliPlugins {
    /// The description of the plugin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The index repository of the plugin.
    #[serde(rename = "indexRepository")]
    pub index_repository: String,
    /// Name of the plugin.
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonDefaultInstallValues {
    /// enabled can be set if there are no specific installation attributes to be set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Installs spec. for extra items.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<Vec<AddonDefaultInstallValuesExtras>>,
    /// Persistent Volume Enabled value.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "persistentVolumeEnabled")]
    pub persistent_volume_enabled: Option<bool>,
    /// Replicas value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// Resource requirements.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<AddonDefaultInstallValuesResources>,
    /// Addon installs parameters selectors by default. If multiple selectors are provided, all selectors must evaluate to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selectors: Option<Vec<AddonDefaultInstallValuesSelectors>>,
    /// Storage class name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageClass")]
    pub storage_class: Option<String>,
    /// Tolerations JSON array string value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonDefaultInstallValuesExtras {
    /// Name of the item.
    pub name: String,
    /// Persistent Volume Enabled value.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "persistentVolumeEnabled")]
    pub persistent_volume_enabled: Option<bool>,
    /// Replicas value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// Resource requirements.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<AddonDefaultInstallValuesExtrasResources>,
    /// Storage class name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageClass")]
    pub storage_class: Option<String>,
    /// Tolerations JSON array string value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<String>,
}

/// Resource requirements.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonDefaultInstallValuesExtrasResources {
    /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified; otherwise, it defaults to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// Resource requirements.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonDefaultInstallValuesResources {
    /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified; otherwise, it defaults to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonDefaultInstallValuesSelectors {
    /// The selector key. Valid values are KubeVersion, KubeGitVersion and KubeProvider. "KubeVersion" the semver expression of Kubernetes versions, i.e., v1.24. "KubeGitVersion" may contain distro. info., i.e., v1.24.4+eks. "KubeProvider" the Kubernetes provider, i.e., aws, gcp, azure, huaweiCloud, tencentCloud etc.
    pub key: AddonDefaultInstallValuesSelectorsKey,
    /// Represents a key's relationship to a set of values. Valid operators are Contains, NotIn, DoesNotContain, MatchRegex, and DoesNoteMatchRegex. 
    ///  Possible enum values: `"Contains"` line contains a string. `"DoesNotContain"` line does not contain a string. `"MatchRegex"` line contains a match to the regular expression. `"DoesNotMatchRegex"` line does not contain a match to the regular expression.
    pub operator: AddonDefaultInstallValuesSelectorsOperator,
    /// An array of string values. It serves as an "OR" expression to the operator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AddonDefaultInstallValuesSelectorsKey {
    KubeGitVersion,
    KubeVersion,
    KubeProvider,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AddonDefaultInstallValuesSelectorsOperator {
    Contains,
    DoesNotContain,
    MatchRegex,
    DoesNotMatchRegex,
}

/// Helm installation spec. It's processed only when type=helm.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonHelm {
    /// A Helm Chart location URL.
    #[serde(rename = "chartLocationURL")]
    pub chart_location_url: String,
    /// chartsImage defines the image of Helm charts.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "chartsImage")]
    pub charts_image: Option<String>,
    /// chartsPathInImage defines the path of Helm charts in the image. It's used to copy Helm charts from the image to the shared volume.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "chartsPathInImage")]
    pub charts_path_in_image: Option<String>,
    /// installOptions defines Helm release installation options.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "installOptions")]
    pub install_options: Option<BTreeMap<String, String>>,
    /// HelmInstallValues defines Helm release installation set values.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "installValues")]
    pub install_values: Option<AddonHelmInstallValues>,
    /// valuesMapping defines add-on normalized resources parameters mapped to Helm values' keys.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valuesMapping")]
    pub values_mapping: Option<AddonHelmValuesMapping>,
}

/// HelmInstallValues defines Helm release installation set values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonHelmInstallValues {
    /// Selects a key of a ConfigMap item list. The value of ConfigMap can be a JSON or YAML string content. Use a key name with ".json" or ".yaml" or ".yml" extension name to specify a content type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapRefs")]
    pub config_map_refs: Option<Vec<AddonHelmInstallValuesConfigMapRefs>>,
    /// Selects a key of a Secrets item list. The value of Secrets can be a JSON or YAML string content. Use a key name with ".json" or ".yaml" or ".yml" extension name to specify a content type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRefs")]
    pub secret_refs: Option<Vec<AddonHelmInstallValuesSecretRefs>>,
    /// Helm install set JSON values. It can specify multiple or separate values with commas(key1=jsonval1,key2=jsonval2).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "setJSONValues")]
    pub set_json_values: Option<Vec<String>>,
    /// Helm install set values. It can specify multiple or separate values with commas(key1=val1,key2=val2).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "setValues")]
    pub set_values: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonHelmInstallValuesConfigMapRefs {
    /// The key to select.
    pub key: String,
    /// Object name of the referent.
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonHelmInstallValuesSecretRefs {
    /// The key to select.
    pub key: String,
    /// Object name of the referent.
    pub name: String,
}

/// valuesMapping defines add-on normalized resources parameters mapped to Helm values' keys.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonHelmValuesMapping {
    /// Helm value mapping items for extra items.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<Vec<AddonHelmValuesMappingExtras>>,
    /// jsonMap defines the "key" mapping values. The valid key is tolerations. Enum values explained: `"tolerations"` sets the toleration mapping key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jsonMap")]
    pub json_map: Option<AddonHelmValuesMappingJsonMap>,
    /// resources sets resources related mapping keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<AddonHelmValuesMappingResources>,
    /// valueMap define the "key" mapping values. Valid keys are replicaCount, persistentVolumeEnabled, and storageClass. Enum values explained: `"replicaCount"` sets the replicaCount value mapping key. `"persistentVolumeEnabled"` sets the persistent volume enabled mapping key. `"storageClass"` sets the storageClass mapping key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueMap")]
    pub value_map: Option<AddonHelmValuesMappingValueMap>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonHelmValuesMappingExtras {
    /// jsonMap defines the "key" mapping values. The valid key is tolerations. Enum values explained: `"tolerations"` sets the toleration mapping key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jsonMap")]
    pub json_map: Option<AddonHelmValuesMappingExtrasJsonMap>,
    /// Name of the item.
    pub name: String,
    /// resources sets resources related mapping keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<AddonHelmValuesMappingExtrasResources>,
    /// valueMap define the "key" mapping values. Valid keys are replicaCount, persistentVolumeEnabled, and storageClass. Enum values explained: `"replicaCount"` sets the replicaCount value mapping key. `"persistentVolumeEnabled"` sets the persistent volume enabled mapping key. `"storageClass"` sets the storageClass mapping key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueMap")]
    pub value_map: Option<AddonHelmValuesMappingExtrasValueMap>,
}

/// jsonMap defines the "key" mapping values. The valid key is tolerations. Enum values explained: `"tolerations"` sets the toleration mapping key.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonHelmValuesMappingExtrasJsonMap {
    /// tolerations sets the toleration mapping key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<String>,
}

/// resources sets resources related mapping keys.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonHelmValuesMappingExtrasResources {
    /// cpu sets CPU requests and limits mapping keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<AddonHelmValuesMappingExtrasResourcesCpu>,
    /// memory sets Memory requests and limits mapping keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<AddonHelmValuesMappingExtrasResourcesMemory>,
    /// storage sets the storage size value mapping key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
}

/// cpu sets CPU requests and limits mapping keys.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonHelmValuesMappingExtrasResourcesCpu {
    /// Limits value mapping key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<String>,
    /// Requests value mapping key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<String>,
}

/// memory sets Memory requests and limits mapping keys.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonHelmValuesMappingExtrasResourcesMemory {
    /// Limits value mapping key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<String>,
    /// Requests value mapping key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<String>,
}

/// valueMap define the "key" mapping values. Valid keys are replicaCount, persistentVolumeEnabled, and storageClass. Enum values explained: `"replicaCount"` sets the replicaCount value mapping key. `"persistentVolumeEnabled"` sets the persistent volume enabled mapping key. `"storageClass"` sets the storageClass mapping key.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonHelmValuesMappingExtrasValueMap {
    /// persistentVolumeEnabled sets the persistent volume enabled mapping key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "persistentVolumeEnabled")]
    pub persistent_volume_enabled: Option<String>,
    /// replicaCount sets the replicaCount value mapping key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaCount")]
    pub replica_count: Option<String>,
    /// storageClass sets the storageClass mapping key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageClass")]
    pub storage_class: Option<String>,
}

/// jsonMap defines the "key" mapping values. The valid key is tolerations. Enum values explained: `"tolerations"` sets the toleration mapping key.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonHelmValuesMappingJsonMap {
    /// tolerations sets the toleration mapping key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<String>,
}

/// resources sets resources related mapping keys.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonHelmValuesMappingResources {
    /// cpu sets CPU requests and limits mapping keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<AddonHelmValuesMappingResourcesCpu>,
    /// memory sets Memory requests and limits mapping keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<AddonHelmValuesMappingResourcesMemory>,
    /// storage sets the storage size value mapping key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
}

/// cpu sets CPU requests and limits mapping keys.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonHelmValuesMappingResourcesCpu {
    /// Limits value mapping key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<String>,
    /// Requests value mapping key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<String>,
}

/// memory sets Memory requests and limits mapping keys.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonHelmValuesMappingResourcesMemory {
    /// Limits value mapping key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<String>,
    /// Requests value mapping key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<String>,
}

/// valueMap define the "key" mapping values. Valid keys are replicaCount, persistentVolumeEnabled, and storageClass. Enum values explained: `"replicaCount"` sets the replicaCount value mapping key. `"persistentVolumeEnabled"` sets the persistent volume enabled mapping key. `"storageClass"` sets the storageClass mapping key.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonHelmValuesMappingValueMap {
    /// persistentVolumeEnabled sets the persistent volume enabled mapping key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "persistentVolumeEnabled")]
    pub persistent_volume_enabled: Option<String>,
    /// replicaCount sets the replicaCount value mapping key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaCount")]
    pub replica_count: Option<String>,
    /// storageClass sets the storageClass mapping key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageClass")]
    pub storage_class: Option<String>,
}

/// Installation parameters.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonInstall {
    /// enabled can be set if there are no specific installation attributes to be set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Installs spec. for extra items.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<Vec<AddonInstallExtras>>,
    /// Persistent Volume Enabled value.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "persistentVolumeEnabled")]
    pub persistent_volume_enabled: Option<bool>,
    /// Replicas value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// Resource requirements.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<AddonInstallResources>,
    /// Storage class name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageClass")]
    pub storage_class: Option<String>,
    /// Tolerations JSON array string value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonInstallExtras {
    /// Name of the item.
    pub name: String,
    /// Persistent Volume Enabled value.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "persistentVolumeEnabled")]
    pub persistent_volume_enabled: Option<bool>,
    /// Replicas value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// Resource requirements.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<AddonInstallExtrasResources>,
    /// Storage class name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageClass")]
    pub storage_class: Option<String>,
    /// Tolerations JSON array string value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<String>,
}

/// Resource requirements.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonInstallExtrasResources {
    /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified; otherwise, it defaults to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// Resource requirements.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonInstallResources {
    /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified; otherwise, it defaults to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// Addon installable spec. It provides selector and auto-install settings.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonInstallable {
    /// autoInstall defines an add-on should be installed automatically.
    #[serde(rename = "autoInstall")]
    pub auto_install: bool,
    /// Add-on installable selectors. If multiple selectors are provided, all selectors must evaluate to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selectors: Option<Vec<AddonInstallableSelectors>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonInstallableSelectors {
    /// The selector key. Valid values are KubeVersion, KubeGitVersion and KubeProvider. "KubeVersion" the semver expression of Kubernetes versions, i.e., v1.24. "KubeGitVersion" may contain distro. info., i.e., v1.24.4+eks. "KubeProvider" the Kubernetes provider, i.e., aws, gcp, azure, huaweiCloud, tencentCloud etc.
    pub key: AddonInstallableSelectorsKey,
    /// Represents a key's relationship to a set of values. Valid operators are Contains, NotIn, DoesNotContain, MatchRegex, and DoesNoteMatchRegex. 
    ///  Possible enum values: `"Contains"` line contains a string. `"DoesNotContain"` line does not contain a string. `"MatchRegex"` line contains a match to the regular expression. `"DoesNotMatchRegex"` line does not contain a match to the regular expression.
    pub operator: AddonInstallableSelectorsOperator,
    /// An array of string values. It serves as an "OR" expression to the operator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AddonInstallableSelectorsKey {
    KubeGitVersion,
    KubeVersion,
    KubeProvider,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AddonInstallableSelectorsOperator {
    Contains,
    DoesNotContain,
    MatchRegex,
    DoesNotMatchRegex,
}

/// AddonSpec defines the desired state of an add-on.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AddonType {
    Helm,
}

/// AddonStatus defines the observed state of an add-on.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonStatus {
    /// Describes the current state of add-on API installation conditions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<AddonStatusConditions>>,
    /// observedGeneration is the most recent generation observed for this add-on. It corresponds to the add-on's generation, which is updated on mutation by the API Server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// Add-on installation phases. Valid values are Disabled, Enabled, Failed, Enabling, Disabling.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<AddonStatusPhase>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddonStatusConditions {
    /// lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// message is a human readable message indicating details about the transition. This may be an empty string.
    pub message: String,
    /// observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
    pub reason: String,
    /// status of the condition, one of True, False, Unknown.
    pub status: AddonStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AddonStatusConditionsStatus {
    True,
    False,
    Unknown,
}

/// AddonStatus defines the observed state of an add-on.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AddonStatusPhase {
    Disabled,
    Enabled,
    Failed,
    Enabling,
    Disabling,
}

