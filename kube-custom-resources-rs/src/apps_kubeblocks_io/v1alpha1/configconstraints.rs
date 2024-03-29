// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/apecloud/kubeblocks/apps.kubeblocks.io/v1alpha1/configconstraints.yaml --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::collections::HashMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// ConfigConstraintSpec defines the desired state of ConfigConstraint
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "apps.kubeblocks.io", version = "v1alpha1", kind = "ConfigConstraint", plural = "configconstraints")]
#[kube(status = "ConfigConstraintStatus")]
#[kube(schema = "disabled")]
pub struct ConfigConstraintSpec {
    /// Top level key used to get the cue rules to validate the config file. It must exist in 'ConfigSchema' TODO (refactored to ConfigSchemaTopLevelKey)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cfgSchemaTopLevelName")]
    pub cfg_schema_top_level_name: Option<String>,
    /// List constraints rules for each config parameters. TODO (refactored to ConfigSchema)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configurationSchema")]
    pub configuration_schema: Option<ConfigConstraintConfigurationSchema>,
    /// A set of actions for regenerating local configs. 
    ///  It works when: - different engine roles have different config, such as redis primary & secondary - after a role switch, the local config will be regenerated with the help of DownwardActions TODO (refactored to DownwardActions)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "downwardAPIOptions")]
    pub downward_api_options: Option<Vec<ConfigConstraintDownwardApiOptions>>,
    /// Indicates the dynamic reload action and restart action can be merged to a restart action. 
    ///  When a batch of parameters updates incur both restart & dynamic reload, it works as: - set to true, the two actions merged to only one restart action - set to false, the two actions cannot be merged, the actions executed in order [dynamic reload, restart]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dynamicActionCanBeMerged")]
    pub dynamic_action_can_be_merged: Option<bool>,
    /// Specifies the policy for selecting the parameters of dynamic reload actions.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dynamicParameterSelectedPolicy")]
    pub dynamic_parameter_selected_policy: Option<ConfigConstraintDynamicParameterSelectedPolicy>,
    /// A list of DynamicParameter. Modifications of dynamic parameters trigger a reload action without process restart.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dynamicParameters")]
    pub dynamic_parameters: Option<Vec<String>>,
    /// Describes the format of the config file. The controller works as follows: 1. Parse the config file 2. Get the modified parameters 3. Trigger the corresponding action
    #[serde(rename = "formatterConfig")]
    pub formatter_config: ConfigConstraintFormatterConfig,
    /// Describes parameters that are prohibited to do any modifications.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "immutableParameters")]
    pub immutable_parameters: Option<Vec<String>>,
    /// Specifies the dynamic reload actions supported by the engine. If set, the controller call the scripts defined in the actions for a dynamic parameter upgrade. The actions are called only when the modified parameter is defined in dynamicParameters part && DynamicReloadActions != nil TODO (refactored to DynamicReloadActions)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reloadOptions")]
    pub reload_options: Option<ConfigConstraintReloadOptions>,
    /// A list of ScriptConfig used by the actions defined in dynamic reload and downward actions.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scriptConfigs")]
    pub script_configs: Option<Vec<ConfigConstraintScriptConfigs>>,
    /// Used to match labels on the pod to do a dynamic reload TODO (refactored to DynamicReloadSelector)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<ConfigConstraintSelector>,
    /// A list of StaticParameter. Modifications of static parameters trigger a process restart.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "staticParameters")]
    pub static_parameters: Option<Vec<String>>,
    /// Tools used by the dynamic reload actions. Usually it is referenced by the 'init container' for 'cp' it to a binary volume. TODO (refactored to ReloadToolsImage)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "toolsImageSpec")]
    pub tools_image_spec: Option<ConfigConstraintToolsImageSpec>,
}

/// List constraints rules for each config parameters. TODO (refactored to ConfigSchema)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintConfigurationSchema {
    /// Enables providers to verify user configurations using the CUE language.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cue: Option<String>,
    /// Transforms the schema from CUE to json for further OpenAPI validation TODO (refactored to SchemaInJson)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintDownwardApiOptions {
    /// The command used to execute for the downward API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// Represents a list of downward API volume files.
    pub items: Vec<ConfigConstraintDownwardApiOptionsItems>,
    /// Specifies the mount point of the scripts file.
    #[serde(rename = "mountPoint")]
    pub mount_point: String,
    /// Specifies the name of the field. It must be a string of maximum length 63. The name should match the regex pattern `^[a-z0-9]([a-z0-9\.\-]*[a-z0-9])?$`.
    pub name: String,
}

/// DownwardAPIVolumeFile represents information to create the file containing the pod field
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintDownwardApiOptionsItems {
    /// Required: Selects a field of the pod: only annotations, labels, name and namespace are supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldRef")]
    pub field_ref: Option<ConfigConstraintDownwardApiOptionsItemsFieldRef>,
    /// Optional: mode bits used to set permissions on this file, must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
    /// Required: Path is  the relative path name of the file to be created. Must not be absolute or contain the '..' path. Must be utf-8 encoded. The first item of the relative path must not start with '..'
    pub path: String,
    /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceFieldRef")]
    pub resource_field_ref: Option<ConfigConstraintDownwardApiOptionsItemsResourceFieldRef>,
}

