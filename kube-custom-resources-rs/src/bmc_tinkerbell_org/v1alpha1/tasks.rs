// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/tinkerbell/rufio/bmc.tinkerbell.org/v1alpha1/tasks.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// TaskSpec defines the desired state of Task.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "bmc.tinkerbell.org", version = "v1alpha1", kind = "Task", plural = "tasks")]
#[kube(namespaced)]
#[kube(status = "TaskStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct TaskSpec {
    /// Connection represents the Machine connectivity information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection: Option<TaskConnection>,
    /// Task defines the specific action to be performed.
    pub task: TaskTask,
}

/// Connection represents the Machine connectivity information.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TaskConnection {
    /// AuthSecretRef is the SecretReference that contains authentication information of the Machine.
    /// The Secret must contain username and password keys. This is optional as it is not required when using
    /// the RPC provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authSecretRef")]
    pub auth_secret_ref: Option<TaskConnectionAuthSecretRef>,
    /// Host is the host IP address or hostname of the Machine.
    pub host: String,
    /// InsecureTLS specifies trusted TLS connections.
    #[serde(rename = "insecureTLS")]
    pub insecure_tls: bool,
    /// Port is the port number for connecting with the Machine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// ProviderOptions contains provider specific options.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerOptions")]
    pub provider_options: Option<TaskConnectionProviderOptions>,
}

/// AuthSecretRef is the SecretReference that contains authentication information of the Machine.
/// The Secret must contain username and password keys. This is optional as it is not required when using
/// the RPC provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TaskConnectionAuthSecretRef {
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// ProviderOptions contains provider specific options.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TaskConnectionProviderOptions {
    /// IntelAMT contains the options to customize the IntelAMT provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "intelAMT")]
    pub intel_amt: Option<TaskConnectionProviderOptionsIntelAmt>,
    /// IPMITOOL contains the options to customize the Ipmitool provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipmitool: Option<TaskConnectionProviderOptionsIpmitool>,
    /// PreferredOrder allows customizing the order that BMC providers are called.
    /// Providers added to this list will be moved to the front of the default order.
    /// Provider names are case insensitive.
    /// The default order is: ipmitool, asrockrack, gofish, intelamt, dell, supermicro, openbmc.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredOrder")]
    pub preferred_order: Option<Vec<String>>,
    /// Redfish contains the options to customize the Redfish provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redfish: Option<TaskConnectionProviderOptionsRedfish>,
    /// RPC contains the options to customize the RPC provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rpc: Option<TaskConnectionProviderOptionsRpc>,
}

/// IntelAMT contains the options to customize the IntelAMT provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TaskConnectionProviderOptionsIntelAmt {
    /// HostScheme determines whether to use http or https for intelAMT calls.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostScheme")]
    pub host_scheme: Option<TaskConnectionProviderOptionsIntelAmtHostScheme>,
    /// Port that intelAMT will use for calls.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}

/// IntelAMT contains the options to customize the IntelAMT provider.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TaskConnectionProviderOptionsIntelAmtHostScheme {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    Https,
}

/// IPMITOOL contains the options to customize the Ipmitool provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TaskConnectionProviderOptionsIpmitool {
    /// CipherSuite that ipmitool will use for calls.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cipherSuite")]
    pub cipher_suite: Option<String>,
    /// Port that ipmitool will use for calls.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}

/// Redfish contains the options to customize the Redfish provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TaskConnectionProviderOptionsRedfish {
    /// Port that redfish will use for calls.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// SystemName is the name of the system to use for redfish calls.
    /// With redfish implementations that manage multiple systems via a single endpoint, this allows for specifying the system to manage.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "systemName")]
    pub system_name: Option<String>,
    /// UseBasicAuth for redfish calls. The default is false which means token based auth is used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useBasicAuth")]
    pub use_basic_auth: Option<bool>,
}

/// RPC contains the options to customize the RPC provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TaskConnectionProviderOptionsRpc {
    /// ConsumerURL is the URL where an rpc consumer/listener is running
    /// and to which we will send and receive all notifications.
    #[serde(rename = "consumerURL")]
    pub consumer_url: String,
    /// Experimental options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experimental: Option<TaskConnectionProviderOptionsRpcExperimental>,
    /// HMAC is the options used to create a HMAC signature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hmac: Option<TaskConnectionProviderOptionsRpcHmac>,
    /// LogNotificationsDisabled determines whether responses from rpc consumer/listeners will be logged or not.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logNotificationsDisabled")]
    pub log_notifications_disabled: Option<bool>,
    /// Request is the options used to create the rpc HTTP request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<TaskConnectionProviderOptionsRpcRequest>,
    /// Signature is the options used for adding an HMAC signature to an HTTP request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<TaskConnectionProviderOptionsRpcSignature>,
}

