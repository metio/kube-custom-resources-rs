// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/gravitational/teleport/resources.teleport.dev/v2/teleportsamlconnectors.yaml --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// SAMLConnector resource definition v2 from Teleport
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "resources.teleport.dev", version = "v2", kind = "TeleportSAMLConnector", plural = "teleportsamlconnectors")]
#[kube(namespaced)]
#[kube(status = "TeleportSAMLConnectorStatus")]
#[kube(schema = "disabled")]
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
    /// Cert is the identity provider certificate PEM. IDP signs <Response> responses using this certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert: Option<String>,
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
    /// SSO is the URL of the identity provider's SSO service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sso: Option<String>,
}

/// EncryptionKeyPair is a key pair used for decrypting SAML assertions.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TeleportSAMLConnectorAssertionKeyPair {
    /// Cert is a PEM-encoded x509 certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert: Option<String>,
    /// PrivateKey is a PEM encoded x509 private key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

/// SigningKeyPair is an x509 key pair used to sign AuthnRequest.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TeleportSAMLConnectorSigningKeyPair {
    /// Cert is a PEM-encoded x509 certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert: Option<String>,
    /// PrivateKey is a PEM encoded x509 private key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
}

/// TeleportSAMLConnectorStatus defines the observed state of TeleportSAMLConnector
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TeleportSAMLConnectorStatus {
    /// Conditions represent the latest available observations of an object's state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<TeleportSAMLConnectorStatusConditions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "teleportResourceID")]
    pub teleport_resource_id: Option<i64>,
}

/// Condition contains details for one aspect of the current state of this API Resource.
/// ---
/// This struct is intended for direct use as an array at the field path .status.conditions.  For example,
/// 
/// 
/// 	type FooStatus struct{
/// 	    // Represents the observations of a foo's current state.
/// 	    // Known .status.conditions.type are: "Available", "Progressing", and "Degraded"
/// 	    // +patchMergeKey=type
/// 	    // +patchStrategy=merge
/// 	    // +listType=map
/// 	    // +listMapKey=type
/// 	    Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"`
/// 
/// 
/// 	    // other fields
/// 	}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TeleportSAMLConnectorStatusConditions {
    /// lastTransitionTime is the last time the condition transitioned from one status to another.
    /// This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// message is a human readable message indicating details about the transition.
    /// This may be an empty string.
    pub message: String,
    /// observedGeneration represents the .metadata.generation that the condition was set based upon.
    /// For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date
    /// with respect to the current state of the instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// reason contains a programmatic identifier indicating the reason for the condition's last transition.
    /// Producers of specific condition types may define expected values and meanings for this field,
    /// and whether the values are considered a guaranteed API.
    /// The value should be a CamelCase string.
    /// This field may not be empty.
    pub reason: String,
    /// status of the condition, one of True, False, Unknown.
    pub status: TeleportSAMLConnectorStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase.
    /// ---
    /// Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be
    /// useful (see .node.status.conditions), the ability to deconflict is important.
    /// The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource.
/// ---
/// This struct is intended for direct use as an array at the field path .status.conditions.  For example,
/// 
/// 
/// 	type FooStatus struct{
/// 	    // Represents the observations of a foo's current state.
/// 	    // Known .status.conditions.type are: "Available", "Progressing", and "Degraded"
/// 	    // +patchMergeKey=type
/// 	    // +patchStrategy=merge
/// 	    // +listType=map
/// 	    // +listMapKey=type
/// 	    Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"`
/// 
/// 
/// 	    // other fields
/// 	}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TeleportSAMLConnectorStatusConditionsStatus {
    True,
    False,
    Unknown,
}

