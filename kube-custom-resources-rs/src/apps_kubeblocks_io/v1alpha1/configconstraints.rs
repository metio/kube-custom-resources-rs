// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/apecloud/kubeblocks/apps.kubeblocks.io/v1alpha1/configconstraints.yaml --derive=PartialEq
// kopium version: 0.16.2

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
    /// cfgSchemaTopLevelName is cue type name, which generates openapi schema.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cfgSchemaTopLevelName")]
    pub cfg_schema_top_level_name: Option<String>,
    /// configurationSchema imposes restrictions on database parameter's rule.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configurationSchema")]
    pub configuration_schema: Option<ConfigConstraintConfigurationSchema>,
    /// downwardAPIOptions is used to watch pod fields.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "downwardAPIOptions")]
    pub downward_api_options: Option<Vec<ConfigConstraintDownwardApiOptions>>,
    /// dynamicParameters, list of DynamicParameter, modifications of them trigger a config dynamic reload without process restart.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dynamicParameters")]
    pub dynamic_parameters: Option<Vec<String>>,
    /// formatterConfig describes the format of the configuration file, the controller 1. parses configuration file 2. analyzes the modified parameters 3. applies corresponding policies.
    #[serde(rename = "formatterConfig")]
    pub formatter_config: ConfigConstraintFormatterConfig,
    /// immutableParameters describes parameters that prohibit user from modification.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "immutableParameters")]
    pub immutable_parameters: Option<Vec<String>>,
    /// reloadOptions indicates whether the process supports reload. if set, the controller will determine the behavior of the engine instance based on the configuration templates, restart or reload depending on whether any parameters in the StaticParameters have been modified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reloadOptions")]
    pub reload_options: Option<ConfigConstraintReloadOptions>,
    /// scriptConfigs, list of ScriptConfig, witch these scripts can be used by volume trigger,downward trigger, or tool image
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scriptConfigs")]
    pub script_configs: Option<Vec<ConfigConstraintScriptConfigs>>,
    /// selector is used to match the label on the pod, for example, a pod of the primary is match on the patroni cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<ConfigConstraintSelector>,
    /// staticParameters, list of StaticParameter, modifications of them trigger a process restart.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "staticParameters")]
    pub static_parameters: Option<Vec<String>>,
    /// toolConfig used to config init container.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "toolsImageSpec")]
    pub tools_image_spec: Option<ConfigConstraintToolsImageSpec>,
}

/// configurationSchema imposes restrictions on database parameter's rule.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintConfigurationSchema {
    /// cue that to let provider verify user configuration through cue language.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cue: Option<String>,
    /// schema provides a way for providers to validate the changed parameters through json.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintDownwardApiOptions {
    /// command used to execute for downwrad api.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// Items is a list of downward API volume file
    pub items: Vec<ConfigConstraintDownwardApiOptionsItems>,
    /// mountPoint is the mount point of the scripts file.
    #[serde(rename = "mountPoint")]
    pub mount_point: String,
    /// Specify the name of the field.
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

/// formatterConfig describes the format of the configuration file, the controller 1. parses configuration file 2. analyzes the modified parameters 3. applies corresponding policies.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintFormatterConfig {
    /// The configuration file format. Valid values are ini, xml, yaml, json, hcl, dotenv, properties and toml. 
    ///  ini: a configuration file that consists of a text-based content with a structure and syntax comprising key–value pairs for properties, reference wiki: https://en.wikipedia.org/wiki/INI_file xml: reference wiki: https://en.wikipedia.org/wiki/XML yaml: a configuration file support for complex data types and structures. json: reference wiki: https://en.wikipedia.org/wiki/JSON hcl: : The HashiCorp Configuration Language (HCL) is a configuration language authored by HashiCorp, reference url: https://www.linode.com/docs/guides/introduction-to-hcl/ dotenv: this was a plain text file with simple key–value pairs, reference wiki: https://en.wikipedia.org/wiki/Configuration_file#MS-DOS properties: a file extension mainly used in Java, reference wiki: https://en.wikipedia.org/wiki/.properties toml: reference wiki: https://en.wikipedia.org/wiki/TOML props-plus: a file extension mainly used in Java, support CamelCase(e.g: brokerMaxConnectionsPerIp)
    pub format: ConfigConstraintFormatterConfigFormat,
    /// iniConfig represents the ini options.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "iniConfig")]
    pub ini_config: Option<ConfigConstraintFormatterConfigIniConfig>,
}

