// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/istio/istio/security.istio.io/v1beta1/peerauthentications.yaml
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

/// Peer authentication configuration for workloads. See more details at: https://istio.io/docs/reference/config/security/peer_authentication.html
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "security.istio.io", version = "v1beta1", kind = "PeerAuthentication", plural = "peerauthentications")]
#[kube(namespaced)]
#[kube(status = "PeerAuthenticationStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct PeerAuthenticationSpec {
    /// Mutual TLS settings for workload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtls: Option<PeerAuthenticationMtls>,
    /// Port specific mutual TLS settings.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portLevelMtls")]
    pub port_level_mtls: Option<BTreeMap<String, PeerAuthenticationPortLevelMtls>>,
    /// The selector determines the workloads to apply the PeerAuthentication on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<PeerAuthenticationSelector>,
}

/// Mutual TLS settings for workload.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PeerAuthenticationMtls {
    /// Defines the mTLS mode used for peer authentication.
    /// 
    /// Valid Options: DISABLE, PERMISSIVE, STRICT
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<PeerAuthenticationMtlsMode>,
}

/// Mutual TLS settings for workload.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PeerAuthenticationMtlsMode {
    #[serde(rename = "UNSET")]
    Unset,
    #[serde(rename = "DISABLE")]
    Disable,
    #[serde(rename = "PERMISSIVE")]
    Permissive,
    #[serde(rename = "STRICT")]
    Strict,
}

/// Port specific mutual TLS settings.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PeerAuthenticationPortLevelMtls {
    /// Defines the mTLS mode used for peer authentication.
    /// 
    /// Valid Options: DISABLE, PERMISSIVE, STRICT
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<PeerAuthenticationPortLevelMtlsMode>,
}

/// Port specific mutual TLS settings.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PeerAuthenticationPortLevelMtlsMode {
    #[serde(rename = "UNSET")]
    Unset,
    #[serde(rename = "DISABLE")]
    Disable,
    #[serde(rename = "PERMISSIVE")]
    Permissive,
    #[serde(rename = "STRICT")]
    Strict,
}

/// The selector determines the workloads to apply the PeerAuthentication on.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PeerAuthenticationSelector {
    /// One or more labels that indicate a specific set of pods/VMs on which a policy should be applied.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PeerAuthenticationStatus {
    /// Current service state of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<IntOrString>,
    /// Includes any errors or warnings detected by Istio's analyzers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "validationMessages")]
    pub validation_messages: Option<Vec<PeerAuthenticationStatusValidationMessages>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PeerAuthenticationStatusValidationMessages {
    /// A url pointing to the Istio documentation for this specific error type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "documentationUrl")]
    pub documentation_url: Option<String>,
    /// Represents how severe a message is.
    /// 
    /// Valid Options: UNKNOWN, ERROR, WARNING, INFO
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<PeerAuthenticationStatusValidationMessagesLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<PeerAuthenticationStatusValidationMessagesType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PeerAuthenticationStatusValidationMessagesLevel {
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
pub struct PeerAuthenticationStatusValidationMessagesType {
    /// A 7 character code matching `^IST[0-9]{4}$` intended to uniquely identify the message type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// A human-readable name for the message type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