/// Required: Selects a field of the pod: only annotations, labels, name and namespace are supported.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintDownwardApiOptionsItemsFieldRef {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Path of the field to select in the specified API version.
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

/// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintDownwardApiOptionsItemsResourceFieldRef {
    /// Container name: required for volumes, optional for env vars
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerName")]
    pub container_name: Option<String>,
    /// Specifies the output format of the exposed resources, defaults to "1"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<IntOrString>,
    /// Required: resource to select
    pub resource: String,
}

/// ConfigConstraintSpec defines the desired state of ConfigConstraint
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConfigConstraintDynamicParameterSelectedPolicy {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "dynamic")]
    Dynamic,
}

/// Describes the format of the config file. The controller works as follows: 1. Parse the config file 2. Get the modified parameters 3. Trigger the corresponding action
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintFormatterConfig {
    /// The config file format. Valid values are `ini`, `xml`, `yaml`, `json`, `hcl`, `dotenv`, `properties` and `toml`. Each format has its own characteristics and use cases. 
    ///  - ini: is a text-based content with a structure and syntax comprising key–value pairs for properties, reference wiki: https://en.wikipedia.org/wiki/INI_file - xml: refers to wiki: https://en.wikipedia.org/wiki/XML - yaml: supports for complex data types and structures. - json: refers to wiki: https://en.wikipedia.org/wiki/JSON - hcl: The HashiCorp Configuration Language (HCL) is a configuration language authored by HashiCorp, reference url: https://www.linode.com/docs/guides/introduction-to-hcl/ - dotenv: is a plain text file with simple key–value pairs, reference wiki: https://en.wikipedia.org/wiki/Configuration_file#MS-DOS - properties: a file extension mainly used in Java, reference wiki: https://en.wikipedia.org/wiki/.properties - toml: refers to wiki: https://en.wikipedia.org/wiki/TOML - props-plus: a file extension mainly used in Java, supports CamelCase(e.g: brokerMaxConnectionsPerIp)
    pub format: ConfigConstraintFormatterConfigFormat,
    /// A pointer to an IniConfig struct that holds the ini options.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "iniConfig")]
    pub ini_config: Option<ConfigConstraintFormatterConfigIniConfig>,
}

/// Describes the format of the config file. The controller works as follows: 1. Parse the config file 2. Get the modified parameters 3. Trigger the corresponding action
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConfigConstraintFormatterConfigFormat {
    #[serde(rename = "xml")]
    Xml,
    #[serde(rename = "ini")]
    Ini,
    #[serde(rename = "yaml")]
    Yaml,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "hcl")]
    Hcl,
    #[serde(rename = "dotenv")]
    Dotenv,
    #[serde(rename = "toml")]
    Toml,
    #[serde(rename = "properties")]
    Properties,
    #[serde(rename = "redis")]
    Redis,
    #[serde(rename = "props-plus")]
    PropsPlus,
}

/// A pointer to an IniConfig struct that holds the ini options.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintFormatterConfigIniConfig {
    /// A string that describes the name of the ini section.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sectionName")]
    pub section_name: Option<String>,
}

/// Specifies the dynamic reload actions supported by the engine. If set, the controller call the scripts defined in the actions for a dynamic parameter upgrade. The actions are called only when the modified parameter is defined in dynamicParameters part && DynamicReloadActions != nil TODO (refactored to DynamicReloadActions)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintReloadOptions {
    /// Used to automatically perform the reload command when conditions are met.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoTrigger")]
    pub auto_trigger: Option<ConfigConstraintReloadOptionsAutoTrigger>,
    /// Used to perform the reload command in shell script.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "shellTrigger")]
    pub shell_trigger: Option<ConfigConstraintReloadOptionsShellTrigger>,
    /// Used to perform the reload command by Go template script.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tplScriptTrigger")]
    pub tpl_script_trigger: Option<ConfigConstraintReloadOptionsTplScriptTrigger>,
    /// Used to trigger a reload by sending a Unix signal to the process.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unixSignalTrigger")]
    pub unix_signal_trigger: Option<ConfigConstraintReloadOptionsUnixSignalTrigger>,
}

/// Used to automatically perform the reload command when conditions are met.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintReloadOptionsAutoTrigger {
    /// The name of the process.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "processName")]
    pub process_name: Option<String>,
}

/// Used to perform the reload command in shell script.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintReloadOptionsShellTrigger {
    /// Specifies the list of commands for reload.
    pub command: Vec<String>,
    /// Specifies whether to synchronize updates parameters to the config manager. Specifies two ways of controller to reload the parameter: - set to 'True', execute the reload action in sync mode, wait for the completion of reload - set to 'False', execute the reload action in async mode, just update the 'Configmap', no need to wait
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync: Option<bool>,
}

