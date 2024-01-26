// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/hashicorp/vault-secrets-operator/secrets.hashicorp.com/v1beta1/vaultpkisecrets.yaml --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// VaultPKISecretSpec defines the desired state of VaultPKISecret
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "secrets.hashicorp.com", version = "v1beta1", kind = "VaultPKISecret", plural = "vaultpkisecrets")]
#[kube(namespaced)]
#[kube(status = "VaultPKISecretStatus")]
#[kube(schema = "disabled")]
pub struct VaultPKISecretSpec {
    /// AltNames to include in the request May contain both DNS names and email addresses.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "altNames")]
    pub alt_names: Option<Vec<String>>,
    /// Clear the Kubernetes secret when the resource is deleted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clear: Option<bool>,
    /// CommonName to include in the request.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "commonName")]
    pub common_name: Option<String>,
    /// Destination provides configuration necessary for syncing the Vault secret to Kubernetes. If the type is set to "kubernetes.io/tls", "tls.key" will be set to the "private_key" response from Vault, and "tls.crt" will be set to "certificate" + "ca_chain" from the Vault response ("issuing_ca" is used when "ca_chain" is empty). The "remove_roots_from_chain=true" option is used with Vault to exclude the root CA from the Vault response.
    pub destination: VaultPKISecretDestination,
    /// ExcludeCNFromSans from DNS or Email Subject Alternate Names. Default: false
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "excludeCNFromSans")]
    pub exclude_cn_from_sans: Option<bool>,
    /// ExpiryOffset to use for computing when the certificate should be renewed. The rotation time will be difference between the expiration and the offset. Should be in duration notation e.g. 30s, 120s, etc.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "expiryOffset")]
    pub expiry_offset: Option<String>,
    /// Format for the certificate. Choices: "pem", "der", "pem_bundle". If "pem_bundle", any private key and issuing cert will be appended to the certificate pem. If "der", the value will be base64 encoded. Default: pem
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// IPSans to include in the request.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipSans")]
    pub ip_sans: Option<Vec<String>>,
    /// IssuerRef reference to an existing PKI issuer, either by Vault-generated identifier, the literal string default to refer to the currently configured default issuer, or the name assigned to an issuer. This parameter is part of the request URL.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "issuerRef")]
    pub issuer_ref: Option<String>,
    /// Mount for the secret in Vault
    pub mount: String,
    /// Namespace to get the secret from in Vault
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// NotAfter field of the certificate with specified date value. The value format should be given in UTC format YYYY-MM-ddTHH:MM:SSZ
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notAfter")]
    pub not_after: Option<String>,
    /// Requested other SANs, in an array with the format oid;type:value for each entry.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "otherSans")]
    pub other_sans: Option<Vec<String>>,
    /// PrivateKeyFormat, generally the default will be controlled by the Format parameter as either base64-encoded DER or PEM-encoded DER. However, this can be set to "pkcs8" to have the returned private key contain base64-encoded pkcs8 or PEM-encoded pkcs8 instead. Default: der
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "privateKeyFormat")]
    pub private_key_format: Option<String>,
    /// Revoke the certificate when the resource is deleted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoke: Option<bool>,
    /// Role in Vault to use when issuing TLS certificates.
    pub role: String,
    /// RolloutRestartTargets should be configured whenever the application(s) consuming the Vault secret does not support dynamically reloading a rotated secret. In that case one, or more RolloutRestartTarget(s) can be configured here. The Operator will trigger a "rollout-restart" for each target whenever the Vault secret changes between reconciliation events. See RolloutRestartTarget for more details.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rolloutRestartTargets")]
    pub rollout_restart_targets: Option<Vec<VaultPKISecretRolloutRestartTargets>>,
    /// TTL for the certificate; sets the expiration date. If not specified the Vault role's default, backend default, or system default TTL is used, in that order. Cannot be larger than the mount's max TTL. Note: this only has an effect when generating a CA cert or signing a CA cert, not when generating a CSR for an intermediate CA. Should be in duration notation e.g. 120s, 2h, etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
    /// The requested URI SANs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "uriSans")]
    pub uri_sans: Option<Vec<String>>,
    /// VaultAuthRef to the VaultAuth resource, can be prefixed with a namespace, eg: `namespaceA/vaultAuthRefB`. If no namespace prefix is provided it will default to namespace of the VaultAuth CR. If no value is specified for VaultAuthRef the Operator will default to the `default` VaultAuth, configured in the operator's namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vaultAuthRef")]
    pub vault_auth_ref: Option<String>,
}

/// Destination provides configuration necessary for syncing the Vault secret to Kubernetes. If the type is set to "kubernetes.io/tls", "tls.key" will be set to the "private_key" response from Vault, and "tls.crt" will be set to "certificate" + "ca_chain" from the Vault response ("issuing_ca" is used when "ca_chain" is empty). The "remove_roots_from_chain=true" option is used with Vault to exclude the root CA from the Vault response.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VaultPKISecretDestination {
    /// Annotations to apply to the Secret. Requires Create to be set to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Create the destination Secret. If the Secret already exists this should be set to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create: Option<bool>,
    /// Labels to apply to the Secret. Requires Create to be set to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// Name of the Secret
    pub name: String,
    /// Type of Kubernetes Secret. Requires Create to be set to true. Defaults to Opaque.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// RolloutRestartTarget provides the configuration required to perform a rollout-restart of the supported resources upon Vault Secret rotation. The rollout-restart is triggered by patching the target resource's 'spec.template.metadata.annotations' to include 'vso.secrets.hashicorp.com/restartedAt' with a timestamp value of when the trigger was executed. E.g. vso.secrets.hashicorp.com/restartedAt: "2023-03-23T13:39:31Z" 
///  Supported resources: Deployment, DaemonSet, StatefulSet
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VaultPKISecretRolloutRestartTargets {
    pub kind: VaultPKISecretRolloutRestartTargetsKind,
    pub name: String,
}

/// RolloutRestartTarget provides the configuration required to perform a rollout-restart of the supported resources upon Vault Secret rotation. The rollout-restart is triggered by patching the target resource's 'spec.template.metadata.annotations' to include 'vso.secrets.hashicorp.com/restartedAt' with a timestamp value of when the trigger was executed. E.g. vso.secrets.hashicorp.com/restartedAt: "2023-03-23T13:39:31Z" 
///  Supported resources: Deployment, DaemonSet, StatefulSet
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VaultPKISecretRolloutRestartTargetsKind {
    Deployment,
    DaemonSet,
    StatefulSet,
}

/// VaultPKISecretStatus defines the observed state of VaultPKISecret
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VaultPKISecretStatus {
    pub error: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration: Option<i64>,
    /// LastGeneration is the Generation of the last reconciled resource.
    #[serde(rename = "lastGeneration")]
    pub last_generation: i64,
    /// LastLastRotation of the certificate.
    #[serde(rename = "lastRotation")]
    pub last_rotation: i64,
    /// SecretMAC used when deciding whether new Vault secret data should be synced. 
    ///  The controller will compare the "new" Vault secret data to this value using HMAC, if they are different, then the data will be synced to the Destination. 
    ///  The SecretMac is also used to detect drift in the Destination Secret's Data. If drift is detected the data will be synced to the Destination.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretMAC")]
    pub secret_mac: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serialNumber")]
    pub serial_number: Option<String>,
    pub valid: bool,
}

