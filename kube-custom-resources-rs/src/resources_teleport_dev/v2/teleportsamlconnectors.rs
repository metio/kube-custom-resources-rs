// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/gravitational/teleport/resources.teleport.dev/v2/teleportsamlconnectors.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// SAMLConnector resource definition v2 from Teleport
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "resources.teleport.dev", version = "v2", kind = "TeleportSAMLConnector", plural = "teleportsamlconnectors")]
#[kube(namespaced)]
#[kube(status = "TeleportSAMLConnectorStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct TeleportSAMLConnectorSpec {
    /// AssertionConsumerService is a URL for assertion consumer service on the service provider (Teleport's side).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acs: Option<String>,
    /// AllowIDPInitiated is a flag that indicates if the connector can be used for IdP-initiated logins.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_idp_initiated: Option<bool>,
    /// EncryptionKeyPair is a key pair used for decrypting SAML assertions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assertion_key_pair: Option<TeleportSAMLConnectorAssertionKeyPair>,
    /// AttributesToRoles is a list of mappings of attribute statements to roles.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes_to_roles: Option<Vec<TeleportSAMLConnectorAttributesToRoles>>,
    /// Audience uniquely identifies our service provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    /// Cert is the identity provider certificate PEM. IDP signs `<Response>` responses using this certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert: Option<String>,
    /// ClientRedirectSettings defines which client redirect URLs are allowed for non-browser SSO logins other than the standard localhost ones.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_redirect_settings: Option<TeleportSAMLConnectorClientRedirectSettings>,
    /// Display controls how this connector is displayed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    /// EntityDescriptor is XML with descriptor. It can be used to supply configuration parameters in one XML file rather than supplying them in the individual elements.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_descriptor: Option<String>,
    /// EntityDescriptorURL is a URL that supplies a configuration XML.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_descriptor_url: Option<String>,
    /// Issuer is the identity provider issuer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// Provider is the external identity provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// ServiceProviderIssuer is the issuer of the service provider (Teleport).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_provider_issuer: Option<String>,
    /// SigningKeyPair is an x509 key pair used to sign AuthnRequest.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signing_key_pair: Option<TeleportSAMLConnectorSigningKeyPair>,
    /// SingleLogoutURL is the SAML Single log-out URL to initiate SAML SLO (single log-out). If this is not provided, SLO is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub single_logout_url: Option<String>,
    /// SSO is the URL of the identity provider's SSO service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sso: Option<String>,
}

/// EncryptionKeyPair is a key pair used for decrypting SAML assertions.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TeleportSAMLConnectorAssertionKeyPair {
    /// Cert is a PEM-encoded x509 certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert: Option<String>,
    /// PrivateKey is a PEM encoded x509 private key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TeleportSAMLConnectorAttributesToRoles {
    /// Name is an attribute statement name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Roles is a list of static teleport roles to map to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    /// Value is an attribute statement value to match.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// ClientRedirectSettings defines which client redirect URLs are allowed for non-browser SSO logins other than the standard localhost ones.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TeleportSAMLConnectorClientRedirectSettings {
    /// a list of hostnames allowed for https client redirect URLs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_https_hostnames: Option<Vec<String>>,
    /// a list of CIDRs allowed for HTTP or HTTPS client redirect URLs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure_allowed_cidr_ranges: Option<Vec<String>>,
}

/// SigningKeyPair is an x509 key pair used to sign AuthnRequest.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TeleportSAMLConnectorSigningKeyPair {
    /// Cert is a PEM-encoded x509 certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert: Option<String>,
    /// PrivateKey is a PEM encoded x509 private key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
}

/// Status defines the observed state of the Teleport resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TeleportSAMLConnectorStatus {
    /// Conditions represent the latest available observations of an object's state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "teleportResourceID")]
    pub teleport_resource_id: Option<i64>,
}