/// Experimental options.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TaskConnectionProviderOptionsRpcExperimental {
    /// CustomRequestPayload must be in json.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customRequestPayload")]
    pub custom_request_payload: Option<String>,
    /// DotPath is the path to the json object where the bmclib RequestPayload{} struct will be embedded. For example: object.data.body
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dotPath")]
    pub dot_path: Option<String>,
}

/// HMAC is the options used to create a HMAC signature.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TaskConnectionProviderOptionsRpcHmac {
    /// PrefixSigDisabled determines whether the algorithm will be prefixed to the signature. Example: sha256=abc123
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "prefixSigDisabled")]
    pub prefix_sig_disabled: Option<bool>,
    /// Secrets are a map of algorithms to secrets used for signing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<BTreeMap<String, TaskConnectionProviderOptionsRpcHmacSecrets>>,
}

/// SecretReference represents a Secret Reference. It has enough information to retrieve secret
/// in any namespace
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TaskConnectionProviderOptionsRpcHmacSecrets {
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Request is the options used to create the rpc HTTP request.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TaskConnectionProviderOptionsRpcRequest {
    /// HTTPContentType is the content type to use for the rpc request notification.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpContentType")]
    pub http_content_type: Option<String>,
    /// HTTPMethod is the HTTP method to use for the rpc request notification.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpMethod")]
    pub http_method: Option<String>,
    /// StaticHeaders are predefined headers that will be added to every request.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "staticHeaders")]
    pub static_headers: Option<BTreeMap<String, String>>,
    /// TimestampFormat is the time format for the timestamp header.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timestampFormat")]
    pub timestamp_format: Option<String>,
    /// TimestampHeader is the header name that should contain the timestamp. Example: X-BMCLIB-Timestamp
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timestampHeader")]
    pub timestamp_header: Option<String>,
}

/// Signature is the options used for adding an HMAC signature to an HTTP request.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TaskConnectionProviderOptionsRpcSignature {
    /// AppendAlgoToHeaderDisabled decides whether to append the algorithm to the signature header or not.
    /// Example: X-BMCLIB-Signature becomes X-BMCLIB-Signature-256
    /// When set to true, a header will be added for each algorithm. Example: X-BMCLIB-Signature-256 and X-BMCLIB-Signature-512
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appendAlgoToHeaderDisabled")]
    pub append_algo_to_header_disabled: Option<bool>,
    /// HeaderName is the header name that should contain the signature(s). Example: X-BMCLIB-Signature
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "headerName")]
    pub header_name: Option<String>,
    /// IncludedPayloadHeaders are headers whose values will be included in the signature payload. Example: X-BMCLIB-My-Custom-Header
    /// All headers will be deduplicated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "includedPayloadHeaders")]
    pub included_payload_headers: Option<Vec<String>>,
}

/// Task defines the specific action to be performed.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TaskTask {
    /// OneTimeBootDeviceAction represents a baseboard management one time set boot device operation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "oneTimeBootDeviceAction")]
    pub one_time_boot_device_action: Option<TaskTaskOneTimeBootDeviceAction>,
    /// PowerAction represents a baseboard management power operation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "powerAction")]
    pub power_action: Option<TaskTaskPowerAction>,
    /// VirtualMediaAction represents a baseboard management virtual media insert/eject.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "virtualMediaAction")]
    pub virtual_media_action: Option<TaskTaskVirtualMediaAction>,
}

/// OneTimeBootDeviceAction represents a baseboard management one time set boot device operation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TaskTaskOneTimeBootDeviceAction {
    /// Devices represents the boot devices, in order for setting one time boot.
    /// Currently only the first device in the slice is used to set one time boot.
    pub device: Vec<String>,
    /// EFIBoot instructs the machine to use EFI boot.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "efiBoot")]
    pub efi_boot: Option<bool>,
}

/// Task defines the specific action to be performed.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TaskTaskPowerAction {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "soft")]
    Soft,
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "cycle")]
    Cycle,
    #[serde(rename = "reset")]
    Reset,
}

/// VirtualMediaAction represents a baseboard management virtual media insert/eject.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TaskTaskVirtualMediaAction {
    pub kind: String,
    /// mediaURL represents the URL of the image to be inserted into the virtual media, or empty to
    /// eject media.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mediaURL")]
    pub media_url: Option<String>,
}

/// TaskStatus defines the observed state of Task.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TaskStatus {
    /// CompletionTime represents time when the task was completed.
    /// The completion time is only set when the task finishes successfully.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "completionTime")]
    pub completion_time: Option<String>,
    /// Conditions represents the latest available observations of an object's current state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<TaskStatusConditions>>,
    /// StartTime represents time when the Task started processing.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TaskStatusConditions {
    /// Message represents human readable message indicating details about last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Status is the status of the Task condition.
    /// Can be True or False.
    pub status: String,
    /// Type of the Task condition.
    #[serde(rename = "type")]
    pub r#type: String,
}