/// formatterConfig describes the format of the configuration file, the controller 1. parses configuration file 2. analyzes the modified parameters 3. applies corresponding policies.
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

/// iniConfig represents the ini options.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintFormatterConfigIniConfig {
    /// sectionName describes ini section.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sectionName")]
    pub section_name: Option<String>,
}

/// reloadOptions indicates whether the process supports reload. if set, the controller will determine the behavior of the engine instance based on the configuration templates, restart or reload depending on whether any parameters in the StaticParameters have been modified.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintReloadOptions {
    /// shellTrigger performs the reload command.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "shellTrigger")]
    pub shell_trigger: Option<ConfigConstraintReloadOptionsShellTrigger>,
    /// goTplTrigger performs the reload command.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tplScriptTrigger")]
    pub tpl_script_trigger: Option<ConfigConstraintReloadOptionsTplScriptTrigger>,
    /// unixSignalTrigger used to reload by sending a signal.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unixSignalTrigger")]
    pub unix_signal_trigger: Option<ConfigConstraintReloadOptionsUnixSignalTrigger>,
}

/// shellTrigger performs the reload command.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintReloadOptionsShellTrigger {
    /// command used to execute for reload.
    pub command: Vec<String>,
    /// Specify synchronize updates parameters to the config manager.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync: Option<bool>,
}

/// goTplTrigger performs the reload command.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintReloadOptionsTplScriptTrigger {
    /// Specify the namespace of the referenced the tpl script ConfigMap object. An empty namespace is equivalent to the "default" namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// scriptConfigMapRef used to execute for reload.
    #[serde(rename = "scriptConfigMapRef")]
    pub script_config_map_ref: String,
    /// Specify synchronize updates parameters to the config manager.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync: Option<bool>,
}

/// unixSignalTrigger used to reload by sending a signal.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintReloadOptionsUnixSignalTrigger {
    /// processName is process name, sends unix signal to proc.
    #[serde(rename = "processName")]
    pub process_name: String,
    /// signal is valid for unix signal. e.g: SIGHUP url: ../../pkg/configuration/configmap/handler.go:allUnixSignals
    pub signal: ConfigConstraintReloadOptionsUnixSignalTriggerSignal,
}

/// unixSignalTrigger used to reload by sending a signal.
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
    /// Specify the namespace of the referenced the tpl script ConfigMap object. An empty namespace is equivalent to the "default" namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// scriptConfigMapRef used to execute for reload.
    #[serde(rename = "scriptConfigMapRef")]
    pub script_config_map_ref: String,
}

/// selector is used to match the label on the pod, for example, a pod of the primary is match on the patroni cluster.
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

/// toolConfig used to config init container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintToolsImageSpec {
    /// mountPoint is the mount point of the scripts file.
    #[serde(rename = "mountPoint")]
    pub mount_point: String,
    /// toolConfig used to config init container.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "toolConfigs")]
    pub tool_configs: Option<Vec<ConfigConstraintToolsImageSpecToolConfigs>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintToolsImageSpecToolConfigs {
    /// exec used to execute for init containers.
    pub command: Vec<String>,
    /// tools Container image name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Specify the name of initContainer. Must be a DNS_LABEL name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// ConfigConstraintStatus defines the observed state of ConfigConstraint.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintStatus {
    /// message field describes the reasons of abnormal status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// observedGeneration is the latest generation observed for this ClusterDefinition. It refers to the ConfigConstraint's generation, which is updated by the API Server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// phase is status of configuration template, when set to CCAvailablePhase, it can be referenced by ClusterDefinition or ClusterVersion.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<ConfigConstraintStatusPhase>,
}

/// ConfigConstraintStatus defines the observed state of ConfigConstraint.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConfigConstraintStatusPhase {
    Available,
    Unavailable,
    Deleting,
}
