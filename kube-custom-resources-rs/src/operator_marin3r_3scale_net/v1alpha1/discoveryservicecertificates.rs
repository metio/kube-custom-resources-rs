// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/3scale-ops/marin3r/operator.marin3r.3scale.net/v1alpha1/discoveryservicecertificates.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// DiscoveryServiceCertificateSpec defines the desired state of DiscoveryServiceCertificate
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "operator.marin3r.3scale.net", version = "v1alpha1", kind = "DiscoveryServiceCertificate", plural = "discoveryservicecertificates")]
#[kube(namespaced)]
#[kube(status = "DiscoveryServiceCertificateStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DiscoveryServiceCertificateSpec {
    /// CertificateRenewalConfig configures the certificate renewal process. If unset default behavior is to renew the certificate but not notify of renewals.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certificateRenewal")]
    pub certificate_renewal: Option<DiscoveryServiceCertificateCertificateRenewal>,
    /// CommonName is the CommonName of the certificate
    #[serde(rename = "commonName")]
    pub common_name: String,
    /// Hosts is the list of hosts the certificate is valid for. Only use when 'IsServerCertificate' is true. If unset, the CommonName field will be used to populate the valid hosts of the certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
    /// IsCA is a boolean specifying that the certificate is a CA
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isCA")]
    pub is_ca: Option<bool>,
    /// SecretRef is a reference to the secret that will hold the certificate and the private key.
    #[serde(rename = "secretRef")]
    pub secret_ref: DiscoveryServiceCertificateSecretRef,
    /// IsServerCertificate is a boolean specifying if the certificate should be issued with server auth usage enabled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<bool>,
    /// Signer specifies  the signer to use to create this certificate. Supported signers are CertManager and SelfSigned.
    pub signer: DiscoveryServiceCertificateSigner,
    /// ValidFor specifies the validity of the certificate in seconds
    #[serde(rename = "validFor")]
    pub valid_for: i64,
}

/// CertificateRenewalConfig configures the certificate renewal process. If unset default behavior is to renew the certificate but not notify of renewals.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DiscoveryServiceCertificateCertificateRenewal {
    /// Enabled is a flag to enable or disable renewal of the certificate
    pub enabled: bool,
}

/// SecretRef is a reference to the secret that will hold the certificate and the private key.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DiscoveryServiceCertificateSecretRef {
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Signer specifies  the signer to use to create this certificate. Supported signers are CertManager and SelfSigned.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DiscoveryServiceCertificateSigner {
    /// CASigned holds specific configuration for the CASigned signer
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caSigned")]
    pub ca_signed: Option<DiscoveryServiceCertificateSignerCaSigned>,
    /// SelfSigned holds specific configuration for the SelfSigned signer
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "selfSigned")]
    pub self_signed: Option<DiscoveryServiceCertificateSignerSelfSigned>,
}

/// CASigned holds specific configuration for the CASigned signer
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DiscoveryServiceCertificateSignerCaSigned {
    /// A reference to a Secret containing the CA
    #[serde(rename = "caSecretRef")]
    pub ca_secret_ref: DiscoveryServiceCertificateSignerCaSignedCaSecretRef,
}

/// A reference to a Secret containing the CA
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DiscoveryServiceCertificateSignerCaSignedCaSecretRef {
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// SelfSigned holds specific configuration for the SelfSigned signer
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DiscoveryServiceCertificateSignerSelfSigned {
}

/// DiscoveryServiceCertificateStatus defines the observed state of DiscoveryServiceCertificate
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DiscoveryServiceCertificateStatus {
    /// CertificateHash stores the current hash of the certificate. It is used for other controllers to validate if a certificate has been re-issued.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certificateHash")]
    pub certificate_hash: Option<String>,
    /// Conditions represent the latest available observations of an object's state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// NotAfter is the time at which the certificate expires
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notAfter")]
    pub not_after: Option<String>,
    /// NotBefore is the time at which the certificate starts being valid
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notBefore")]
    pub not_before: Option<String>,
    /// Ready is a boolean that specifies if the certificate is ready to be used
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
}

