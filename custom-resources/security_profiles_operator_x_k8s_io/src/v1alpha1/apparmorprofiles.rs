// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubernetes-sigs/security-profiles-operator/security-profiles-operator.x-k8s.io/v1alpha1/apparmorprofiles.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// AppArmorProfileSpec defines the desired state of AppArmorProfile.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "security-profiles-operator.x-k8s.io", version = "v1alpha1", kind = "AppArmorProfile", plural = "apparmorprofiles")]
#[kube(status = "AppArmorProfileStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct AppArmorProfileSpec {
    /// Abstract stores the apparmor profile allow lists for executable, file, network and capabilities access.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "abstract")]
    pub r#abstract: Option<AppArmorProfileAbstract>,
    /// ComplainMode places the apparmor profile into "complain" mode, by default is placed in "enforce" mode.
    /// In complain mode, if a given action is not allowed, it will be allowed, but this violation will be
    /// logged with a tag of access being "ALLOWED unconfined".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "complainMode")]
    pub complain_mode: Option<bool>,
    /// Whether the profile is disabled and should be skipped during reconciliation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

/// Abstract stores the apparmor profile allow lists for executable, file, network and capabilities access.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AppArmorProfileAbstract {
    /// Capability rules for Linux capabilities.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capability: Option<AppArmorProfileAbstractCapability>,
    /// Executable rules for allowed executables.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executable: Option<AppArmorProfileAbstractExecutable>,
    /// Filesystem rules for filesystem access.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filesystem: Option<AppArmorProfileAbstractFilesystem>,
    /// Network rules for network access.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<AppArmorProfileAbstractNetwork>,
}

/// Capability rules for Linux capabilities.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AppArmorProfileAbstractCapability {
    /// AllowedCapabilities lost of allowed capabilities.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedCapabilities")]
    pub allowed_capabilities: Option<Vec<String>>,
}

/// Executable rules for allowed executables.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AppArmorProfileAbstractExecutable {
    /// AllowedExecutables list of allowed executables.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedExecutables")]
    pub allowed_executables: Option<Vec<String>>,
    /// AllowedLibraries list of allowed libraries.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedLibraries")]
    pub allowed_libraries: Option<Vec<String>>,
}

/// Filesystem rules for filesystem access.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AppArmorProfileAbstractFilesystem {
    /// ReadOnlyPaths list of allowed read only file paths.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readOnlyPaths")]
    pub read_only_paths: Option<Vec<String>>,
    /// ReadWritePaths list of allowed read write file paths.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readWritePaths")]
    pub read_write_paths: Option<Vec<String>>,
    /// WriteOnlyPaths list of allowed write only file paths.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "writeOnlyPaths")]
    pub write_only_paths: Option<Vec<String>>,
}

/// Network rules for network access.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AppArmorProfileAbstractNetwork {
    /// AllowRaw allows raw sockets.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowRaw")]
    pub allow_raw: Option<bool>,
    /// Protocols keeps the allowed networking protocols.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedProtocols")]
    pub allowed_protocols: Option<AppArmorProfileAbstractNetworkAllowedProtocols>,
}

/// Protocols keeps the allowed networking protocols.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AppArmorProfileAbstractNetworkAllowedProtocols {
    /// AllowTCP allows TCP socket connections.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowTcp")]
    pub allow_tcp: Option<bool>,
    /// AllowUDP allows UDP sockets connections.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowUdp")]
    pub allow_udp: Option<bool>,
}

/// AppArmorProfileStatus defines the observed state of AppArmorProfile.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AppArmorProfileStatus {
    /// Conditions of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// ProfileState defines the state that the profile is in. A profile in this context
    /// refers to a SeccompProfile or a SELinux profile, the states are shared between them
    /// as well as the management API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

