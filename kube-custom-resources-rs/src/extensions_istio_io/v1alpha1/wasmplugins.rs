// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/istio/istio/extensions.istio.io/v1alpha1/wasmplugins.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// Extend the functionality provided by the Istio proxy through WebAssembly filters. See more details at: https://istio.io/docs/reference/config/proxy_extensions/wasm-plugin.html
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "extensions.istio.io", version = "v1alpha1", kind = "WasmPlugin", plural = "wasmplugins")]
#[kube(namespaced)]
#[kube(status = "WasmPluginStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct WasmPluginSpec {
    /// Specifies the failure behavior for the plugin due to fatal errors.
    /// 
    /// Valid Options: FAIL_CLOSE, FAIL_OPEN
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failStrategy")]
    pub fail_strategy: Option<WasmPluginFailStrategy>,
    /// The pull behaviour to be applied when fetching Wasm module by either OCI image or `http/https`.
    /// 
    /// Valid Options: IfNotPresent, Always
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullPolicy")]
    pub image_pull_policy: Option<WasmPluginImagePullPolicy>,
    /// Credentials to use for OCI image pulling.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullSecret")]
    pub image_pull_secret: Option<String>,
    /// Specifies the criteria to determine which traffic is passed to WasmPlugin.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<Vec<WasmPluginMatch>>,
    /// Determines where in the filter chain this `WasmPlugin` is to be injected.
    /// 
    /// Valid Options: AUTHN, AUTHZ, STATS
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<WasmPluginPhase>,
    /// The configuration that will be passed on to the plugin.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pluginConfig")]
    pub plugin_config: Option<BTreeMap<String, serde_json::Value>>,
    /// The plugin name to be used in the Envoy configuration (used to be called `rootID`).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pluginName")]
    pub plugin_name: Option<String>,
    /// Determines ordering of `WasmPlugins` in the same `phase`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// Criteria used to select the specific set of pods/VMs on which this plugin configuration should be applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<WasmPluginSelector>,
    /// SHA256 checksum that will be used to verify Wasm module or OCI container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetRef")]
    pub target_ref: Option<WasmPluginTargetRef>,
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetRefs")]
    pub target_refs: Option<Vec<WasmPluginTargetRefs>>,
    /// Specifies the type of Wasm Extension to be used.
    /// 
    /// Valid Options: HTTP, NETWORK
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<WasmPluginType>,
    /// URL of a Wasm module or OCI container.
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verificationKey")]
    pub verification_key: Option<String>,
    /// Configuration for a Wasm VM.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vmConfig")]
    pub vm_config: Option<WasmPluginVmConfig>,
}

/// Extend the functionality provided by the Istio proxy through WebAssembly filters. See more details at: https://istio.io/docs/reference/config/proxy_extensions/wasm-plugin.html
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum WasmPluginFailStrategy {
    #[serde(rename = "FAIL_CLOSE")]
    FailClose,
    #[serde(rename = "FAIL_OPEN")]
    FailOpen,
}

/// Extend the functionality provided by the Istio proxy through WebAssembly filters. See more details at: https://istio.io/docs/reference/config/proxy_extensions/wasm-plugin.html
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum WasmPluginImagePullPolicy {
    #[serde(rename = "UNSPECIFIED_POLICY")]
    UnspecifiedPolicy,
    IfNotPresent,
    Always,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WasmPluginMatch {
    /// Criteria for selecting traffic by their direction.
    /// 
    /// Valid Options: CLIENT, SERVER, CLIENT_AND_SERVER
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<WasmPluginMatchMode>,
    /// Criteria for selecting traffic by their destination port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<WasmPluginMatchPorts>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum WasmPluginMatchMode {
    #[serde(rename = "UNDEFINED")]
    Undefined,
    #[serde(rename = "CLIENT")]
    Client,
    #[serde(rename = "SERVER")]
    Server,
    #[serde(rename = "CLIENT_AND_SERVER")]
    ClientAndServer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WasmPluginMatchPorts {
    pub number: i64,
}

/// Extend the functionality provided by the Istio proxy through WebAssembly filters. See more details at: https://istio.io/docs/reference/config/proxy_extensions/wasm-plugin.html
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum WasmPluginPhase {
    #[serde(rename = "UNSPECIFIED_PHASE")]
    UnspecifiedPhase,
    #[serde(rename = "AUTHN")]
    Authn,
    #[serde(rename = "AUTHZ")]
    Authz,
    #[serde(rename = "STATS")]
    Stats,
}

/// Criteria used to select the specific set of pods/VMs on which this plugin configuration should be applied.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WasmPluginSelector {
    /// One or more labels that indicate a specific set of pods/VMs on which a policy should be applied.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WasmPluginTargetRef {
    /// group is the group of the target resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// kind is kind of the target resource.
    pub kind: String,
    /// name is the name of the target resource.
    pub name: String,
    /// namespace is the namespace of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WasmPluginTargetRefs {
    /// group is the group of the target resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// kind is kind of the target resource.
    pub kind: String,
    /// name is the name of the target resource.
    pub name: String,
    /// namespace is the namespace of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Extend the functionality provided by the Istio proxy through WebAssembly filters. See more details at: https://istio.io/docs/reference/config/proxy_extensions/wasm-plugin.html
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum WasmPluginType {
    #[serde(rename = "UNSPECIFIED_PLUGIN_TYPE")]
    UnspecifiedPluginType,
    #[serde(rename = "HTTP")]
    Http,
    #[serde(rename = "NETWORK")]
    Network,
}

/// Configuration for a Wasm VM.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WasmPluginVmConfig {
    /// Specifies environment variables to be injected to this VM.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<WasmPluginVmConfigEnv>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WasmPluginVmConfigEnv {
    /// Name of the environment variable.
    pub name: String,
    /// Value for the environment variable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Source for the environment variable's value.
    /// 
    /// Valid Options: INLINE, HOST
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<WasmPluginVmConfigEnvValueFrom>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum WasmPluginVmConfigEnvValueFrom {
    #[serde(rename = "INLINE")]
    Inline,
    #[serde(rename = "HOST")]
    Host,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WasmPluginStatus {
    /// Current service state of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Resource Generation to which the Reconciled Condition refers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<IntOrString>,
    /// Includes any errors or warnings detected by Istio's analyzers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "validationMessages")]
    pub validation_messages: Option<Vec<WasmPluginStatusValidationMessages>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WasmPluginStatusValidationMessages {
    /// A url pointing to the Istio documentation for this specific error type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "documentationUrl")]
    pub documentation_url: Option<String>,
    /// Represents how severe a message is.
    /// 
    /// Valid Options: UNKNOWN, ERROR, WARNING, INFO
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<WasmPluginStatusValidationMessagesLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<WasmPluginStatusValidationMessagesType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum WasmPluginStatusValidationMessagesLevel {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "WARNING")]
    Warning,
    #[serde(rename = "INFO")]
    Info,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WasmPluginStatusValidationMessagesType {
    /// A 7 character code matching `^IST[0-9]{4}$` intended to uniquely identify the message type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// A human-readable name for the message type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

