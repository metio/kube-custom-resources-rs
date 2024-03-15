// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/apecloud/kubeblocks/apps.kubeblocks.io/v1alpha1/configurations.yaml --derive=PartialEq
// kopium version: 0.17.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;

/// ConfigurationSpec defines the desired state of a Configuration resource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "apps.kubeblocks.io", version = "v1alpha1", kind = "Configuration", plural = "configurations")]
#[kube(namespaced)]
#[kube(status = "ConfigurationStatus")]
#[kube(schema = "disabled")]
pub struct ConfigurationSpec {
    /// Specifies the name of the cluster that this configuration is associated with.
    #[serde(rename = "clusterRef")]
    pub cluster_ref: String,
    /// Represents the name of the cluster component that this configuration pertains to.
    #[serde(rename = "componentName")]
    pub component_name: String,
    /// An array of ConfigurationItemDetail objects that describe user-defined configuration templates.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configItemDetails")]
    pub config_item_details: Option<Vec<ConfigurationConfigItemDetails>>,
}

/// ConfigurationItemDetail represents a specific configuration item within a configuration template.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigurationConfigItemDetails {
    /// Used to set the parameters to be updated. It is optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configFileParams")]
    pub config_file_params: Option<BTreeMap<String, ConfigurationConfigItemDetailsConfigFileParams>>,
    /// Used to set the configuration template. It is optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configSpec")]
    pub config_spec: Option<ConfigurationConfigItemDetailsConfigSpec>,
    /// Specifies the configuration template. It is optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "importTemplateRef")]
    pub import_template_ref: Option<ConfigurationConfigItemDetailsImportTemplateRef>,
    /// Defines the unique identifier of the configuration template. It must be a string of maximum 63 characters, and can only include lowercase alphanumeric characters, hyphens, and periods. The name must start and end with an alphanumeric character.
    pub name: String,
    /// Holds the configuration-related rerender. Preserves unknown fields and is optional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<BTreeMap<String, serde_json::Value>>,
    /// Deprecated: No longer used. Please use 'Payload' instead. Previously represented the version of the configuration template.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Used to set the parameters to be updated. It is optional.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigurationConfigItemDetailsConfigFileParams {
    /// Holds the configuration keys and values. This field is a workaround for issues found in kubebuilder and code-generator. Refer to https://github.com/kubernetes-sigs/kubebuilder/issues/528 and https://github.com/kubernetes/code-generator/issues/50 for more details. 
    ///  Represents the content of the configuration file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Represents the updated parameters for a single configuration file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<BTreeMap<String, String>>,
}

/// Used to set the configuration template. It is optional.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigurationConfigItemDetailsConfigSpec {
    /// An optional field where the list of containers will be injected into EnvFrom.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "asEnvFrom")]
    pub as_env_from: Option<Vec<String>>,
    /// An optional field that defines the name of the referenced configuration constraints object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "constraintRef")]
    pub constraint_ref: Option<String>,
    /// Refers to the mode bits used to set permissions on created files by default. 
    ///  Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. 
    ///  Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultMode")]
    pub default_mode: Option<i32>,
    /// Defines a list of keys. If left empty, ConfigConstraint applies to all keys in the configmap.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
    /// An optional field that defines the secondary rendered config spec.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "legacyRenderedConfigSpec")]
    pub legacy_rendered_config_spec: Option<ConfigurationConfigItemDetailsConfigSpecLegacyRenderedConfigSpec>,
    /// Specifies the name of the configuration template.
    pub name: String,
    /// Specifies the namespace of the referenced configuration template ConfigMap object. An empty namespace is equivalent to the "default" namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// An optional field defines which resources change trigger re-render config.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reRenderResourceTypes")]
    pub re_render_resource_types: Option<Vec<String>>,
    /// Specifies the name of the referenced configuration template ConfigMap object.
    #[serde(rename = "templateRef")]
    pub template_ref: String,
    /// Refers to the volume name of PodTemplate. The configuration file produced through the configuration template will be mounted to the corresponding volume. Must be a DNS_LABEL name. The volume name must be defined in podSpec.containers[*].volumeMounts.
    #[serde(rename = "volumeName")]
    pub volume_name: String,
}

