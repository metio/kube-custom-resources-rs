// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/tinkerbell/rufio/bmc.tinkerbell.org/v1alpha1/machines.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// MachineSpec defines desired machine state.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "bmc.tinkerbell.org", version = "v1alpha1", kind = "Machine", plural = "machines")]
#[kube(namespaced)]
#[kube(status = "MachineStatus")]
#[kube(schema = "disabled")]
pub struct MachineSpec {
    /// Connection contains connection data for a Baseboard Management Controller.
    pub connection: MachineConnection,
}

/// Connection contains connection data for a Baseboard Management Controller.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineConnection {
    /// AuthSecretRef is the SecretReference that contains authentication information of the Machine. The Secret must contain username and password keys. This is optional as it is not required when using the RPC provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authSecretRef")]
    pub auth_secret_ref: Option<MachineConnectionAuthSecretRef>,
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
    pub provider_options: Option<MachineConnectionProviderOptions>,
}

/// AuthSecretRef is the SecretReference that contains authentication information of the Machine. The Secret must contain username and password keys. This is optional as it is not required when using the RPC provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineConnectionAuthSecretRef {
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// ProviderOptions contains provider specific options.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineConnectionProviderOptions {
    /// IntelAMT contains the options to customize the IntelAMT provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "intelAMT")]
    pub intel_amt: Option<MachineConnectionProviderOptionsIntelAmt>,
    /// IPMITOOL contains the options to customize the Ipmitool provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipmitool: Option<MachineConnectionProviderOptionsIpmitool>,
    /// Redfish contains the options to customize the Redfish provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redfish: Option<MachineConnectionProviderOptionsRedfish>,
    /// RPC contains the options to customize the RPC provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rpc: Option<MachineConnectionProviderOptionsRpc>,
}

/// IntelAMT contains the options to customize the IntelAMT provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineConnectionProviderOptionsIntelAmt {
    /// Port that intelAMT will use for calls.
    pub port: i64,
}

/// IPMITOOL contains the options to customize the Ipmitool provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineConnectionProviderOptionsIpmitool {
    /// CipherSuite that ipmitool will use for calls.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cipherSuite")]
    pub cipher_suite: Option<String>,
    /// Port that ipmitool will use for calls.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}

/// Redfish contains the options to customize the Redfish provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineConnectionProviderOptionsRedfish {
    /// Port that redfish will use for calls.
    pub port: i64,
}

/// RPC contains the options to customize the RPC provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineConnectionProviderOptionsRpc {
    /// ConsumerURL is the URL where an rpc consumer/listener is running and to which we will send and receive all notifications.
    #[serde(rename = "consumerURL")]
    pub consumer_url: String,
    /// Experimental options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experimental: Option<MachineConnectionProviderOptionsRpcExperimental>,
    /// HMAC is the options used to create a HMAC signature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hmac: Option<MachineConnectionProviderOptionsRpcHmac>,
    /// LogNotificationsDisabled determines whether responses from rpc consumer/listeners will be logged or not.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logNotificationsDisabled")]
    pub log_notifications_disabled: Option<bool>,
    /// Request is the options used to create the rpc HTTP request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<MachineConnectionProviderOptionsRpcRequest>,
    /// Signature is the options used for adding an HMAC signature to an HTTP request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<MachineConnectionProviderOptionsRpcSignature>,
}

/// Experimental options.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineConnectionProviderOptionsRpcExperimental {
    /// CustomRequestPayload must be in json.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customRequestPayload")]
    pub custom_request_payload: Option<String>,
    /// DotPath is the path to the json object where the bmclib RequestPayload{} struct will be embedded. For example: object.data.body
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dotPath")]
    pub dot_path: Option<String>,
}

/// HMAC is the options used to create a HMAC signature.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineConnectionProviderOptionsRpcHmac {
    /// PrefixSigDisabled determines whether the algorithm will be prefixed to the signature. Example: sha256=abc123
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "prefixSigDisabled")]
    pub prefix_sig_disabled: Option<bool>,
    /// Secrets are a map of algorithms to secrets used for signing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<BTreeMap<String, MachineConnectionProviderOptionsRpcHmacSecrets>>,
}

/// SecretReference represents a Secret Reference. It has enough information to retrieve secret in any namespace
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineConnectionProviderOptionsRpcHmacSecrets {
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Request is the options used to create the rpc HTTP request.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineConnectionProviderOptionsRpcRequest {
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
pub struct MachineConnectionProviderOptionsRpcSignature {
    /// AppendAlgoToHeaderDisabled decides whether to append the algorithm to the signature header or not. Example: X-BMCLIB-Signature becomes X-BMCLIB-Signature-256 When set to true, a header will be added for each algorithm. Example: X-BMCLIB-Signature-256 and X-BMCLIB-Signature-512
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appendAlgoToHeaderDisabled")]
    pub append_algo_to_header_disabled: Option<bool>,
    /// HeaderName is the header name that should contain the signature(s). Example: X-BMCLIB-Signature
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "headerName")]
    pub header_name: Option<String>,
    /// IncludedPayloadHeaders are headers whose values will be included in the signature payload. Example: X-BMCLIB-My-Custom-Header All headers will be deduplicated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "includedPayloadHeaders")]
    pub included_payload_headers: Option<Vec<String>>,
}

/// MachineStatus defines the observed state of Machine.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineStatus {
    /// Conditions represents the latest available observations of an object's current state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<MachineStatusConditions>>,
    /// Power is the current power state of the Machine.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "powerState")]
    pub power_state: Option<MachineStatusPowerState>,
}

/// MachineCondition defines an observed condition of a Machine.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineStatusConditions {
    /// LastUpdateTime of the condition.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUpdateTime")]
    pub last_update_time: Option<String>,
    /// Message is a human readable message indicating with details of the last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Status of the condition.
    pub status: String,
    /// Type of the Machine condition.
    #[serde(rename = "type")]
    pub r#type: String,
}

/// MachineStatus defines the observed state of Machine.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MachineStatusPowerState {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "unknown")]
    Unknown,
}

