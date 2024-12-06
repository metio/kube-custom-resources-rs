// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/gpu-ninja/dex-operator/dex.gpu-ninja.com/v1alpha1/dexusers.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// DexUserSpec defines the desired state of the user.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "dex.gpu-ninja.com", version = "v1alpha1", kind = "DexUser", plural = "dexusers")]
#[kube(namespaced)]
#[kube(status = "DexUserStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DexUserSpec {
    /// Email and identifying name of the password. Emails are assumed to be valid and determining that an end-user controls the address is left to an outside application.
    pub email: String,
    /// IdentityProviderRef is a reference to the identity provider which this user is associated with.
    #[serde(rename = "identityProviderRef")]
    pub identity_provider_ref: DexUserIdentityProviderRef,
    /// SecretName is the name of the secret that will be created to store the generated user password.
    #[serde(rename = "secretName")]
    pub secret_name: String,
}

/// IdentityProviderRef is a reference to the identity provider which this user is associated with.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DexUserIdentityProviderRef {
    /// Name of the referenced DexIdentityProvider.
    pub name: String,
}

/// DexUserStatus defines the observed state of the user.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DexUserStatus {
    /// ObservedGeneration is the most recent generation observed for this user by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// Phase is the current phase of the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// Reason is a human readable message indicating details about why the user is in this condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