/// An optional field that defines the secondary rendered config spec.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigurationConfigItemDetailsConfigSpecLegacyRenderedConfigSpec {
    /// Specifies the namespace of the referenced configuration template ConfigMap object. An empty namespace is equivalent to the "default" namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Defines the strategy for merging externally imported templates into component templates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<ConfigurationConfigItemDetailsConfigSpecLegacyRenderedConfigSpecPolicy>,
    /// Specifies the name of the referenced configuration template ConfigMap object.
    #[serde(rename = "templateRef")]
    pub template_ref: String,
}

/// An optional field that defines the secondary rendered config spec.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConfigurationConfigItemDetailsConfigSpecLegacyRenderedConfigSpecPolicy {
    #[serde(rename = "patch")]
    Patch,
    #[serde(rename = "replace")]
    Replace,
    #[serde(rename = "none")]
    None,
}

/// Specifies the configuration template. It is optional.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigurationConfigItemDetailsImportTemplateRef {
    /// Specifies the namespace of the referenced configuration template ConfigMap object. An empty namespace is equivalent to the "default" namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Defines the strategy for merging externally imported templates into component templates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<ConfigurationConfigItemDetailsImportTemplateRefPolicy>,
    /// Specifies the name of the referenced configuration template ConfigMap object.
    #[serde(rename = "templateRef")]
    pub template_ref: String,
}

/// Specifies the configuration template. It is optional.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConfigurationConfigItemDetailsImportTemplateRefPolicy {
    #[serde(rename = "patch")]
    Patch,
    #[serde(rename = "replace")]
    Replace,
    #[serde(rename = "none")]
    None,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigurationStatus {
    /// Provides detailed status information for opsRequest.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Provides the status of each component undergoing reconfiguration.
    #[serde(rename = "configurationStatus")]
    pub configuration_status: Vec<ConfigurationStatusConfigurationStatus>,
    /// Provides a description of any abnormal status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Represents the latest generation observed for this ClusterDefinition. It corresponds to the ConfigConstraint's generation, which is updated by the API Server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigurationStatusConfigurationStatus {
    /// Represents the last completed revision of the configuration item. This field is optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastDoneRevision")]
    pub last_done_revision: Option<String>,
    /// Provides a description of any abnormal status. This field is optional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Specifies the name of the configuration template. It is a required field and must be a string of maximum 63 characters. The name should only contain lowercase alphanumeric characters, hyphens, or periods. It should start and end with an alphanumeric character.
    pub name: String,
    /// Indicates the current status of the configuration item. This field is optional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<ConfigurationStatusConfigurationStatusPhase>,
    /// Provides detailed information about the execution of the configuration change. This field is optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reconcileDetail")]
    pub reconcile_detail: Option<ConfigurationStatusConfigurationStatusReconcileDetail>,
    /// Represents the updated revision of the configuration item. This field is optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updateRevision")]
    pub update_revision: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConfigurationStatusConfigurationStatusPhase {
    Creating,
    Init,
    Running,
    Pending,
    Merged,
    MergeFailed,
    FailedAndPause,
    Upgrading,
    Deleting,
    FailedAndRetry,
    Finished,
}

/// Provides detailed information about the execution of the configuration change. This field is optional.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigurationStatusConfigurationStatusReconcileDetail {
    /// Represents the current revision of the configuration item.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentRevision")]
    pub current_revision: Option<String>,
    /// Represents the error message generated when the execution of configuration changes fails.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errMessage")]
    pub err_message: Option<String>,
    /// Represents the outcome of the most recent execution.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "execResult")]
    pub exec_result: Option<String>,
    /// Represents the total number of pods that require execution of configuration changes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "expectedCount")]
    pub expected_count: Option<i32>,
    /// Represents the policy applied during the most recent execution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// Represents the number of pods where configuration changes were successfully applied.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "succeedCount")]
    pub succeed_count: Option<i32>,
}