/// Used to perform the reload command by Go template script.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintReloadOptionsTplScriptTrigger {
    /// Specifies the namespace where the referenced tpl script ConfigMap in. If left empty, by default in the "default" namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specifies the reference to the ConfigMap that contains the script to be executed for reload.
    #[serde(rename = "scriptConfigMapRef")]
    pub script_config_map_ref: String,
    /// Specifies whether to synchronize updates parameters to the config manager. Specifies two ways of controller to reload the parameter: - set to 'True', execute the reload action in sync mode, wait for the completion of reload - set to 'False', execute the reload action in async mode, just update the 'Configmap', no need to wait
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync: Option<bool>,
}

/// Used to trigger a reload by sending a Unix signal to the process.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintReloadOptionsUnixSignalTrigger {
    /// Represents the name of the process that the Unix signal sent to.
    #[serde(rename = "processName")]
    pub process_name: String,
    /// Represents a valid Unix signal. Refer to the following URL for a list of all Unix signals: ../../pkg/configuration/configmap/handler.go:allUnixSignals
    pub signal: ConfigConstraintReloadOptionsUnixSignalTriggerSignal,
}

/// Used to trigger a reload by sending a Unix signal to the process.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConfigConstraintReloadOptionsUnixSignalTriggerSignal {
    #[serde(rename = "SIGHUP")]
    Sighup,
    #[serde(rename = "SIGINT")]
    Sigint,
    #[serde(rename = "SIGQUIT")]
    Sigquit,
    #[serde(rename = "SIGILL")]
    Sigill,
    #[serde(rename = "SIGTRAP")]
    Sigtrap,
    #[serde(rename = "SIGABRT")]
    Sigabrt,
    #[serde(rename = "SIGBUS")]
    Sigbus,
    #[serde(rename = "SIGFPE")]
    Sigfpe,
    #[serde(rename = "SIGKILL")]
    Sigkill,
    #[serde(rename = "SIGUSR1")]
    Sigusr1,
    #[serde(rename = "SIGSEGV")]
    Sigsegv,
    #[serde(rename = "SIGUSR2")]
    Sigusr2,
    #[serde(rename = "SIGPIPE")]
    Sigpipe,
    #[serde(rename = "SIGALRM")]
    Sigalrm,
    #[serde(rename = "SIGTERM")]
    Sigterm,
    #[serde(rename = "SIGSTKFLT")]
    Sigstkflt,
    #[serde(rename = "SIGCHLD")]
    Sigchld,
    #[serde(rename = "SIGCONT")]
    Sigcont,
    #[serde(rename = "SIGSTOP")]
    Sigstop,
    #[serde(rename = "SIGTSTP")]
    Sigtstp,
    #[serde(rename = "SIGTTIN")]
    Sigttin,
    #[serde(rename = "SIGTTOU")]
    Sigttou,
    #[serde(rename = "SIGURG")]
    Sigurg,
    #[serde(rename = "SIGXCPU")]
    Sigxcpu,
    #[serde(rename = "SIGXFSZ")]
    Sigxfsz,
    #[serde(rename = "SIGVTALRM")]
    Sigvtalrm,
    #[serde(rename = "SIGPROF")]
    Sigprof,
    #[serde(rename = "SIGWINCH")]
    Sigwinch,
    #[serde(rename = "SIGIO")]
    Sigio,
    #[serde(rename = "SIGPWR")]
    Sigpwr,
    #[serde(rename = "SIGSYS")]
    Sigsys,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintScriptConfigs {
    /// Specifies the namespace where the referenced tpl script ConfigMap in. If left empty, by default in the "default" namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specifies the reference to the ConfigMap that contains the script to be executed for reload.
    #[serde(rename = "scriptConfigMapRef")]
    pub script_config_map_ref: String,
}

/// Used to match labels on the pod to do a dynamic reload TODO (refactored to DynamicReloadSelector)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ConfigConstraintSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Tools used by the dynamic reload actions. Usually it is referenced by the 'init container' for 'cp' it to a binary volume. TODO (refactored to ReloadToolsImage)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintToolsImageSpec {
    /// Represents the point where the scripts file will be mounted.
    #[serde(rename = "mountPoint")]
    pub mount_point: String,
    /// Used to configure the initialization container.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "toolConfigs")]
    pub tool_configs: Option<Vec<ConfigConstraintToolsImageSpecToolConfigs>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintToolsImageSpecToolConfigs {
    /// Commands to be executed when init containers.
    pub command: Vec<String>,
    /// Represents the url of the tool container image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Specifies the name of the initContainer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintStatus {
    /// Provides descriptions for abnormal states.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Refers to the most recent generation observed for this ConfigConstraint. This value is updated by the API Server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// Specifies the status of the configuration template. When set to CCAvailablePhase, the ConfigConstraint can be referenced by ClusterDefinition or ClusterVersion.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<ConfigConstraintStatusPhase>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConfigConstraintStatusPhase {
    Available,
    Unavailable,
    Deleting,
}

